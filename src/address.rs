pub fn address() {
  let x = 23781;
  let s = String::from("foo");

  println!("value of {x} is and address of x is {:p}", &x);
  println!("value of '{s}' is and address of s is {:p}", &s);
}
