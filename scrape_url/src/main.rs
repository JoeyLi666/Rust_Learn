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



// fn fib_loop(n: u8) {
//   let mut a = 1;
//   let mut b = 1;
//   let mut i = 2u8;
  
//   loop {
//       let c = a + b;
//       a = b;
//       b = c;
//       i += 1;
      
//       println!("next val is {}", b);
      
//       if i >= n {
//           break;
//       }
//   }
// }

// fn fib_while(n: u8) {
//   let (mut a, mut b, mut i) = (1, 1, 2);
  
//   while i < n {
//       let c = a + b;
//       a = b;
//       b = c;
//       i += 1;
      
//       println!("next val is {}", b);
//   }
// }

// // Rust 的 for 循环可以用于任何实现了 IntoIterator trait 的数据结构
// fn fib_for(n: u8) {
//   let (mut a, mut b) = (1, 1);
//   // 2 =< i < n
//   for _i in 2..n {
//       let c = a + b;
//       a = b;
//       b = c;
//       println!("next val is {}", b);
//   }
// }

// fn main() {
//   let n = 10;
//   fib_loop(n);
//   println!("finished");
//   fib_while(n);
//   println!("finished");
//   fib_for(n);
//   println!("finished");
// }


#[derive(Debug)]
enum Gender {
  Unspecified = 0,
  Female = 1,
  Male = 2,
}

#[derive(Debug, Copy, Clone)]
struct UserId(u64);

#[derive(Debug, Copy, Clone)]
struct TopicId(u64);

#[derive(Debug)]
struct User {
  id: UserId,
  name: String,
  gender: Gender,
}

#[derive(Debug)]
struct Topic {
  id: TopicId,
  name: String,
  owner: UserId,
}

// 定义聊天室中可能发生的事件
#[derive(Debug)]
enum Event {
  Join((UserId, TopicId)),
  Leave((UserId, TopicId)),
  Message((UserId, TopicId, String)),
}

fn main() {
    let alice = User { id: UserId(1), name: "Alice".into(), gender: Gender::Female };
    let bob = User { id: UserId(2), name: "Bob".into(), gender: Gender::Male };
    
    let topic = Topic { id: TopicId(1), name: "rust".into(), owner: UserId(1) };
    let event1 = Event::Join((alice.id, topic.id));
    let event2 = Event::Join((bob.id, topic.id));
    let event3 = Event::Message((alice.id, topic.id, "Hello world!".into()));
    
    println!("event1: {:?}, event2: {:?}, event3: {:?}", event1, event2, event3);
}