// Using the ? operator on an Option<T> value 

fn last_char_of_first_line(text: &str) -> Option<char> { 
    text.lines().next()?.chars().last()
} 

pub fn check_error() { 
    assert_eq!( 
        last_char_of_first_line("Hello, world\n How are you today"), 
        Some('d')
    ); 

    assert_eq!(last_char_of_first_line(""), None); 
    assert_eq!(last_char_of_first_line("\nhi"), None);
} 

