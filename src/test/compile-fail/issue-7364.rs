// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.


use std::cell::RefCell;

// Regresion test for issue 7364
static boxed: Box<RefCell<int>> = box RefCell::new(0);
//~^ ERROR statics are not allowed to have custom pointers
//~| ERROR: the trait `core::marker::Sync` is not implemented for the type
//~| ERROR: the trait `core::marker::Sync` is not implemented for the type

fn main() { }
