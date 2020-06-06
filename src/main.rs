/*
 * Convert strings to pig latin. The first consonant of each word is moved to the end of the word
 * and “ay” is added, so “first” becomes “irst-fay.” Words that start with a vowel have “hay” added
 * to the end instead (“apple” becomes “apple-hay”). Keep in mind the details about UTF-8 encoding!
 */
use pig_latin::pig_latenize;

fn main() {
    println!("{}", pig_latenize("Hello again, out there wonderful world"));
    println!(
        "{}",
        pig_latenize("Today is a fantastic day, and I am so happy!")
    );
}
