// Copyright 2016 The Rust Project Developers. See the COPYRIGHT
// file at the top-level directory of this distribution and at
// http://rust-lang.org/COPYRIGHT.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.

#![feature(reflect_marker)]

use std::marker::Reflect;
use std::any::Any;

struct Foo;

trait Bar {}

impl Bar for Foo {}

fn main() {
    let any: &Any = &Bar; //~ ERROR E0425
                          //~| HELP trait `Bar`
    if any.is::<u32>() { println!("u32"); }
}
