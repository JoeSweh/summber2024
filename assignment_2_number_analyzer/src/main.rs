fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    println!("Name: Jose F. Gonzalez Jr.");
    let numbers = [12, 15, 22, 33, 45, 50, 65, 75, 82, 99];

    for &number in numbers.iter() {
        if number % 3 == 0 && number % 5 == 0 {
            println!("{}: FizzBuzz", number);
        } else if number % 3 == 0 {
            println!("{}: Fizz", number);
        } else if number % 5 == 0 {
            println!("{}: Buzz", number);
        } else if is_even(number) {
            println!("{}: is even", number);
        } else {
            println!("{}: is odd", number);
        }
    }

    
    // Use a while loop to find and print the sum of all numbers
    let mut sum = 0;
    let mut index = 0;
    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }
    println!("Sum of all numbers: {}", sum);

    // Use a loop to find and print the largest number
    let mut largest = numbers[0];
    for &number in numbers.iter() {
        if number > largest {
            largest = number;
        }
    }
    println!("Largest number: {}", largest);
}