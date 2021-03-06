// Copyright 2012 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

use std::thread::Thread;

fn borrow<F>(v: &int, f: F) where F: FnOnce(&int) {
    f(v);
}

fn box_imm() {
    let v = box 3i;
    let _w = &v;
    Thread::spawn(move|| {
        println!("v={}", *v);
        //~^ ERROR cannot move `v` into closure
    });
}

fn box_imm_explicit() {
    let v = box 3i;
    let _w = &v;
    Thread::spawn(move|| {
        println!("v={}", *v);
        //~^ ERROR cannot move
    });
}

fn main() {
}
