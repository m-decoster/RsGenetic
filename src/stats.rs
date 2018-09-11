// file: stats.rs
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

use pheno::Fitness;

/// A collector for potential stats on the population's fitness or timing.
///
pub trait StatsCollector<F: Fitness> {
    ///Executed before a step, passing the current population's fitness
    ///
    fn before_step(&mut self, pop_fitness: &[F]) {}

    /// Executed after a step passing in the current population's fitness.
    ///
    fn after_step(&mut self, pop_fitness: &[F]) {}
}

/// A NOOP implementation for common fitness types
///
#[derive(Debug, Clone, Copy)]
pub struct NoStats {}
impl StatsCollector<i32> for NoStats {}
impl StatsCollector<i64> for NoStats {}
impl StatsCollector<u32> for NoStats {}
impl StatsCollector<u64> for NoStats {}
