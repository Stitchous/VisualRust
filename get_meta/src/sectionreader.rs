#![feature(rustc_private)]

use rustc_back::target::Target;

use rustc_metadata::cstore::MetadataBlob;

use rustc::util::common;

use rustc_llvm as llvm;
use rustc_llvm::{False, ObjectFile, mk_section_iter};
use rustc_llvm::archive_ro::ArchiveRO;


use std::cmp;
use std::fmt;
use std::fs;
use std::io;
use std::path::{Path, PathBuf};
use std::ptr;
use std::slice;
use std::time::Instant;

use flate;



/// Metadata encoding version.
/// NB: increment this if you change the format of metadata such that
/// the rustc version can't be found to compare with `RUSTC_VERSION`.
pub const METADATA_VERSION: u8 = 3;

/// Metadata header which includes `METADATA_VERSION`.
/// To get older versions of rustc to ignore this metadata,
/// there are 4 zero bytes at the start, which are treated
/// as a length of 0 by old compilers.
///
/// This header is followed by the position of the `CrateRoot`.
pub const METADATA_HEADER: &'static [u8; 12] = &[
    0, 0, 0, 0,
    b'r', b'u', b's', b't',
    0, 0, 0, METADATA_VERSION
];

pub const METADATA_FILENAME: &'static str = "rust.metadata.bin";

#[derive(Copy, Clone, PartialEq)]
pub enum CrateFlavor {
	    Rlib,
	    Dylib,
}

pub struct ArchiveMetadata {
    _archive: ArchiveRO,
    // points into self._archive
    data: *const [u8],
}

impl ArchiveMetadata {
    fn new(ar: ArchiveRO) -> Option<ArchiveMetadata> {
        let data = {
            let section = ar.iter().filter_map(|s| s.ok()).find(|sect| {
                sect.name() == Some(METADATA_FILENAME)
            });
            match section {
                Some(s) => s.data() as *const [u8],
                None => {
                    debug!("didn't find '{}' in the archive", METADATA_FILENAME);
                    return None;
                }
            }
        };

        Some(ArchiveMetadata {
            _archive: ar,
            data: data,
        })
    }

    pub fn as_slice<'a>(&'a self) -> &'a [u8] { unsafe { &*self.data } }
}


pub fn get_metadata_section(target: &Target, flavor: CrateFlavor, filename: &Path)
                        -> Result<MetadataBlob, String> {
    let start = Instant::now();
    let ret = get_metadata_section_imp(target, flavor, filename);
    info!("reading {:?} => {:?}", filename.file_name().unwrap(),
          start.elapsed());
    return ret
}

fn verify_decompressed_encoding_version(blob: &MetadataBlob,
                                        filename: &Path)
                                        -> Result<(), String> {
    if !blob.is_compatible() {
        Err((format!("incompatible metadata version found: '{}'",
                     filename.display())))
    } else {
        Ok(())
    }
}

fn get_metadata_section_imp(target: &Target, flavor: CrateFlavor, filename: &Path)
                            -> Result<MetadataBlob, String> {
    if !filename.exists() {
        return Err(format!("no such file: '{}'", filename.display()));
    }
    /*if flavor == CrateFlavor::Rlib {
        // Use ArchiveRO for speed here, it's backed by LLVM and uses mmap
        // internally to read the file. We also avoid even using a memcpy by
        // just keeping the archive along while the metadata is in use.
        let archive = match ArchiveRO::open(filename) {
            Some(ar) => ar,
            None => {
                debug!("llvm didn't like `{}`", filename.display());
                return Err(format!("failed to read rlib metadata: '{}'",
                                   filename.display()));
            }
        };
        return match ArchiveMetadata::new(archive).map(|ar| MetadataBlob::Archive(ar)) {
            None => Err(format!("failed to read rlib metadata: '{}'",
                                filename.display())),
            Some(blob) => {
                verify_decompressed_encoding_version(&blob, filename)?;
                Ok(blob)
            }
        };
    }*/
    unsafe {
        let buf = common::path2cstr(filename);
        let mb = llvm::LLVMRustCreateMemoryBufferWithContentsOfFile(buf.as_ptr());
        if mb as isize == 0 {
            return Err(format!("error reading library: '{}'",
                               filename.display()))
        }
        let of = match ObjectFile::new(mb) {
            Some(of) => of,
            _ => {
                return Err((format!("provided path not an object file: '{}'",
                                    filename.display())))
            }
        };
        let si = mk_section_iter(of.llof);
        while llvm::LLVMIsSectionIteratorAtEnd(of.llof, si.llsi) == False {
            let mut name_buf = ptr::null();
            let name_len = llvm::LLVMRustGetSectionName(si.llsi, &mut name_buf);
            let name = slice::from_raw_parts(name_buf as *const u8,
                                             name_len as usize).to_vec();
            let name = String::from_utf8(name).unwrap();
            debug!("get_metadata_section: name {}", name);
            if read_meta_section_name(target) == name {
                let cbuf = llvm::LLVMGetSectionContents(si.llsi);
                let csz = llvm::LLVMGetSectionSize(si.llsi) as usize;
                let cvbuf: *const u8 = cbuf as *const u8;
                let vlen = METADATA_HEADER.len();
                debug!("checking {} bytes of metadata-version stamp",
                       vlen);
                let minsz = cmp::min(vlen, csz);
                let buf0 = slice::from_raw_parts(cvbuf, minsz);
                let version_ok = buf0 == METADATA_HEADER;
                if !version_ok {
                    return Err((format!("incompatible metadata version found: '{}'",
                                        filename.display())));
                }

                let cvbuf1 = cvbuf.offset(vlen as isize);
                debug!("inflating {} bytes of compressed metadata",
                       csz - vlen);
                let bytes = slice::from_raw_parts(cvbuf1, csz - vlen);
                match flate::inflate_bytes(bytes) {
                    Ok(inflated) => {
                        let blob = MetadataBlob::Inflated(inflated);
                        verify_decompressed_encoding_version(&blob, filename)?;
                        return Ok(blob);
                    }
                    Err(_) => {}
                }
            }
            llvm::LLVMMoveToNextSection(si.llsi);
        }
        Err(format!("metadata not found: '{}'", filename.display()))
    }
}

pub fn read_meta_section_name(_target: &Target) -> &'static str {
    ".rustc"
}
