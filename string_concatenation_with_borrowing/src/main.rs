fn concat_strings(s1: &String, s2: &String) -> String {
    // Your code here
    format!("{}{}", s1, s2)
}

fn main() {
    println!("Name: Jose F. Gonzalez Jr.");
    let s1 = String::from("Hello, ");
    let s2 = String::from("World!");
    let result = concat_strings(&s1, &s2);
    println!("{}", result); // Should print: "Hello, World!"
}