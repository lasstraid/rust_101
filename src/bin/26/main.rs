
fn remove_duplicates(nums: &mut Vec<i32>) -> i32 {
  if nums.len() <= 1 {
    return nums.len() as i32; 
  }
  
  let mut writer: usize = 1;
  let mut reader: usize = 1;
  
  while reader < nums.len() {
    if nums[reader] != nums[writer - 1] {
      nums[writer] = nums[reader];
      writer += 1;
    }
    reader += 1;
  }

  return (writer + 1) as i32;
}

fn main() {

}