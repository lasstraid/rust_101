fn remove_element(nums: &mut Vec<i32>, value: i32) -> i32 {
    let mut k: usize = nums.len();
    let mut i: usize = 0;
    while i < k {
        if nums[i] == value {
            k -= 1;
            nums[i] = nums[k];
        } else {
            i += 1;
        }
    }
    return k as i32;
}

fn main() {}
