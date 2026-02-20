fn merge(nums1: &mut Vec<i32>, m: i32, nums2: &mut Vec<i32>, n: i32) {
    let mut i = m as usize;
    let mut j = n as usize;
    let mut k = nums1.len();

    while j > 0 {
        k -= 1;
        if i > 0 && nums1[i - 1] >= nums2[j - 1] {
            nums1[k] = nums1[i - 1];
            i -= 1;
        } else {
            nums1[k] = nums2[j - 1];
            j -= 1;
        }
    }
}

fn main() {
    let mut nums1: Vec<i32> = vec![1, 2, 3, 0, 0, 0];
    let m = 3;
    let mut nums2: Vec<i32> = vec![2, 5, 6];
    let n = 3;
    println!("{:?} | {:?}", nums1, nums2);
    merge(&mut nums1, m, &mut nums2, n);
    println!("{:?} | {:?}", nums1, nums2);
}
