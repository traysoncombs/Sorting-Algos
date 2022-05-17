pub fn verify(arr: &Vec<i32>) -> bool {
  if arr.len() == 0 || arr.len() == 1 {
    return true;
  }
  for i in 1..arr.len()-1 {
    if arr[i-1] > arr[i] {
      println!("false");
      return false;
    }
  }
  true
}