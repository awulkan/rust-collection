// Task: Find the Longest Word in a String (from FreeCodeCamp).
// Return the length of the longest word in the provided sentence.
// Your response should be a number.

fn main() {
    assert_eq!(longest_word_length(""), 0);
    assert_eq!(longest_word_length("Ferris"), 6);
    assert_eq!(longest_word_length("The quick brown fox jumped over the lazy dog"), 6);
    assert_eq!(longest_word_length("May the force be with you"), 5);
    assert_eq!(longest_word_length("What if we try a super long word such as otorhinolaryngology"), 19);
}

fn longest_word_length(text: &str) -> usize {
    if text.len() == 0 { return 0 }
    let mut largest_value = 0;
    text.split_whitespace().for_each(|x| {
        if x.len() > largest_value {
            largest_value = x.len();
        }
    });
    largest_value
}
