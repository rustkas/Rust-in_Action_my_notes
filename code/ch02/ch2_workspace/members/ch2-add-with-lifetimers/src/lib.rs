use std::ops::Add;

pub fn add_with_lifetimers(i: &i32, j: &i32) -> i32 {
  *i + *j
}

pub fn add_with_lifetimers2<'a, 'b>(i: &'a i32, j: &'b i32) -> i32 {
  *i + *j
}

pub fn add_with_lifetimes<'a, 'b, T>(i: &'a T, j: &'b T) -> T
where
    T: Add<Output = T> + Copy,
{
    *i + *j
}

pub fn add<T: Add<Output = T>>(i:T, j:T) -> T {
  i +j
}