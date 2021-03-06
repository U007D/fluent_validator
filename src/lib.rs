#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

mod consts;
mod error;

#[cfg(test)] mod unit_tests;

pub use self::error::Error;
pub type Result<T> = std::result::Result<T, Error>;

pub trait Validator<T> {
    fn validate(T) -> Result<T>;
}

pub trait FluentValidator: Sized {
    fn validate<T: Validator<Self>>(self) -> Result<Self>;
}

impl<T> FluentValidator for T {
    fn validate<U: Validator<Self>>(self) -> Result<Self> {
        U::validate(self)
    }
}
