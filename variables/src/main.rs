fn main() {
  let mut x = 5;
  println!("The value of x is: {}", x);
  x = 6;
  println!("The value of x is: {}", x);

  let y = 5;
  let y = y + 1;
  let y = y * 2;
  println!("The value of y is: {}", y);

  println!("Hello, world");
  another_function(5);
}

fn another_function(x: i32) {
  println!("The value of x is: {}", x);
}