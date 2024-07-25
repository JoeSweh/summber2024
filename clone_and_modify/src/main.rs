fn clone_and_modify(s: &String) -> String {
    // Your code here:
    // Clone the original string
    //let mut cloned_string = s.clone();
    // Append a new word to the cloned string
    //cloned_string.push_str("World!");
    //cloned_string

    let mut result = s.clone();
    result.push_str("world!");
    result

}

fn main() {
    println!("Name: Jose F. Gonzalez Jr.");
    let s = String::from("Hello, ");
    let modified = clone_and_modify(&s);
    println!("Original: {}", s); // Should print: "Original: Hello, "
    println!("Modified: {}", modified); // Should print: "Modified: Hello, World!"
}
