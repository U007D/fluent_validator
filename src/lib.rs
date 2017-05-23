#![cfg_attr(feature="clippy", feature(plugin))]
#![cfg_attr(feature="clippy", plugin(clippy))]
#![allow(non_camel_case_types)]
#![warn(missing_debug_implementations, missing_copy_implementations, trivial_casts, trivial_numeric_casts, unused_import_braces, unused_qualifications)]
#![deny(unused_must_use, overflowing_literals)]

mod consts;

#[cfg(test)] mod unit_tests;

#[allow(dead_code)] type GeneralError = Box<std::error::Error>;
#[allow(dead_code)] type GeneralResult<T> = Result<T, GeneralError>;
type ValidatorResult<T> = Result<T, Error>;

#[derive(Debug, PartialEq, Eq)]
pub enum Error {
    FailedConstraint(String),
}

pub trait Validator<T> {
    fn validate(T) -> Result<Self, Error> where Self: Sized;
}

trait FluentValidator: Sized {
    fn validate<T: Validator<Self>>(self) -> ValidatorResult<T>;
}

impl<T> FluentValidator for T {
    fn validate<U: Validator<T>>(self) -> ValidatorResult<U> {
        U::validate(self)
    }
}

