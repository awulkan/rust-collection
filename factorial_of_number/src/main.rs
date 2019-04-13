// Task (from FreeCodeCamp).
// Return the factorial of the provided integer.
// If the integer is represented with the letter n, a factorial is the product of all positive integers less than or equal to n.
// Factorials are often represented with the shorthand notation n!
// For example: 5! = 1 * 2 * 3 * 4 * 5 = 120.
// Only integers greater than or equal to zero will be supplied to the function.

fn main() {
    assert_eq!(factorialize(0), 1);
    assert_eq!(factorialize(5), 120);
    assert_eq!(factorialize(10), 3628800);
    assert_eq!(factorialize(20), 2432902008176640000);
    // assert_eq!(factorialize_with_negative(-5), -120);
}

fn factorialize(num: isize) -> isize {
    if num < 2 {
        1
    } else {
        num * factorialize(num - 1)
    }
}

// fn factorialize_with_negative(num: isize) -> isize {
//     let mut factorial = 1;
//     if num == 0 { return factorial }
//     for n in 1..=num.abs() {
//         factorial *= n;
//     }
//     if num < 0 {
//         return factorial * -1
//     } else {
//         return factorial
//     }
// }
