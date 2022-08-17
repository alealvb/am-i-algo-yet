pub struct BinarySearchList {}

impl BinarySearchList {
    pub fn binary_search_list(haystack: &[i32], needle: i32) -> bool {
        let mut lo: usize = 0;
        let mut hi: usize = haystack.len();

        while lo < hi {
            let middle: usize = (lo + hi) / 2;
            let num = haystack[middle];

            if num == needle {
                return true;
            }

            if num > needle {
                hi = middle;
            } else {
                lo = middle + 1;
            }
        }

        return false;
    }
}
