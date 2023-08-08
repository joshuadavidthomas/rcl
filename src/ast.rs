// RCL -- A sane configuration language.
// Copyright 2023 Ruud van Asseldonk

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

//! The Abstract Syntax Tree.

pub use crate::cst::{BinOp, UnOp};

/// An identifier.
pub type Ident = &'static str;

/// An expression.
#[derive(Debug)]
pub enum Expr {
    /// A map or set literal, depending on the element types.
    MapLit(Vec<Seq>),

    /// A list literal.
    ListLit(Vec<Seq>),

    /// A string literal.
    StringLit(String),

    // TODO: Having those would require an explicit type for them.
    // It may be nice for some function calls, but for now we can just require
    // the user to wrap them in [] or {}.
    // /// A for-comprehension.
    // Compr(Box<Compr>),
    /// An conditional choice (if, then, else).
    IfThenElse(Box<Expr>, Box<Expr>, Box<Expr>),

    /// Access a variable.
    Var(Ident),

    /// Access a field or key.
    Field(Ident, Box<Expr>),

    /// A let-binding. First is the bound value, then the result expression.
    Let(Ident, Box<Expr>, Box<Expr>),

    /// Call a function.
    Call(Box<Expr>, Vec<Expr>),

    /// Define a function.
    Lam(Vec<Ident>, Box<Expr>),

    /// Apply a unary operator.
    UnOp(UnOp, Box<Expr>),

    /// Apply a binary operator.
    BinOp(BinOp, Box<Expr>, Box<Expr>),
}

/// One or more elements of a sequence.
#[derive(Debug)]
pub enum Seq {
    /// A single element.
    Elem(Box<Expr>),

    /// A `key: value` mapping.
    Assoc(Box<Expr>, Box<Expr>),

    /// A comprehension that loops over the collection.
    For {
        collection: Box<Expr>,
        elements: Vec<Ident>,
        body: Box<Seq>,
    },

    /// Enter the loop only if the condition is true.
    If {
        condition: Box<Expr>,
        body: Box<Seq>,
    },

    /// Let in the middle of a comprehension.
    ///
    /// This is syntactically different from a let before an expression, because
    /// associations are not first-class values.
    Let {
        name: Ident,
        value: Box<Expr>,
        body: Box<Seq>,
    },
}
