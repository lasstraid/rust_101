fn majority_element(nums: Vec<i32>) -> i32 {
    let mut answer: i32 = nums[0];
    let mut count: i32 = 1;

    let mut index: usize = 1;

    while index < nums.len() {
        if nums[index] == answer {
            count += 1;
        } else {
            count -= 1;
        }

        if count == 0 {
            answer = nums[index];
            count = 1;
        }

        index += 1;
    }

    return answer;
}

fn main() {
    println!("{}", majority_element(vec![3, 2, 3]));
    println!("{}", majority_element(vec![2, 2, 1, 1, 1, 2, 2]));
}
