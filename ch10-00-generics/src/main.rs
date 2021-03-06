use std::cmp::PartialOrd;
// fn largest(list: &[i32]) -> i32 {
fn largest<T: PartialOrd + Copy>(list: &[T]) -> T{
  let mut largest = list[0];
  for &item in list.iter(){
    if item > largest {
      largest = item;
    }
  }
  largest
}
fn main() {
  let number_list = vec![34, 50, 25, 100, 65];

  let result = largest(&number_list);
  println!("最大値は{}です", result);
}
