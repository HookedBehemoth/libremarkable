#![feature(integer_atomics)]
#![feature(const_size_of)]

#[macro_use]
extern crate ioctl_gen;

extern crate libc;
extern crate mmap;
extern crate rusttype;

pub extern crate image;
pub extern crate epoll;
pub extern crate rb;
pub extern crate evdev;
pub extern crate line_drawing;

pub mod mxc_types;
pub mod fb;
pub mod fbio;
pub mod fbdraw;
pub mod refresh;
pub mod ev;

pub mod unifiedinput;
pub mod uix;