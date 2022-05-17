use rand::prelude::*;
use crate::verify::*;
pub fn gen_test_cases(cases_num: i32, cases_size: i32) -> Vec<Vec<i32>> {
  let mut rng = rand::thread_rng();
  let mut cases = Vec::new();
  for _ in 0..cases_num {
    let mut unsorted: Vec<i32> = (1..cases_size).collect();
    unsorted.shuffle(&mut rng);
    cases.push(unsorted);
  }
  cases
}

pub fn run_test(sorter: fn(&mut Vec<i32>), cases_num: i32, cases_size: i32) {
  for mut i in gen_test_cases(cases_num, cases_size) {
    println!("Sorting {:?}", i);
    sorter(&mut i);
    println!("Sorted {:?}", &i);
    assert!(verify(&i));
  }
}