fn rotate(nums: &mut Vec<i32>, k: i32) {
    if nums.len() <= 1 {
        return;
    }

    let n = nums.len();
    let k = (k as usize) % n;

    // reverse whole array
    for i in 0..k {
        let index = nums.len() - 1 - i;
        let temp: i32 = nums[i];
        nums[i] = nums[index];
        nums[index] = temp;
    }

    // reverse firt k elements
    for i in 0..k / 2 {
        let index = k - 1 - i;
        let temp: i32 = nums[i];
        nums[i] = nums[index];
        nums[index] = temp;
    }

    // reverse rest elements
    for i in 0..(n - k) / 2 {
        let index = n - 1 - i;
        let temp: i32 = nums[k + i];
        nums[k + i] = nums[index];
        nums[index] = temp;
    }
}

fn main() {
    let mut nums: Vec<i32> = vec![1, 2, 3, 4, 5, 6, 7];
    rotate(&mut nums, 3);
    println!("{:?}", nums);
}
