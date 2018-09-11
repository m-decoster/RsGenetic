// file: mod.rs
//
// Copyright 2015-2017 The RsGenetic Developers
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

//! The selection module provides a trait that can be implemented
//! to implement new selection algorithms. This module also provides a couple
//! of useful selection algorithms.
//!
//! Each of the selection algorithms provided has a parameter `count`, which indicates the
//! number of selected parents.

mod max;
mod stochastic;
mod tournament;

use pheno::{Fitness, Phenotype};
use std::fmt::Debug;

#[allow(deprecated)]
pub use self::max::MaximizeSelector;
pub use self::stochastic::StochasticSelector;
pub use self::tournament::TournamentSelector;

/// `Parents` come in a `Vec` of two `T`'s.
pub type Parents<T> = Vec<(T, T)>;

/// A `Selector` can select `Parents` for a new iteration of a `Simulation`.
pub trait Selector<T, F>: Debug
where
    T: Phenotype<F>,
    F: Fitness,
{
    /// Select elements from a `population` for breeding.
    ///
    /// If invalid parameters are supplied or the algorithm fails, this function returns an
    /// `Err(String)`, containing a message indicating the error.
    ///
    /// Otherwise it contains a vector of parent pairs wrapped in `Ok`.
    fn select<'a>(&self, population: &'a [T]) -> Result<Parents<&'a T>, String>;
}
