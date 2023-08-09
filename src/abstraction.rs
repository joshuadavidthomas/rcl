// RCL -- A sane configuration language.
// Copyright 2023 Ruud van Asseldonk

// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// A copy of the License has been included in the root of the repository.

//! Abstraction converts a Concrete Syntax Tree into an Abstract Syntax Tree.
//!
//! It is responsible for a few things:
//!
//! * Forgetting about non-code such as blank lines and comments.
//! * Converting literals in the source code into values in the runtime.
//! * Removing syntactical differences (e.g. converting `k = v;` into `"k": v`).

use crate::ast::{Expr as AExpr, Seq as ASeq};
use crate::cst::Prefixed;
use crate::cst::{Expr as CExpr, Seq as CSeq};

/// Abstract an expression.
pub fn abstract_expr(input: &str, expr: &Prefixed<CExpr>) -> AExpr {
    Abstractor::new(input).expr(&expr.inner)
}

/// The abstractor can convert CST nodes to AST nodes for a given document.
struct Abstractor<'a> {
    input: &'a str,
}

impl<'a> Abstractor<'a> {
    pub fn new(input: &'a str) -> Self {
        Self { input }
    }

    /// Abstract an expression.
    pub fn expr(&self, expr: &CExpr) -> AExpr {
        match expr {
            CExpr::Let { ident, value, body } => AExpr::Let {
                ident: ident.resolve(self.input).into(),
                value: Box::new(self.expr(&value)),
                body: Box::new(self.expr(&body.inner)),
            },
            CExpr::BraceLit { elements, .. } => {
                AExpr::BraceLit(elements.iter().map(|elem| self.seq(&elem.inner)).collect())
            }
            todo => unimplemented!("{:?}", todo),
        }
    }

    /// Abstract a sequence element.
    pub fn seq(&self, seq: &CSeq) -> ASeq {
        match seq {
            CSeq::Elem { value } => ASeq::Elem(Box::new(self.expr(value))),
            CSeq::AssocIdent { field, value } => {
                // We convert the `key = value` as if it had been written
                // `"key": value` so we can treat them uniformly from here on.
                let key_str = field.resolve(self.input);
                let key_expr = AExpr::StringLit(key_str.into());
                ASeq::Assoc {
                    key: Box::new(key_expr),
                    value: Box::new(self.expr(value)),
                }
            }
            todo => unimplemented!("{:?}", todo),
        }
    }
}
