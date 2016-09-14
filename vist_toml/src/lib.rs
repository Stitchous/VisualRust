extern crate toml_document;
extern crate winapi;
extern crate kernel32;

use std::fmt::{Display, Error, Formatter};
use std::iter;
use std::mem;
use std::ptr;
use std::slice;
use std::str;
use std::marker::PhantomData;

use toml_document::{ArrayEntry, ArrayValueMut, Container, ContainerKind, Document};
use toml_document::{EntryRef, EntryRefMut, InlineArray, InternalNode, InlineTable, TableEntry};
use toml_document::{TableValue, ValueRefMut};
use winapi::INT32;

mod panic;
pub mod capi;

macro_rules! set_output_target {
    ($container: ident, $target: ident, $set_string: ident, $set_bool: ident) => (
        $set_string($container, "name", $target.name);
        $set_string($container, "path", $target.path);
        $set_bool($container, "test", $target.test);
        $set_bool($container, "doctest", $target.doctest);
        $set_bool($container, "bench", $target.bench);
        $set_bool($container, "doc", $target.doc);
        $set_bool($container, "plugin", $target.plugin);
        $set_bool($container, "harness", $target.harness);
    )
}

fn entry_kind(e: EntryRef) -> &'static str {
    match e {
        EntryRef::String(..) => "string",
        EntryRef::Integer(..) => "integer",
        EntryRef::Float(..) => "float",
        EntryRef::Boolean(..) => "boolean",
        EntryRef::Datetime(..) => "datetime",
        EntryRef::Array(..) => "array",
        EntryRef::Table(..) => "table",
    }
}

fn array_kind(e: ArrayEntry) -> Option<&'static str> {
    if e.len() == 0 {
        None
    } else {
        match e.get(0) {
            EntryRef::String(..) => Some("array of strings"),
            EntryRef::Integer(..) => Some("array of integers"),
            EntryRef::Float(..) => Some("array of floats"),
            EntryRef::Boolean(..) => Some("array of booleans"),
            EntryRef::Datetime(..) => Some("array of datetimes"),
            EntryRef::Array(..) => Some("array of arrays"),
            EntryRef::Table(..) => Some("array of tables"),
        }
    }
}

pub struct Manifest {
    doc: Document
}

impl Display for Manifest {
    fn fmt(&self, f: &mut Formatter) -> Result<(), Error> {
        self.doc.fmt(f)
    }
}

// Set functions:
// * if no table exists, create new top-level one
// * overwrites value, even if it has a value but wrong type 
impl Manifest {
    pub fn new(doc: Document) -> Manifest {
        Manifest { doc: doc }
    }

