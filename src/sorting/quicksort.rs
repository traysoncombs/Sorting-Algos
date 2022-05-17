pub fn partition(arr: &mut Vec<i32>, low: usize, high: usize) -> usize {
  // set our pivot as the last item in the array
  let pivot = high;
  // This is our 'greatest' item in the array
  let mut i = low;
  // loop through all elems
  for j in low..(high) {
    // If current element is less than pivot and greatest element
    // Swap them
    if arr[j] <= arr[pivot] {
      arr.swap(i, j);
      i += 1;
    }
  }
  // Swap pivot with greatest element at end
  arr.swap(pivot, i);
  i
}

pub fn quicksort(arr: &mut Vec<i32>, low: usize, high: usize) {
  if low > high {
    return
  }
  let m = partition(arr, low, high);
  // base case
  if m == 0 {
    return
  }
  // sort left side
  quicksort(arr, 0, m-1);
  // sort right side
  quicksort(arr, m+1, high);
}