// Function to check if a number is even
fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    ////Assignment 2: Number Analyzer

    // Array of 10 ints
    let numbers: [i32; 10] = [3, 8, 15, 22, 30, 7, 10, 18, 5, 12];

    // For loop to iterate through arr
    for num in numbers {

        // Even or Odd
        if is_even(num) {
            print!("{} is Even -> ", num);
        } else {
            print!("{} is Odd -> ", num);
        }

        // FizzBuzz logic
        let mod3 = num % 3 == 0;
        let mod5 = num % 5 == 0;
        if mod3 {
            print!("Fizz");
        } 
        if mod5 {
            print!("Buzz");
        } 
        println!("");
    }

    // While loop to sum nums
    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        sum += numbers[index];
        index += 1;
    }

    println!("\nSum of all numbers = {}", sum);

    // Loop to find largest num
    let mut largest = numbers[0];

    for num in numbers {
        if num > largest {
            largest = num;
        }
    }

    println!("Largest number = {}", largest);
}