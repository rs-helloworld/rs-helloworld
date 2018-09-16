extern crate mbedtls_sys;

use mbedtls_sys::*;
use std::ffi::CStr;

fn main() {

    let some_error = unsafe {
        let mut buf = [0i8; 1024];
        // ---------------------------------------
        // strerror is a function from mbedtls library
        strerror( ERR_MD_FEATURE_UNAVAILABLE, buf.as_mut_ptr(), buf.len() );
        // ---------------------------------------
        CStr::from_ptr( buf.as_ptr() ).to_string_lossy().into_owned()
    };

    println!( "Hello, world!\n{}", some_error );
}
