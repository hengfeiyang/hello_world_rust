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

use super::Sum64;

pub struct CityHash {}

pub fn new() -> CityHash {
    CityHash {}
}

impl Sum64 for CityHash {
    fn sum64(&mut self, key: &str) -> u64 {
        cityhasher::hash(key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_murmur3_sum64() {
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
