// RCL -- A sane configuration language.
// Copyright 2023 Ruud van Asseldonk

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

//! Error types.

use crate::source::Span;

#[derive(Debug)]
pub struct Error {
    // Not dead, used in the debug impl.
    #[allow(dead_code)]
    message: &'static str,
}

pub type Result<T> = std::result::Result<T, Error>;

impl Error {
    // TODO: Add a better error type which records source span info etc.
    pub fn new(message: &'static str) -> Error {
        Error { message }
    }
}

impl From<&'static str> for Error {
    fn from(message: &'static str) -> Error {
        Error { message }
    }
}

/// A syntax error that causes lexing or parsing to fail.
#[derive(Debug)]
pub struct ParseError {
    pub span: Span,
    pub message: &'static str,
    pub note: Option<(&'static str, Span)>,
}