    pub fn get_string<'a, 'b:'a>(&'a self, path: &'b [&'b str]) -> Result<&'a str, QueryError> {
        match Manifest::lookup(&self.doc, path) {
            Ok(EntryRef::String(value)) => Ok(value.get()),
            Ok(entry) => Err(QueryError::Conflict { depth: path.len(), kind: entry_kind(entry) }),
            Err(err) => Err(err)
        }
    }

    // It's the caller responsibility to make sure we are not
    // setting value on a conflicting path, eg. for
    //   [[a]]
    //   b = "c"
    // `set_string(&["a", "b"], "c")` will simply panic
    pub fn set_string<'a>(&'a mut self, _: &'a [&'a str], _: &'a str) -> bool {
        unimplemented!()
    }

    pub fn get_string_array<'a>(&'a self,
                                path: &'a [&'a str])
                                -> Result<Vec<&'a str>, QueryError> {
        fn string_value<'a>(entry: EntryRef<'a>) -> &'a str {
            match entry {
                EntryRef::String(value) => value.get(),
                _ => unreachable!()
            }
        }
        match Manifest::lookup(&self.doc, path) {
            Ok(EntryRef::Array(array)) => {
                if array.len() == 0 {
                    return Ok(Vec::new());
                }
                match array.get(0) {
                    EntryRef::String(_) => Ok(array.iter().map(string_value).collect()),
                    entry => Err(QueryError::Conflict { depth: path.len(), kind: entry_kind(entry) })
                }
            }
            Ok(entry) => Err(QueryError::Conflict { depth: path.len(), kind: entry_kind(entry) }),
            Err(err) => Err(err)
        }
    }

    pub fn get_dependencies(&self) -> Result<Vec<Dependency>, Vec<PathError>> {
        fn get_inner<'a>(deps: &mut Vec<Dependency<'a>>,
                         errors: &mut Vec<PathError>,
                         target: Option<&'a str>,
                         entry: EntryRef<'a>) {
            match entry {
                EntryRef::Table(table) => {
                    for (name, entry) in table.iter() {
                        match entry {
                            EntryRef::String(version) => {
                                deps.push(Dependency::simple(name, target, version.get()));
                            }
                            EntryRef::Table(table) => {
                                deps.push(Dependency::complex(name, target, table));
                            }
                            entry => {
                                let path = match target {
                                    Some(target) => {
                                        format!("target.{}.dependencies.{}", target, name)
                                    }
                                    None => format!("dependencies.{}", name)
                                };
                                let error = PathError {
                                    path: path,
                                    expected: "string",
                                    got: entry_kind(entry)
                                };
                                errors.push(error);
                            }
                        }
                    }
                }
                entry => {
                    let path = match target {
                        Some(target) => {
                            format!("target.{}.dependencies", target)
                        }
                        None => "dependencies".to_owned()
                    };
                    let error = PathError {
                        path: path,
                        expected: "table",
                        got: entry_kind(entry)
                    };
                    errors.push(error);
                }
            }
        }
        let mut deps = Vec::new();
        let mut errors = Vec::new();
        if let Some(entry) = self.doc.get("dependencies") {
            get_inner(&mut deps, &mut errors, None, entry);
        }
        if let Some(EntryRef::Table(targets)) = self.doc.get("target") {
            for (target, target_entry) in targets.iter() {
                if let EntryRef::Table(target_table) = target_entry {
                    if let Some(entry) = target_table.get("dependencies") {
                        get_inner(&mut deps, &mut errors, Some(target), entry);
                    }
                }
            }
        }
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(deps)
        }
    }

    pub fn get_output_targets(&self) -> Result<Vec<OutputTarget>, Vec<PathError>> {
        fn get_string<'a>(entry: Option<EntryRef<'a>>,
                          path: String)
                          -> Result<Option<&'a str>, PathError> {
            match entry {
                Some(EntryRef::String(s)) => Ok(Some(s.get())),
                Some(entry) => {
                    let error = PathError {
                        path: path,
                        expected: "string",
                        got: entry_kind(entry)
                    };
                    Err(error)
                }
                None => Ok(None)
            }
        }
        fn get_bool<'a>(entry: Option<EntryRef<'a>>,
                        path: String)
                        -> Result<Option<bool>, PathError> {
            match entry {
                Some(EntryRef::Boolean(b)) => Ok(Some(b.get())),
                Some(entry) => {
                    let error = PathError {
                        path: path,
                        expected: "boolean",
                        got: entry_kind(entry)
                    };
                    Err(error)
                }
                None => Ok(None)
            }
        }
        fn get_target<'a>(src: &'a str,
                         entry: TableEntry<'a>,
                         mut target: OutputTarget<'a>)
                         -> Result<OutputTarget<'a>, PathError> {
            target.name = try!(get_string(entry.get("name"), format!("{}.name", src)));
            target.path = try!(get_string(entry.get("path"), format!("{}.path", src)));
            target.test = try!(get_bool(entry.get("test"), format!("{}.test", src)));
            target.doctest = try!(get_bool(entry.get("doctest"), format!("{}.doctest", src)));
            target.bench = try!(get_bool(entry.get("bench"), format!("{}.bench", src)));
            target.doc = try!(get_bool(entry.get("doc"), format!("{}.doc", src)));
            target.plugin = try!(get_bool(entry.get("plugin"), format!("{}.plugin", src)));
            target.harness= try!(get_bool(entry.get("harness"), format!("{}.harness", src)));
            Ok(target)
        }
        fn get_table<'a, F>(src: &'a str,
                            entry: Option<EntryRef<'a>>,
                            ctor: &F,
                            targets: &mut Vec<OutputTarget<'a>>,
                            errors: &mut Vec<PathError>)
                            where F: Fn(usize) -> OutputTarget<'a> {
            // This is a-ok, because only way we can spot an implicit table is in [lib]:
            // Some deranged mind might have a definition like [lib.fuck_your_parsing] in his
            // manifest, in which case it's pretty easy to just throw away all containers whose
            // keys start with `lib`.
            fn get_table_ptr(table: TableEntry) -> usize {
                match table.to_value() {
                    TableValue::Inline(inline_table) => inline_table.ptr(),
                    TableValue::Explicit(container) => container.ptr(),
                    TableValue::Implicit => 0
                }
            }
            match entry {
                Some(EntryRef::Table(table)) => {
                    let target = ctor(get_table_ptr(table));
                    match get_target(src, table, target) {
                        Ok(target) => targets.push(target),
                        Err(error) => errors.push(error)
                    }
                }
                Some(entry) => {
                    let error = PathError {
                        path: src.to_owned(),
                        expected: "table",
                        got: entry_kind(entry)
                    };
                    errors.push(error);
                }
                None => {}
            }
        }
        fn get_array<'a, F>(src: &'a str,
                            entry: Option<EntryRef<'a>>,
                            ctor: &F,
                            mut targets: &mut Vec<OutputTarget<'a>>,
                            mut errors: &mut Vec<PathError>)
                            where F: Fn(usize) -> OutputTarget<'a> {
            match entry {
                Some(EntryRef::Array(array)) => {
                    let kind = array_kind(array);
                    if kind != None && kind != Some("array of tables") {
                        let error = PathError {
                            path: src.to_owned(),
                            expected: "array of tables",
                            got: kind.unwrap()
                        };
                        errors.push(error);
                        return;
                    }
                    for entry in array.iter() {
                        get_table(src, Some(entry), ctor, &mut targets, &mut errors);
                    }
                }
                Some(entry) => {
                    let error = PathError {
                        path: src.to_owned(),
                        expected: "array",
                        got: entry_kind(entry)
                    };
                    errors.push(error);
                }
                None => { }
            }
        }
        let mut targets = Vec::new();
        let mut errors = Vec::new();
        get_table("lib", self.doc.get("lib"), &OutputTarget::lib, &mut targets, &mut errors);
        get_array("bin", self.doc.get("bin"), &OutputTarget::bin, &mut targets, &mut errors);
        get_array("bench", self.doc.get("bench"), &OutputTarget::bench, &mut targets, &mut errors);
        get_array("test", self.doc.get("test"), &OutputTarget::test, &mut targets, &mut errors);
        get_array("example",
                  self.doc.get("example"),
                  &OutputTarget::example,
                  &mut targets,
                  &mut errors);
        if errors.len() > 0 {
            Err(errors)
        } else {
            Ok(targets)
        }
    }

    fn lookup<'a>(doc: &'a Document,path: &'a [&'a str]) -> Result<EntryRef<'a>, QueryError> {
        fn lookup_inner<'a>(entry: EntryRef<'a>,
                            path: &'a [&'a str],
                            depth: usize)
                            -> Result<EntryRef<'a>, QueryError> {
            if path.len() == 0 {
                Ok(entry)
            } else {
                match entry {
                    EntryRef::Table(table) => {
                        table.get(path[0])
                             .map_or_else(|| Err(QueryError::Vacant{ depth: depth }),
                                          |e| lookup_inner(e, &path[1..], depth + 1))
                    }
                    _ => {
                        Err(QueryError::Conflict { depth: depth, kind: entry_kind(entry) })
                    }
                }
            }
        }
        doc.get(path[0])
           .map_or(Err(QueryError::Vacant{ depth: 0 }),
                   |entry| lookup_inner(entry, &path[1..], 0))
    }

    fn add_output_target(&mut self, target: OutputTarget) -> usize {
        let kind = if target.kind == "lib" {
            ContainerKind::Table
        } else {
            ContainerKind::ArrayMember
        };
        let index = self.doc.len();
        let mut container = self.doc.insert_container(index, iter::once(target.kind), kind);
        Manifest::add_output_target_inner(&mut container, target);
        container.ptr()
    }

    fn add_output_target_inner(container: &mut Container, target: OutputTarget) {
        fn append_bool(cnt: &mut Container, key: &'static str, value: Option<bool>) {
            if let Some(value) = value {
                let index = cnt.len_children();
                cnt.insert_boolean(index, key, value);
            }
        }
        fn append_string<'a>(cnt: &mut Container, key: &'static str, value: Option<&'a str>) {
            if let Some(value) = value {
                let index = cnt.len_children();
                cnt.insert_string(index, key, value);
            }
        }
        set_output_target!(container, target, append_string, append_bool);
    }

    fn set_output_target(&mut self, target: OutputTarget) -> Option<usize> {
        if target.handle == 0 {
            let length = self.doc.len();
            let container = self.doc.insert_container(length,
                                                      iter::once(target.kind),
                                                      ContainerKind::Table);
            Manifest::set_output_target_inner(container, target);
            Some(container.ptr())
        } else {
            let cursor = NodeCursor(target.handle);
            let maybe_index = self.doc.find(&cursor);
            match maybe_index {
                Some(index) => {
                    if index < self.doc.len_children() {
                        let child = self.doc.get_child_mut(index);
                        if let ValueRefMut::Table(table) = child.value_mut() {
                            Manifest::set_output_target_inline_table(table, target);
                        } else {
                            unreachable!()
                        }
                    } else {
                        let children_len = self.doc.len_children();
                        let container = self.doc.get_container_mut(index - children_len);
                        Manifest::set_output_target_inner(container, target);
                    }
                }
                None => {
                    match self.doc.get_mut(target.kind) {
                        Some(EntryRefMut::Array(array)) => {
                            match array.to_value() {
                                ArrayValueMut::Inline(mut inline_array) => {
                                    Manifest::set_output_target_inline_array(inline_array, target);
                                }
                                ArrayValueMut::OfTables => unreachable!()
                            }
                        }
                        _ => unreachable!()
                    }
                }
            }
            None
        }
    }

    fn set_output_target_inner(container: &mut Container, target: OutputTarget) {
        fn set_bool(cnt: &mut Container, key: &'static str, value: Option<bool>) {
            if let Some(value) = value {
                Manifest::remove_child(cnt, key);
                let index = cnt.len_children();
                cnt.insert_boolean(index, key, value);
            }
        }
        fn set_string<'a>(cnt: &mut Container, key: &'static str, value: Option<&'a str>) {
            if let Some(value) = value {
                Manifest::remove_child(cnt, key);
                let index = cnt.len_children();
                cnt.insert_string(index, key, value);
            }
        }
        set_output_target!(container, target, set_string, set_bool);
    }

    fn set_output_target_inline_array(inline_array: &mut InlineArray, target: OutputTarget) {
        let idx = inline_array.find(&NodeCursor(target.handle)).unwrap();
        let value = inline_array.get_mut(idx);
        let table = if let ValueRefMut::Table(table) = value {
            table
        } else {
            panic!("Invalid operation");
        };
        Manifest::set_output_target_inline_table(table, target);
    }

    fn set_output_target_inline_table(table: &mut InlineTable, target: OutputTarget) {
        fn set_bool(table: &mut InlineTable, key: &'static str, value: Option<bool>) {
            if let Some(value) = value {
                Manifest::remove_child_inline(table, key);
                let index = table.len();
                table.insert_boolean(index, key, value);
            }
        }
        fn set_string<'a>(table: &mut InlineTable, key: &'static str, value: Option<&'a str>) {
            if let Some(value) = value {
                Manifest::remove_child_inline(table, key);
                let index = table.len();
                table.insert_string(index, key, value);
            }
        }
        set_output_target!(table, target, set_string, set_bool);
    }

    fn remove_output_target(&mut self, handle: usize, kind: &str) {
        fn remove_child(doc: &mut Document, key: &str) {
            if let Some(idx) = doc.iter_children().position(|c| key == c.key().get()) {
                doc.remove(idx)
            }
        }
        if handle == 0 {
            remove_child(&mut self.doc, kind);
            Manifest::remove_containers(&mut self.doc, iter::once(kind));
        } else {
            let position = self.doc.find(&NodeCursor(handle)).unwrap();
            self.doc.remove(position);
        }
    }

    fn remove_child(cnt: &mut Container, key: &str) {
        if let Some(idx) = cnt.iter_children().position(|c| key == c.key().get()) {
            cnt.remove(idx)
        }
    }

    fn remove_child_inline(table: &mut InlineTable, key: &str) {
        if let Some(idx) = table.iter().position(|c| key == c.key().get()) {
            table.remove(idx)
        }
    }

    fn remove_containers<'a, I:Iterator<Item=&'a str>+Clone>(doc: &mut Document, keys: I) {
        fn find_container<'a, I>(doc: &mut Document, keys: I)
                                -> Option<usize> where I:Iterator<Item=&'a str>+Clone {
            doc.iter_containers().position(|c| {
                c.keys().markup().iter().zip(keys.clone()).all(|(m, k)| m.get() == k)
            })
        }
        loop {
            let position = find_container(doc, keys.clone());
            match position {
                Some(position) => {
                    let len_children = doc.len_children();
                    doc.remove(position + len_children);
                }
                None => break
            }
        }
    }
}
impl std::panic::RefUnwindSafe for Manifest { }

