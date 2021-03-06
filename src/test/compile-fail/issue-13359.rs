// Copyright 2014 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

fn foo(_s: i16) { }

fn bar(_s: u32) { }

fn main() {
    foo(1*(1 as int));
    //~^ ERROR: mismatched types: expected `i16`, found `isize` (expected i16, found isize)

    bar(1*(1 as uint));
    //~^ ERROR: mismatched types: expected `u32`, found `usize` (expected u32, found usize)
}
