/* 
Basic Rust Syntax
Types and Type Inference
Control Flow
User Defined Types
*/

// 5. Types and Values

// 5.1 Hello, World
// fn main() {
//     println!("Hello, 🌍!");
// }

// 5.2 Variables
// fn main() {
//     let /*mut*/ x: i32 = 10;
//     println!("x: {}", x);
//     // immutability example
//     // x = 20;
//     println!("x: {}", x);
// }

// 5.3 Values
// Signed
//  i8, i16, i32, i64, i128, isize
//  -10, 0, 1_000, 123_i64
// Unsigned
//  u8, u16, u32, u64, u128, usize
//  0, 123, 10_u16
// Floating Point
//  f32, f64
//  3.14, -10.0e20, 2_f32
// Unicode Scalar
//  char (32 bits)
//  any unicode
// Booleans (8 bits)
//  bool
//  true, false

// 5.4 Arithmetic
// fn interproduct(a: i32, b: i32, c: i32) -> i32 {
//     return a * b + c / a;
//     // or
//     // a * b + c / a
// }
// fn main() {
//     println!("result: {}", interproduct(120, 100, 248));
// }

// 5.5 Type Inference
// fn takes_u32(a: u32) {
//     println!("u32: {a}");
// }
// fn takes_i8(a: i8) {
//     println!("i8: {a}");
// }
// fn main() {
//     let x = 10;
//     let y = 127;
//     takes_u32(x);
//     takes_i8(y);
// }

// 5.6 Exercise: Fibonacci
// fn fib(n: u128) -> u128 {
//     if n < 2 {
//         n
//     } else {
//         fib(n-1) + fib(n-2)
//     }
// }
// fn main() {
//     let n: u128 = 8;
//     println!("fib({n}) = {}", fib(n));
// }

