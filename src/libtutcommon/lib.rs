#![deny(deprecated)]
#![deny(missing_docs)]
#![deny(non_snake_case)]
#![deny(non_upper_case_globals)]

#![doc = "Common stuff for tutorials."]
#![crate_name = "tutcommon"]

// Include SDL2 library.
extern crate sdl2;

extern crate byteorder;

extern crate libc;

extern crate gl;

pub mod glutils;

pub mod sdl;

pub mod controls;

pub mod matrix;

