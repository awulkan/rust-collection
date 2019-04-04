fn main() {
    let text = String::from("!dlrow olleH");
    println!("{}", str_reverse_one(&text));
    println!("{}", str_reverse_two(&text));
}

fn str_reverse_one(text: &str) -> String {
    let mut reversed = String::new();
    for c in text.chars().rev() {
        reversed.push(c);
    }
    reversed
}

fn str_reverse_two(text: &str) -> String {
    text.chars().rev().collect()
}