struct NodeCursor(usize);
impl InternalNode for NodeCursor {
    fn ptr(&self) -> usize {
        self.0
    }
}



pub enum QueryError {
    Vacant{ depth: usize },
    Conflict{ depth: usize, kind: &'static str }
}

pub struct Dependency<'a> {
    name: &'a str,
    version: Option<&'a str>,
    git: Option<&'a str>,
    path: Option<&'a str>,
    target: Option<&'a str>
}

impl<'a> Dependency<'a> {
    fn simple(name: &'a str, target: Option<&'a str>, version: &'a str) -> Dependency<'a> {
        Dependency {
            name: name,
            version: Some(version),
            git: None,
            path: None,
            target: target
        }
    }

    fn complex(name: &'a str, target: Option<&'a str>, table: TableEntry<'a>) -> Dependency<'a> {
        fn get_string<'b>(tabl: TableEntry<'b>, key: &'b str) -> Option<&'b str> {
            match tabl.get(key) {
                Some(EntryRef::String(s)) => Some(s.get()),
                _ => None
            }
        }
        Dependency {
            name: name,
            version: get_string(table, "version"),
            git: get_string(table, "git"),
            path: get_string(table, "path"),
            target: target,
        }
    }
}

pub struct PathError {
    path: String,
    expected: &'static str,
    got: &'static str,
}

pub struct OutputTarget<'a> {
    handle: usize,
    kind: &'a str,
    name: Option<&'a str>,
    path: Option<&'a str>,
    test: Option<bool>,
    doctest: Option<bool>,
    bench: Option<bool>,
    doc: Option<bool>,
    plugin: Option<bool>,
    harness: Option<bool>
}

impl<'a> OutputTarget<'a> {
    fn new(handle: usize, kind: &'static str) -> OutputTarget<'a> {
        OutputTarget {
            handle: handle,
            kind: kind,
            name: None,
            path: None,
            test: None,
            doctest: None,
            bench: None,
            doc: None,
            plugin: None,
            harness: None
        }
    }

    fn bin(handle: usize) -> OutputTarget<'a> {
        OutputTarget::new(handle, "bin")
    }

