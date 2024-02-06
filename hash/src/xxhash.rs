// Copyright 2023 Zinc Labs Inc.
//
// This program is free software: you can redistribute it and/or modify
// it under the terms of the GNU Affero General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.
//
// This program is distributed in the hope that it will be useful
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU Affero General Public License for more details.
//
// You should have received a copy of the GNU Affero General Public License
// along with this program.  If not, see <http://www.gnu.org/licenses/>.

use std::hash::Hasher;

use xxhash_rust::xxh3::Xxh3;

use super::Sum64;

pub fn new() -> Xxh3 {
    Xxh3::new()
}

impl Sum64 for Xxh3 {
    fn sum64(&mut self, key: &str) -> u64 {
        self.write(key.as_bytes());
        self.finish()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xxhash_sum64() {
        let mut h = new();
        assert_eq!(h.sum64("hello"), 10760762337991515389);
        assert_eq!(h.sum64("world"), 18436838148490100038);
        assert_eq!(h.sum64("foo"), 5289624599890063979);
        assert_eq!(h.sum64("bar"), 14344799839972817241);
        assert_eq!(h.sum64("test"), 14444285316842008032);
        assert_eq!(h.sum64("test1"), 826316940449227468);
        assert_eq!(h.sum64("test2"), 10866895676741295672);
    }
}
