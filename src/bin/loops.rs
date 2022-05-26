fn main() {
  let mut i = 6;
  
  loop {
    println!("{:?}", i);
    i = i - 1;

    if i == 0 {
      break;
    }
  }

  println!("===============");

  let mut counter = 5;
  while counter >= 1 {
    println!("{:}", counter);
    counter = counter - 1
  }
}