    fn lib(handle: usize) -> OutputTarget<'a> {
        OutputTarget::new(handle, "lib")
    }

    fn bench(handle: usize) -> OutputTarget<'a> {
        OutputTarget::new(handle, "bench")
    }

    fn test(handle: usize) -> OutputTarget<'a> {
        OutputTarget::new(handle, "test")
    }

    fn example(handle: usize) -> OutputTarget<'a> {
        OutputTarget::new(handle, "example")
    }
}

#[repr(C)]
pub struct RawSlice<T> {
    arr: *mut T,
    len: INT32
}

impl<T> RawSlice<T> {
    fn empty() -> RawSlice<T> {
        RawSlice {
            arr: ptr::null_mut(),
            len: 0
        }
    }

    fn from_vec(vec: Vec<T>) -> RawSlice<T> {
        let mut boxed = vec.into_boxed_slice();
        let result = RawSlice {
            arr: boxed.as_mut_ptr(),
            len: boxed.len() as INT32
        };
        mem::forget(boxed);
        result
    }
}

#[repr(C)]
pub struct OwnedSlice<T> {
    data: RawSlice<T>
}

impl<T> Drop for OwnedSlice<T> {
    fn drop(&mut self) {
        let this = &mut self.data;
        if this.arr != ptr::null_mut() {
            let slice = unsafe { slice::from_raw_parts_mut(this.arr, this.len as usize) };
            drop(unsafe { Box::from_raw(slice) });
            this.arr = ptr::null_mut();
            this.len = 0;
        }
    }
}
impl<T> OwnedSlice<T> {
    fn empty() -> OwnedSlice<T> {
        OwnedSlice {
            data: RawSlice::empty()
        }
    }
}

