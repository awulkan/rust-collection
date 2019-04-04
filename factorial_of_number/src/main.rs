fn main() {
    println!("Positive factorial: {}", factorialize(5));
    println!("Negative factorial: {}", factorialize(-5));
}

fn factorialize(num: isize) -> isize {
    let mut factorial = 1;
    if num == 0 { return factorial }
    for n in 1..=num.abs() {
        factorial *= n;
    }
    if num < 0 {
        return factorial * -1
    } else {
        return factorial
    }
}
