// Copyright 2023 The Fuchsia Authors
//
// Licensed under a BSD-style license <LICENSE-BSD>, Apache License, Version 2.0
// <LICENSE-APACHE or https://www.apache.org/licenses/LICENSE-2.0>, or the MIT
// license <LICENSE-MIT or https://opensource.org/licenses/MIT>, at your option.
// This file may not be copied, modified, or distributed except according to
// those terms.

extern crate zerocopy;

use zerocopy::transmute_mut;

fn main() {}

#[derive(zerocopy::FromBytes, zerocopy::IntoBytes)]
#[repr(C)]
struct Src;

#[derive(zerocopy::FromBytes, zerocopy::IntoBytes)]
#[repr(C)]
struct Dst;

// `transmute_mut` does not require that the source type implements `NoCell`
const SRC_NOT_NO_CELL: &mut Dst = transmute_mut!(&mut Src);
