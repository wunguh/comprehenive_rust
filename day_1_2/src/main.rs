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
fn main() {
    let a = 'A';
    let b = 'B';
    let mut r: &char = &a;
    dbg!(r);
    r = &b;
    dbg!(r, a, b);
}