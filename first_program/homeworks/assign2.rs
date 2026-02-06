fn is_even(n: i32) -> bool {
    n % 2 == 0
}

fn main() {
    let numbers: [i32; 10] = [3, 5, 8, 15, 22, 9, 10, 7, 30, 4];

    for i in 0..numbers.len() {
        let n = numbers[i];

        if n % 3 == 0 && n % 5 == 0 {
            println!("{}: FizzBuzz", n);
        } else if n % 3 == 0 {
            println!("{}: Fizz", n);
        } else if n % 5 == 0 {
            println!("{}: Buzz", n);
        } else {
            if is_even(n) {
                println!("{}: Even", n);
            } else {
                println!("{}: Odd", n);
            }
        }
    }

    let mut index = 0;
    let mut sum = 0;

    while index < numbers.len() {
        sum = sum + numbers[index];
        index = index + 1;
    }

    println!("Sum: {}", sum);

    let mut largest = numbers[0];
    let mut i = 1;

    loop {
        if i >= numbers.len() {
            break;
        }

        if numbers[i] > largest {
            largest = numbers[i];
        }

        i = i + 1;
    }

    println!("Largest: {}", largest);
}
