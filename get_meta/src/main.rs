#![feature(rustc_private)]


extern crate rustc;
extern crate rustc_back;
extern crate rustc_metadata;
extern crate rustc_llvm;
extern crate flate;
extern crate syntax_pos;
extern crate serialize as rustc_serialize;

#[macro_use] extern crate log;


mod sectionreader;

use rustc_serialize::{Encodable, Encoder, Decodable};


use rustc_back::target::{Target, TargetResult, TargetOptions};

use rustc::middle::lang_items;


//use rustc_metadata::locator::get_metadata_section;

use sectionreader::{get_metadata_section, CrateFlavor};

use rustc::hir;
use rustc::hir::def::{self, CtorKind};
use rustc::hir::def_id::{DefIndex, DefId};
use rustc::middle::cstore::{LinkagePreference, NativeLibraryKind};
use rustc_metadata::cstore::MetadataBlob;
use rustc_metadata::schema::LazySeq;
use rustc_metadata::schema::CrateDep;
use rustc_metadata::schema::MacroDef;
use rustc_metadata::schema::TraitImpls;
use rustc_metadata::index;

use rustc_back::PanicStrategy;

use std::path::Path;
use std::default::Default;
use std::env;
use rustc_serialize::json::{self};




use rustc_serialize::json::ToJson;

pub fn opts() -> TargetOptions {
    TargetOptions {
        function_sections: true,
        linker: "link.exe".to_string(),
        ar: "llvm-ar.exe".to_string(),
        dynamic_linking: true,
        executables: true,
        dll_prefix: "".to_string(),
        dll_suffix: ".dll".to_string(),
        exe_suffix: ".exe".to_string(),
        staticlib_prefix: "".to_string(),
        staticlib_suffix: ".lib".to_string(),
        is_like_windows: true,
        is_like_msvc: true,
        pre_link_args: vec![
            "/NOLOGO".to_string(),
            "/NXCOMPAT".to_string(),
        ],
        exe_allocation_crate: "alloc_system".to_string(),

        .. Default::default()
    }
}


fn target() -> TargetResult {
    let mut base = opts();
    base.cpu = "x86-64".to_string();
    base.max_atomic_width = Some(64);
    Ok(Target {
        llvm_target: "x86_64-pc-windows-msvc".to_string(),
        target_endian: "little".to_string(),
        target_pointer_width: "64".to_string(),
        data_layout: "e-m:w-i64:64-f80:128-n8:16:32:64-S128".to_string(),
        arch: "x86_64".to_string(),
        target_os: "windows".to_string(),
        target_env: "msvc".to_string(),
        target_vendor: "pc".to_string(),
        options: base,
    })
}


//#[derive(RustcEncodable, RustcDecodable)]


#[derive(RustcEncodable)]
pub struct KrateRut
{
    pub rustc_version: String,
    pub name: String,
    pub triple: String,
    pub hash: u64,
    pub disambiguator: String,
    pub panic_strategy: PanicStrategy,

    pub plugin_registrar_fn: Option<DefIndex>,
    pub macro_derive_registrar: Option<DefIndex>,
    
    dependencies : Vec<CrateDep>,
    dylib_dependency_formats : Vec<Option<LinkagePreference>>,
    lang_items : Vec<(DefIndex, usize)>,
    lang_items_missing: Vec<lang_items::LangItem>,

    native_libraries: Vec<(NativeLibraryKind, String)>,
    codemap: Vec<syntax_pos::FileMap>,
    macro_defs: Vec<MacroDef>,
    // impls: Vec<TraitImpls>,
    reachable_ids: Vec<DefIndex>,
    // index: Vec<index::Index>

    /*
    
    pub plugin_registrar_fn: Option<DefIndex>,
    pub macro_derive_registrar: Option<DefIndex>,*/

   
}

fn deserialize<T : Decodable>(inp : LazySeq<T>, meta : &MetadataBlob) ->  Vec<T>
{
    let mut res = Vec::new();
    for (_, r) in inp.decode(meta).enumerate() {
           res.push(r);
       };
    res
}

impl KrateRut
{
    pub fn new(metadata_blob : MetadataBlob) -> KrateRut
    {
        let mut strins = Vec::new();
   
        let crate_root = metadata_blob.get_root();

        

        for (_, dep) in crate_root.crate_deps.decode(&metadata_blob).enumerate() {
            strins.push(format!("{}",dep.name));
        };

        KrateRut
        {   
          
            name : crate_root.name,
            rustc_version : crate_root.rustc_version,
            triple : crate_root.triple,
            hash : crate_root.hash.as_u64(),
            disambiguator : crate_root.disambiguator,
            panic_strategy: crate_root.panic_strategy,

            plugin_registrar_fn : crate_root.plugin_registrar_fn,
            macro_derive_registrar : crate_root.macro_derive_registrar,

            dependencies : deserialize(crate_root.crate_deps, &metadata_blob),
            dylib_dependency_formats : deserialize(crate_root.dylib_dependency_formats, &metadata_blob),
            lang_items : deserialize(crate_root.lang_items, &metadata_blob),
            lang_items_missing : deserialize(crate_root.lang_items_missing, &metadata_blob),
            native_libraries: deserialize(crate_root.native_libraries, &metadata_blob),
            codemap: deserialize(crate_root.codemap, &metadata_blob),
            macro_defs: deserialize(crate_root.macro_defs, &metadata_blob),
            //impls: deserialize(crate_root.impls, &metadata_blob),
            reachable_ids: deserialize(crate_root.reachable_ids, &metadata_blob),
           // index: deserialize(crate_root.index, &metadata_blob),
        
    
        }
    }   
}

//impl Encodable for hir::svh::Svh {
//    fn encode<S: Encoder>(&self, s: &mut S) -> Result<(), S::Error> {
//       s.emit_u64(self.as_u64().to_le())
//    }
//}


fn d0(path : String)
{
    let crate_path = Path::new(&path);

    let flavor = CrateFlavor::Dylib;
    let target = target().unwrap();
    let metadata_blob = get_metadata_section(&target, flavor, crate_path).unwrap();
   
    
    let krate_rut = KrateRut::new(metadata_blob);
    let encoded = json::encode(&krate_rut).unwrap();

    //let crate_root = metadata_blob.get_root();

    //let encoded2 = json::encode(&crate_root).unwrap();

    println!("{}",encoded);
}

fn main() {


    let strategey = rustc_back::PanicStrategy::Unwind;

    strategey.to_json();

    match env::args().nth(1)
    {
        Some(arg) => d0(arg),
        _         => panic!("no") 
    }
}
