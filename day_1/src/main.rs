/* 
Basic Rust Syntax
Types and Type Inference
Control Flow
User Defined Types
*/

/*
MORNING
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

// 6. Control Flow Basics

// 6.1 Blocks and Scopes
// fn main() {
//      let z = 13;
//      let x = {
//         let y = 10;
//         dbg!(y);
//         z - y
//      };
//      dbg!(x);
//     // dbg!(y);
// }

// 6.2 if Expressions
// fn main() {
//     let x = 101;
//     if x == 0 {
//         println!("zero!");
//     } else if x < 100 {
//         println!("biggish");
//     } else {
//         println!("yuge");
//     }
// }
// fn main() {
//     let x = 21;
//     let size = if x < 20 {"small"} else {"big"};
//     println!("{x} is {size}");
// }

// 6.3 match Expressions
// fn main() {
//     let val = 1;
//     match val {
//         1 => println!("one"),
//         10 => println!("ten"),
//         100 => println!("one hundred"),
//         _ => {
//             println!("something else");
//         }
//     }
// }
// fn main() {
//     let flag = 4;
//     let val = match flag {
//         1 => true,
//         0 => false,
//         _ => true
//     };
//     println!("The value of {flag} is {val}.");
// }

// 6.4 Loops
    // while loop
    // fn main() {
    //     let mut x = 200;
    //     while x >= 10 {
    //         x -= 10;
    //         dbg!(x);
    //     }
    // }
    // for loop
    // fn main() {
    //     for x in 0..5 {
    //         dbg!(x);
    //     }
    //     for elem in [2, 4, 8, 16, 32] {
    //         dbg!(elem);
    //     }
    // }
    // standard loop
    // fn main() {
    //     let mut i = 0;
    //     loop {
    //         i += 1;
    //         dbg!(i);
    //         if i >= 20 {
    //             break;
    //         }
    //     }
    // }

// 6.5 break and continue
// fn main() {
//     let mut i = 0;
//     loop {
//         dbg!(i);
//         i += 1;
//         if i > 5 {
//             break;
//         }
//         if i % 2 == 0 {
//             continue;
//         }
//     }
// }
// labels
// fn main() {
//     let s = [[5, 6 , 7], [8, 9, 10], [11, 12, 13]];
//     let mut elements_searched = 1;
//     let target_value = 5;
//     'outer: for i in 0..=2 {
//         for j in 0..=2 {
//             elements_searched += 1;
//             if s[i][j] == target_value {
//                 println!("target_value found at location {},{}", i, j);
//                 break 'outer;
//             }
//         }
//     }
//     dbg!(elements_searched);
// }

// 6.6 Functions
// fn gcd(a: u32, b: u32) -> u32 {
//     if b > 0 {
//         dbg!(a, b);
//         gcd(b, a % b)
//     } else {
//         a
//     }
// }
// fn main() {
//     dbg!(gcd(1024, 32));
// }

// 6.7 Macros
// fn factorial(n: u32) -> u32 {
//     let mut product = 1;
//     for i in 1..=n {
//         product *= dbg!(i);
//     }
//     product
// }
// fn fizzbuzz (n: u32) -> u32 {
//     todo!();
//     // panic!();
// }
// fn main() {
//     let n = 4;
//     println!("{n}! = {}", factorial(n));
//     // fizzbuzz(1);
// }
// some other macros are assert!, unreachable!, eprintln! & more

// 6.8 Exercise: Collatz Sequence
// fn collatz_length(mut n: i32) -> u32 {
//     let mut len = 1;
//     while n > 1 {
//         n = if n % 2 == 0 { n / 2 } else { 3 * n + 1 };
//         len += 1;
//     }
//     len
// }
// fn main() {
//     let n = 1113;
//     println!("Length: {}", collatz_length(n));
// }

/*
AFTERNOON
*/

// 8. Tuples and Arrays

// 8.1 Arrays
fn main() {
    let x = 3;
    let mut a: [i8; 5] = [5, 4, 3, 2, 1];
    a[x] = 9;
    println!("a: {a:?}");
    println!("a[{}]: {:?}", x, a[x]);
}
