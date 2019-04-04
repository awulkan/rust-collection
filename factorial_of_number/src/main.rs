fn main() {
    println!("{}", factorialize(5));
}

fn factorialize(num: isize) -> isize {
    if num == 0 { return num }
    let mut factorial = 1;
    for n in 1..=num.abs() {
        factorial *= n;
    }
    if num < 0 {
        return factorial * -1
    } else {
        return factorial
    }
}
