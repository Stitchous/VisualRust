
extern crate pcre;
extern crate std; // equivalent to: extern crate std as std;
extern crate std as ruststd; // linking to 'std' under another name
pub extern crate pcre; //warning: `pub extern crate` does not work as expected and should not be used. Likely to become an error. Prefer `extern crate` and `pub use`. 

use p::q::r as x;
use a::b::{c,d,e,f};
use a::b::*;
use a::b::{self, c, d};
use foo::baz::foobaz;

// Load the `vec` module from `vec.rs`
mod vec;

mod thread {
    // Load the `local_data` module from `thread/local_data.rs`
    // or `thread/local_data/mod.rs`.
    mod local_data;
}

mod quux {
    pub use quux::foo::{bar, baz};

    pub mod foo {
        //pub fn bar() { }
        //pub fn baz() { }
    }
}

// A conditionally-compiled module
#[cfg(target_os="linux")]
mod bar {
        /* ... */
}
