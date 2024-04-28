#[macro_export]
macro_rules! perma_string { // <1>
  () => { // <2>
      String::from("hello")
  }
}