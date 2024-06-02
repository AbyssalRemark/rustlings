// iterators2.rs
//
// In this exercise, you'll learn some of the unique advantages that iterators
// can offer. Follow the steps to complete the exercise.
//
// Execute `rustlings hint iterators2` or use the `hint` watch subcommand for a
// hint.

// I AM DONE

// Step 1.
// Complete the `capitalize_first` function.
// "hello" -> "Hello"
pub fn capitalize_first(input: &str) -> String {
    let mut c = input.chars();
    match c.next() {
        None => String::new(),
        Some(first) => first.to_uppercase().chain(c).collect(),
        //I would have NEVER guessed that. this feels like a wrong an improper 
        //use of itterators. If there is a reason it needs to be this way I 
        //assume its ebcause of how &str needs to be handled. I would take 
        //null terminated arrays over this repeated fucntion call chaining and
        //mutation any day. 
    }
}

// Step 2.
// Apply the `capitalize_first` function to a slice of string slices.
// Return a vector of strings.
// ["hello", "world"] -> ["Hello", "World"]
pub fn capitalize_words_vector(words: &[&str]) -> Vec<String> {
    //vec![]
    //no? this is itterators.. whatever..
    
    let mut ret = Vec::<String>::new();
    for string in words {
        ret.push(capitalize_first(string));
    }
    ret
    
    //So the intended impl seems to be
    //words.iter().map(|&s| capitalize_first(s)).collect()
    //which. hi closures. how ya doing? 

}

// Step 3.
// Apply the `capitalize_first` function again to a slice of string slices.
// Return a single string.
// ["hello", " ", "world"] -> "Hello World"
pub fn capitalize_words_string(words: &[&str]) -> String {
    let mut ret = String::new();

    for string in words {
        ret.push_str(&capitalize_first(string));
    }
    ret
    //way more clear to me what its doing but hey.. people like there one line
    //things a lot. So, no big deal. just basically the same assuming map works
    //the way I think it does. 
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_success() {
        assert_eq!(capitalize_first("hello"), "Hello");
    }

    #[test]
    fn test_empty() {
        assert_eq!(capitalize_first(""), "");
    }

    #[test]
    fn test_iterate_string_vec() {
        let words = vec!["hello", "world"];
        assert_eq!(capitalize_words_vector(&words), ["Hello", "World"]);
    }

    #[test]
    fn test_iterate_into_string() {
        let words = vec!["hello", " ", "world"];
        assert_eq!(capitalize_words_string(&words), "Hello World");
    }
}
