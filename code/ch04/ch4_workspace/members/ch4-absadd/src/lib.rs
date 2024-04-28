fn absolute(a: i32) -> i32 {
  if a < 0 {
    return -a;
  }
  a
}

/// Retuns sum two absolute value
pub fn add_abs(a: i32, b: i32) -> i32 {
  let c = absolute(a) + absolute(b);
  c
}
