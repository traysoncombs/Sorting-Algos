mod sorting;
mod utils;

use crate::sorting::*;
use crate::utils::*;

fn main() {
  let quickie = |arr: &mut Vec<i32>|{
    let len = arr.len() - 1;
    quicksort::quicksort(arr, 0, len);
  };
  let bubbly = |arr: &mut Vec<i32>|{
    bubblesort::bubblesort(arr, None);
  };
  let mergie = |arr: &mut Vec<i32>|{
    let len = arr.len() - 1;
    mergesort::mergesort(arr, 0, len);
  };
  test::run_test(mergie, 10, 20);
}