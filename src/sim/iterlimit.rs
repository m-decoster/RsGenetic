// file: iterlimit.rs
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

/// An iteration limiter.
#[derive(Copy, Clone, Debug)]
pub struct IterLimit {
    /// Maximum number of iterations allowed.
    max: u64,
    /// Current number of iterations.
    cur: u64,
}

impl IterLimit {
    /// Create a new iteration limiter.
    pub fn new(max: u64) -> IterLimit {
        IterLimit { max: max, cur: 0 }
    }

    /// Increment the number of iterations.
    pub fn inc(&mut self) {
        self.cur += 1;
    }

    /// Check if the maximum has been reached.
    pub fn reached(&self) -> bool {
        self.cur >= self.max
    }

    /// Reset the number of iterations to zero.
    pub fn reset(&mut self) {
        self.cur = 0;
    }

    /// Get the current number of iterations.
    pub fn get(&self) -> u64 {
        self.cur
    }
}

#[cfg(test)]
mod tests {
    use super::IterLimit;

    #[test]
    fn test_iter_limit_reset() {
        let mut limit = IterLimit::new(5);
        for _ in 0..4 {
            limit.inc();
        }
        assert_eq!(limit.reached(), false);
        limit.reset();
        assert_eq!(limit.reached(), false);
    }

    #[test]
    fn test_iter_limit_reached() {
        let mut limit = IterLimit::new(5);
        for _ in 0..5 {
            limit.inc();
        }
        assert!(limit.reached());
        for _ in 0..10 {
            limit.inc();
        }
        assert!(limit.reached());
        assert_eq!(limit.get(), 15);
    }
}
