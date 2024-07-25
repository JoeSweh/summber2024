#[allow(unused_variables, unused_mut)]
fn sum(total: &mut i32, low: i32, high: i32) {
    // Initialize total to 0
    *total = 0;
    // Iterate from low to high (inclusive) and add each number to total
    for i in low..=high {
        *total += i;
    }
}

fn main() {
    println!("Name: Jose F. Gonzalez Jr.");
    let mut total = 0;
    sum(&mut total, 0, 100);
    println!("Total: {}", total); // Should print: "Total: 5050"
}