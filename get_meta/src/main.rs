#![feature(rustc_private)]

extern crate rustc_metadata;

extern crate rustc_back;
extern crate serialize as rustc_serialize;


use rustc_back::target::{Target, TargetResult, TargetOptions};
use rustc_metadata::locator::{get_metadata_section, CrateFlavor};

use std::path::Path;
use std::default::Default;
use std::env;
use rustc_serialize::json::{self};

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

#[derive(RustcEncodable, RustcDecodable)]
pub struct KrateRut
{
   dependencies : Vec<String>
}

impl KrateRut
{
    pub fn new(strns : Vec<String>) -> KrateRut
    {
        KrateRut
        {   
            dependencies : strns
        }
    }   
}

fn d0(path : String)
{
    let crate_path = Path::new(&path);

    let flavor = CrateFlavor::Dylib;
    let target = target().unwrap();
    let metadata_blob = get_metadata_section(&target, flavor, crate_path).unwrap();

    let crate_root = metadata_blob.get_root();
    let mut strins = Vec::new();
   
    for (_, dep) in crate_root.crate_deps.decode(&metadata_blob).enumerate() {
        strins.push(format!("{}",dep.name));
    }
    
    let krate_rut = KrateRut::new(strins);
    let encoded = json::encode(&krate_rut).unwrap();

    println!("{}",encoded);
}

fn main() {
    match env::args().nth(1)
    {
        Some(arg) => d0(arg),
        _         => panic!("no") 
    }
}
