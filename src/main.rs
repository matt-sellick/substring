use std::io;
use std::time::Instant;

fn get_input() -> String {
    let mut input_line = String::new();
    io::stdin()
        .read_line(&mut input_line)
        .expect("Error getting input");
    input_line
}

struct Substring {
    length: usize,
    contents: String,
}

// main function for checking longest substring
fn longest_substring(s: &str) {

    // instantiate struct to contain "the longest substring found yet"
    let mut longest = Substring {
        length: 0,
        contents: String::new(),
    };

    // "starting from any given element, how far can you get before a character repeats itself?"
    for (starting_index, _starting_char) in s.char_indices() { // don't need _starting_char, but I don't know how else to set up a for loop for this w/o "char_indices()""
        
        // call helper function and store results in buffer; compare to currently stored max-lengths and update as required until you reach the end
        let (buffer, end_reached) = no_repeats(&s[starting_index..]);
        if longest.length < buffer.length {
            longest.contents = buffer.contents;
            longest.length = buffer.length;
        }
        if end_reached {
            break;
        }
    }

    // print result
    println!("\nThe longest substring is '{}', length {}", longest.contents, longest.length);
}

// helper function for longest_substring
// returns length of longest substring found, the longest substring found, and whether the end of the slice was reached with no matches
fn no_repeats(s: &str) -> (Substring, bool) { 
   
    // returns if the slice is empty
    if s.is_empty() {
        let return_string = Substring {
            length: 0,
            contents: String::from(s),
        };
        return (return_string, true)
    }

    // for every element in slice, checks if it matches any preceding elements
    // if it finds a match, returns the string and its length prior to the match
    for chars in s.char_indices() {
        if chars.0 > 0 { // keeps from checking out of bounds
            if s[.. chars.0].contains(chars.1) { // if the current iterator matches any character preceding it, & remember, upper bound is exclusive
                let return_string = Substring {
                    length: chars.0,
                    contents: String::from(&s[.. chars.0]),
                };
                return (return_string, false);
            }
        }
    }

    // if end is reached and no matches are found
    let return_string = Substring {
        length: s.chars().count(),
        contents: String::from(s),
    };
    (return_string, true)
}

fn main() {
    println!("\nHello, Matt! This is Puzzle #006, 'longest substring'");
    // given a string 's', find the length of the longest substring without repeating characters
    // examples: s = "abcabcbb", s = "bbbbb", s = "pwwkew", s = ""
    // & answers: 3 ("abc"), 1 ("b"), 3 ("wke"), 0

    // get input string
    println!("\nPlease enter the string you want to check");
    let s: String = get_input()
        .trim()
        .parse()
        .expect("Error binding to String 's'");
    println!("You entered: {}", s);

    // set timer and call function
    let now = Instant::now();
    longest_substring(&s);
    let duration = now.elapsed();

    // print elapsed time
    println!("Elapsed time: {:?} \n", duration);
}
