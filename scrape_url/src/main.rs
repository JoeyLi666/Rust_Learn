// use std::fs;
// fn main() {
//   let url = "https://www.rust-lang.org/";
//   let output = "rust.md";
  
//   println!("Fetching url: {}", url);
//   let body = reqwest::blocking::get(url).unwrap().text().unwrap();

//   println!("Converting html to markdown...");
//   let md = html2md::parse_html(&body);

//   fs::write(output, md.as_bytes()).unwrap();
//   println!("Converted markdown has been saved in {}.", output);
// }

// fn apply(value: i32, f: fn(i32) -> i32) -> i32 {
//   f(value)
// }

// fn square(value: i32) -> i32 {
//   value * value
// }

// fn cube(value: i32) -> i32 {
//   value * value * value
// }

// fn main() {
//   println!("apply square: {}", apply(2, square));
//   println!("apply cube: {}", apply(2, cube));
// }



fn fib_loop(n: u8) {
  let mut a = 1;
  let mut b = 1;
  let mut i = 2u8;
  
  loop {
      let c = a + b;
      a = b;
      b = c;
      i += 1;
      
      println!("next val is {}", b);
      
      if i >= n {
          break;
      }
  }
}

fn fib_while(n: u8) {
  let (mut a, mut b, mut i) = (1, 1, 2);
  
  while i < n {
      let c = a + b;
      a = b;
      b = c;
      i += 1;
      
      println!("next val is {}", b);
  }
}

// Rust 的 for 循环可以用于任何实现了 IntoIterator trait 的数据结构
fn fib_for(n: u8) {
  let (mut a, mut b) = (1, 1);
  // 2 =< i < n
  for _i in 2..n {
      let c = a + b;
      a = b;
      b = c;
      println!("next val is {}", b);
  }
}

fn main() {
  let n = 10;
  fib_loop(n);
  println!("finished");
  fib_while(n);
  println!("finished");
  fib_for(n);
  println!("finished");
}