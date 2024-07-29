// Example  
// string slice - is a reference to part of a String, and it looks like this 

/*  
  write a function that takes a string of words separated by spaces and returns the first word
  it finds in that string. If the function doesn't find a space in the string. If the function doesn't 
  find a space in the string, the whole string must be one word, so the entire string should be returned 
 */   

// as bytes method converts the String to an array
// iter is a method that returns each element in a collection
// enumerate wraps the result of iter and returns each element as part of a tuple instead

fn first_word(s: &String) -> usize { 
    let bytes = s.as_bytes(); 

    for (i, &item) in bytes.iter().enumerate() { 
        if item == b' ' { 
            return i;
        }
    } 

    s.len()
}

pub fn main() { 
    let mut s = String::from("hello world");
    let word = first_word(&s); 

    s.clear();
}








