// chapter 1
fn main() {
    println!("Hello, world!");
}

// chapter 2

fn main() {
    println!("Starting the guessing game");

    let secret_number = rand::thread_rng().gen_range(1..=10);
    loop {
        println!("Enter a input number");

        let mut guess = String::new();
        io::stdin()
            .read_line(&mut guess)
            .expect("failed to read line");
        let guess: u32 = match guess.trim().parse() {
            Ok(num) => num,
            Err(_err) => {
                println!("enter correct input");
                continue;
            }
        };
        println!("Input number is : {guess}");

        match guess.cmp(&secret_number) {
            Ordering::Equal => {
                println!("Both are equal Brilliant!!!");
                break;
            }
            Ordering::Less => println!("Smaller"),
            Ordering::Greater => println!("Bigger"),
        };
    }
}

