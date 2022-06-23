// #[derive(Debug)]
// struct Fibonacci {
//     a: u32,
//     b: u32,
// }

// impl Iterator for Fibonacci {
//     type Item = u32;

// // Trả về số fibonaci tiếp theo dựa trên kiểu dữ liệu struct Fibonacci
//     fn next(&mut self) -> Option<u32> {
//         let new_next = self.b + self.a;
//         self.b = self.a;
//         self.a = new_next;
//         Some(self.b)
//     }
// }

// // Khởi tạo ban đầu cho Fibonaci: 0, 1
// fn fibonacci_numbers() -> Fibonacci {
//     Fibonacci { a: 1, b: 0 }

// }

// fn main() {
// //     Vì struct Fibonacci có implement trait Iterator của Rust nên 
// // có thể dùng câu lệnh for dc
// // Câu lệnh for bản chất sẽ chuyển qua trait Iterator nên instance của
// // struct Fibonacci có thể duyệt được, 
// // Mỗi lần duyệt sẽ tự động chạy function signature next() trên
// // Nên cần implement hàm next() cho struct Fiboncci.
//     for number in fibonacci_numbers() {
//         println!("{}", number);
//     }
// }

// Bài 2: Lifetime
// Yêu cầu: Sửa lỗi Lifetime 

// use std::fmt;
// struct StrDisplayable<'a>(Vec<&'a str>);

// impl fmt::Display for StrDisplayable<'_> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for v in &self.0 {
//             write!(f, "\n{}", v)?;
//         }
//         Ok(())
//     }
// }

// fn main() {
//         let vec: Vec<&str> = vec!["a","bc","def"];
//         let vec_Foo = StrDisplayable(vec);
//         println!("{}",vec_Foo);
// }