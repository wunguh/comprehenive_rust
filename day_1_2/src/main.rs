/*
AFTERNOON
*/

// 8. Tuples and Arrays

// 8.1 Arrays
// fn main() {
//     let mut a: [i8; 5] = [5, 4, 3, 2, 1];
//     a[2] = 0;
//     println!("a: {a:?}");
// }
//out of bounds array index
// fn main() {
//     let mut a: [i8; 5] = [5, 4, 3, 2, 1];
//     a[6] = 0;
//     println!("a: {a:?}");
// }
// fn get_index() -> usize {
//     6
// }
// fn main() {
//     let mut a: [i8; 5] = [5, 4, 3, 2, 1];
//     a[get_index()] = 0;
//     println!("a: {a:?}");
// }

// 8.2 Tuples
// fn main() {
//     let t: (i8, bool) = (7, true);
//     dbg!(t.0);
//     dbg!(t.1);
// }

// 8.3 Array iteration
// fn main() {
//     let primes = [2, 3, 5, 7, 11, 13, 17, 19];
//     for prime in primes {
//         for i in 2..prime {
//             assert_ne!(prime % i, 0);
//         }
//     }
// }

// 8.4 Patterns and Destructuring
// fn check_order(tuple: (i32, i32, i32)) -> bool {
//     let (left, middle, right) = tuple;
//     left < middle && middle < right
// }
// fn main() {
//     let tuple = (1, 3, 5);
//     println!(
//         "{tuple:?}: {}",
//         if check_order(tuple) { "ordered" } else { "unordered" }
//     );
// }

// 8.5 Exercise: Nested Arrays
// fn transpose(matrix: [[i32; 3]; 3]) -> [[i32; 3]; 3] {
//     let mut result = [[0; 3]; 3];
//     for i in 0..3 {
//         for j in 0..3 {
//             result[j][i] = matrix[i][j];
//         }
//     }
//     result
// }
// fn main() {
//     let matrix = [[1, 2, 3], [4, 5, 6], [7, 8, 9]];
//     dbg!(matrix);
//     let transposed = transpose(matrix);
//     dbg!(transposed);
// }

// 9. References

// 9.1 Shared References
// fn main() {
//     let a = 'A';
//     let b = 'B';
//     let mut r: &char = &a;
//     dbg!(r);
//     r = &b;
//     dbg!(r, a, b);
// }

// 9.2 Exclusive References
// fn main() {
//     let mut point = (1, 2);
//     let x_coord = &mut point.0;
//     *x_coord = 20;
//     println!("point: {point:?}");
// }

// 9.3 Slices
// fn main() {
//     let a: [i32; 6] = [10, 20, 30, 40, 50, 60];
//     println!("a: {a:?}");
//     let b: &[i32] = &a[..a.len()-1];
//     println!("b: {b:?}");
//     let c: &[i32] = &b[..b.len()-1];
//     println!("c: {c:?}");
//     let d: &[i32] = &c[..c.len()-1];
//     println!("d: {d:?}");
//     let e: &[i32] = &d[..d.len()-1];
//     println!("e: {e:?}");
//     let f: &[i32] = &e[..e.len()-1];
//     println!("f: {f:?}");
// }

// 9.4 Strings
// fn main() {
//     let s1: &str = "World";
//     println!("s1: {s1}");
//     let mut s2: String = String::from("Hello ");
//     println!("s2: {s2}");
//     s2.push_str(s1);
//     println!("s2: {s2}");
//     let s3: &str = &s2[2..9];
//     println!("s3: {s3}");
// }

// 9.5 Reference Validity
// fn main() {
//     let x_ref = {
//         let x = 10;
//         &x
//     };
//     dbg!(x_ref);
// }

// Exercise: Geometry
fn magnitude(vector: &[f64; 3]) -> f64 {
    let mut mag_squared = 0.0;
    for coord in vector {
        mag_squared += coord.powf(2.0);
    }
    mag_squared.sqrt()
}
fn normalize(vector: &mut [f64; 3]) {
    let mag = magnitude(vector);
    for item in vector {
        *item /= mag;
    }
}
fn main() {
    println!("Magnitude of a unit vector: {}", magnitude(&[0.0, 1.0, 0.0]));
    let mut v = [1.0, 2.0, 9.0];
    println!("Magnitude of {v:?}: {}", magnitude(&v));
    normalize(&mut v);
    println!("Magnitude of {v:?} after normalization: {}", magnitude(&v));
}