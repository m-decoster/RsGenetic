// file: types.rs
//
// Copyright 2015-2016 The RsGenetic Developers
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
// 	http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! This module provides implementations of the `Fitness` trait for
//! some primitive types, such as `i32`, `i64` etcetera.
//! This is because Rust does not allow programmers to implement
//! a foreign trait for a foreign type, which would stop you as a library user
//! from using primitive types as fitness values.

use pheno::Fitness;

impl Fitness for i32 {
    fn zero() -> i32 {
        0
    }

    fn abs_diff(&self, other: &i32) -> i32 {
        (self - other).abs()
    }
}

impl Fitness for i64 {
    fn zero() -> i64 {
        0
    }

    fn abs_diff(&self, other: &i64) -> i64 {
        (self - other).abs()
    }
}

impl Fitness for u32 {
    fn zero() -> u32 {
        0
    }

    fn abs_diff(&self, other: &u32) -> u32 {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

impl Fitness for u64 {
    fn zero() -> u64 {
        0
    }

    fn abs_diff(&self, other: &u64) -> u64 {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}

impl Fitness for usize {
    fn zero() -> usize {
        0
    }

    fn abs_diff(&self, other: &usize) -> usize {
        if self > other {
            self - other
        } else {
            other - self
        }
    }
}
