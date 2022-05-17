pub fn merge(arr: &mut Vec<i32>, left: usize, mid: usize, right: usize) {
  let mut temp = Vec::new();
  let mut i = left;
  let mut j = mid+1;
  println!("Sorting from {} to {} to {}", i, mid, j);
  while i <= mid && j <= right {
    println!("Left array: {:?}, Right array: {:?}, Temp array: {:?}", &arr[i..mid], &arr[j..right], &temp);
    if arr[i] < arr[j] {
      temp.push(arr[i]);
      i += 1;
    } else {
      temp.push(arr[j]);
      j += 1;
    }
    if i <= mid && j >= right {
      temp.push(arr[i]);
      i += 1;
    } else if i >= mid && j <= right {
      temp.push(arr[j]);
      j += 1;
    }
  }
  let mut k = 0;
  println!("Temp: {:?}", &temp);
  for i in left..right {
    arr[i] = temp[k];
    k += 1;
  }
}

pub fn mergesort(arr: &mut Vec<i32>, left: usize, right: usize) {
  if right > left {
    let mid = left+(right-left) / 2;
    mergesort(arr, left, mid);
    mergesort(arr, mid+1, right);
    merge(arr, left, mid, right);
  }
}
