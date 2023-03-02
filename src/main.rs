use std::io;

fn main() {
    println!("Enter a sequence of integers (one per line), then press Ctrl-D to sort:");

    let mut numbers = Vec::new();
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        match line {
            Ok(line) => {
                match line.trim().parse() {
                    Ok(number) => {
                        numbers.push(number);
                    },
                    Err(_) => {
                        eprintln!("Invalid number: {}", line.trim());
                    }
                }
            },
            Err(_) => {
                eprintln!("Failed to read input line");
            }
        }
    }

    numbers.sort();

    println!("Sorted sequence:");
    for number in numbers {
        println!("{}", number);
    }
}