impl<T> OwnedSlice<T> {
    fn from_slice<F, E>(src: &[E], f:F) -> OwnedSlice<T> where F: FnMut(&E) -> T {
        let vec = src.iter().map(f).collect::<Vec<_>>();
        OwnedSlice {
            data: RawSlice::from_vec(vec)
        }
    }
}

impl OwnedSlice<u8> {
    fn from_str_opt(src: Option<&str>) -> OwnedSlice<u8> {
        match src {
            Some(s) => OwnedSlice::from_string(s),
            None => OwnedSlice::empty()
        }
    }

    fn from_string<S:Into<String>>(src: S) -> OwnedSlice<u8> {
        let mut text = src.into().into_bytes().into_boxed_slice();
        let inner = RawSlice {
            arr: text.as_mut_ptr(),
            len: text.len() as INT32
        };
        let result = OwnedSlice{ data: inner };
        mem::forget(text);
        result
    }
}

#[repr(C)]
pub struct BorrowedSlice<'a, T: 'a> {
    data: RawSlice<T>,
    marker: PhantomData<&'a T>
}

impl<'a> BorrowedSlice<'a, u8> {
    fn as_str(&'a self) -> &'a str {
        unsafe {
            let slice = slice::from_raw_parts(self.data.arr, self.data.len as usize);
            str::from_utf8_unchecked(slice)
        }
    }

    fn as_str_opt(&'a self) -> Option<&'a str> {
        if self.data.arr == ptr::null_mut() {
            None
        } else {
            Some(self.as_str())
        }
    }
}

impl<'a> BorrowedSlice<'a, BorrowedSlice<'a, u8>> {
    fn as_str_vec<'b>(&'b self) -> Vec<&'b str> {
        let mut vec = Vec::with_capacity(self.data.len as usize);
        for i in 0..(self.data.len as isize) {
            vec.push(unsafe { &*self.data.arr.offset(i) }.as_str());
        }
        vec
    }
}

impl<'a, T> BorrowedSlice<'a, T> {
    fn empty() -> BorrowedSlice<'a, T> {
        BorrowedSlice {
            data: RawSlice::empty(),
            marker: PhantomData
        }
    }
}

impl<T> BorrowedSlice<'static, T> {
    fn from_static(string: &str) -> BorrowedSlice<'static, T> {
        BorrowedSlice  {
            data: RawSlice {
                arr: string.as_ptr() as *mut _,
                len: string.len() as INT32
            },
            marker: PhantomData
        }
    }
}

#[repr(C)]
pub struct RawDependency {
    name: OwnedSlice<u8>,
    version: OwnedSlice<u8>,
    git: OwnedSlice<u8>,
    path: OwnedSlice<u8>,
    target: OwnedSlice<u8>
}

impl RawDependency {
    fn from(d: &Dependency) -> RawDependency {
        RawDependency {
            name: OwnedSlice::from_string(d.name),
            version: OwnedSlice::from_str_opt(d.version),
            git: OwnedSlice::from_str_opt(d.git),
            path: OwnedSlice::from_str_opt(d.path),
            target: OwnedSlice::from_str_opt(d.target)
        }
    }
}