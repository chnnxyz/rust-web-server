pub fn iterate_on_words(string: &str, sep: char) -> Option<(&str, &str)>{
    for (i, c) in string.chars().enumerate() {
        if c == sep || c =='\r'{
            return Some((&string[..i], &string[i+1..]))
        }
    }
    None 
}