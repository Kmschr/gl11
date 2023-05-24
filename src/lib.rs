#![crate_name = "gl11"]
#![crate_type = "lib"]

//! An OpenGL loader generated by [gl-rs](https://github.com/brendanzab/gl-rs).
//!
//! This is useful for directly accessing the underlying OpenGL API via the
//! `GlDevice::with_gl` method. It is also used internally by the `GlDevice`
//! implementation.

include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
