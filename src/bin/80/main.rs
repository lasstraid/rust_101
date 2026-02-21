fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
    if nums.len() <= 2 {
        return nums.len() as i32;
    }

    let mut writer: usize = 2;
    let mut reader: usize = 2;

    while reader < nums.len() {
        if (nums[reader] != nums[writer - 2]) {
            nums[writer] = nums[reader];
            writer += 1;
        }
        reader += 1;
    }

    return writer as i32;
}

fn main() {}
