pub fn bubblesort(arr: &mut Vec<i32>, swapped: Option<bool>) {
  // this tracks swaps
  // if a swap doesn't take plae during an iteration is should be sorted
  let mut swapped = swapped.unwrap_or(true);
  // base case
  if !swapped {
    return;
  }
  let mut swapped = Some(false);
  // Loops through all elements and check if prev elements are greater than later elements
  for i in 1..arr.len()-1 {
    // if 
    if arr[i-1] > arr[i] {
      swapped = Some(true);
      arr.swap(i, i-1);
    }
    if arr[i] > arr[i+1] {
      swapped = Some(true);
      arr.swap(i, i+1);
    }
  }
  // Run until no swaps take place
  bubblesort(arr, swapped);
}