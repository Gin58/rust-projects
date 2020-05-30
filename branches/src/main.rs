fn main() {
  let number = 7;

  if number < 5 {
    println!("condition was true");
  } else {
    println!("condition was false");
  }

  if number != 0 {
    println!("number was something other than zero");
  }

  if number % 4 == 0 {
    println!("number is divisible by 4");
  } else if number % 3 == 0 {
    println!("number is divisible by 3");
  } else if number % 2 == 0 {
    println!("number is divisible by 2");
  } else {
    println!("number is not divisible by 4, 3, or 2");
  }

  let condition = true;
  let num = if condition {
    5
  } else {
    6
  };
  println!("The value of num is: {}", num);

  while_loop();
  item_loop();
  for_loop();
  lift_off();

  str();
  reff();
}

fn while_loop() {
  let mut number = 3;
  while number != 0 {
    println!("{}!", number);
    number = number - 1;
  }

  println!("LIFT OFF!!")
}

fn item_loop() {
  let a = [10, 20, 30, 40, 50];
  let mut index = 0;

  while index < 5 {
    println!("The value is: {}", a[index]);
    index = index + 1;
  }
}

fn for_loop() {
  let a = [10, 20, 30, 40, 50];
  for element in a.iter() {
    println!("the value is: {}", element);
  }
}

fn lift_off() {
  for number in (1..4).rev() {
    println!("{}!", number);
  }
  println!("Lift Off!");
}

fn str() {
  let mut s = String::from("Hello");
  s.push_str(", world!");
  println!("{}", s);
}

fn reff() {
  let s1 = String::from("hello");
  let len = calculate_length(&s1);
  println!("The length of '{}' is: {}", s1, len);
}

fn calculate_length(s: &String) -> usize {
  s.len()
}

fn first_word(s: &String) -> &str {
  let bytes = a.as_bytes();

  for (i, &item) in bytes.iter().enumerate() {
    if item == b' ' {
      return &s[0..i];
    }
  }
  &s[..]
}