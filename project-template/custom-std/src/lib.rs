#![no_implicit_prelude]

pub use ::std::array;
pub use ::std::borrow;
pub use ::std::boxed;
pub use ::std::cell;
pub use ::std::char;
pub use ::std::clone;
pub use ::std::cmp;
pub use ::std::collections;
pub use ::std::convert;
pub use ::std::default;
pub use ::std::error;
pub use ::std::f32;
pub use ::std::f64;
pub use ::std::fmt;
pub use ::std::future;
pub use ::std::hash;
pub mod hint {
    pub use ::std::hint::black_box;
}
pub use ::std::iter;
pub use ::std::marker;
pub mod mem {
    pub use ::std::mem::{align_of, align_of_val, discriminant, drop, needs_drop, size_of, size_of_val, swap, take};
}
pub use ::std::num;
pub use ::std::ops;
pub use ::std::option;
pub use ::std::primitive;
pub use ::std::rc;
pub use ::std::result;
pub use ::std::slice;
pub use ::std::str;
pub use ::std::string;
pub use ::std::sync;
pub use ::std::task;
pub use ::std::time;
pub use ::std::vec;

pub mod thread {
    pub use ::std::thread::sleep;
}

pub use prelude::rust_2021::*;

pub mod prelude {
    pub mod rust_2021 {
        pub use ::std::{assert, assert_eq, assert_ne, dbg, format, matches, panic, print, println, todo, unimplemented, unreachable, vec};
        pub use ::std::prelude::rust_2021::{FromIterator, TryFrom, TryInto, Copy, Send, Sized, Sync, Drop, Fn, FnMut, FnOnce, drop, Clone, Eq, Ord, PartialEq, PartialOrd, AsMut, AsRef, From, Into, Default, DoubleEndedIterator, ExactSizeIterator, Extend, IntoIterator, Iterator, Option, None, Some, Result, Err, Ok, Debug, Hash, ToOwned, Box, String, ToString, Vec };
    }
}