fn main() {
    let str: &str = "Hello World";
    let mut s: String = String::from("Hello World");
  
    let slice = &s[..6];
    // slice.len();
  
    s.push('1');
    s.push_str("Added string");
  
    s = s.replace("Hello", "Bye");
  
    println!("{}", s);
  }
  