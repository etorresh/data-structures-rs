use std::cmp::Ordering;

// Intersection of two sorted arrays
pub fn intersection(nums1: Vec<i32>, nums2: Vec<i32>) -> Vec<i32> {
    let mut result: Vec<i32> = Vec::new();
    let mut nums1_iter = nums1.iter().peekable();
    let mut nums2_iter = nums2.iter().peekable();
    
    loop {
        if let (Some(&&i), Some(&&j)) = (nums1_iter.peek(), nums2_iter.peek())  {
            match i.cmp(&j) {
                Ordering::Less => {nums1_iter.next();},
                Ordering::Greater => {nums2_iter.next();},
                Ordering::Equal => {
                    if result.last().map_or(true, |&last| last != i) {
                        result.push(i);
                    };
                    nums1_iter.next();
                    nums2_iter.next();
                },
            };
        } else {
            return result;
        }
    }
}