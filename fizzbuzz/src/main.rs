// Task: Play fizzbuzz game (https://en.wikipedia.org/wiki/Fizz_buzz).
// Return the corresponding string (either "Fizz", "Buzz", "FizzBuzz" or a number)
// Your response should be a String.

fn do_fizzbuzz(n: u32) -> String{
    match(n%3, n%5){
        (0, 0) => String::from("FizzBuzz"),
        (0, _) => String::from("Fizz"),
        (_, 0) => String::from("Buzz"),
        (_, _) => n.to_string()
    }
}

fn fizzbuzz(n: u32){
    (1..n).into_iter()
    .map(|i| do_fizzbuzz(i))
    .for_each(|s| println!("{}", s))
}

fn main() {
    fizzbuzz(20);
}
