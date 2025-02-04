fn main() {
    const MAX_NUMBER: u32 = 143;

    let integers: Vec<u32> = prime_boy(MAX_NUMBER);
    let mut integer_string: String = String::from("2");
    for item in integers {
        if item != 2 {
            integer_string = format!("{}, {}", integer_string, item);
        }
    }
    println!("{}", integer_string);
}

fn prime_boy(n: u32) -> Vec<u32> {
    let mut primes: Vec<u32> = vec!();
    let mut integers: Vec<bool> = vec![true; (n + 2).try_into().unwrap()];
    for i in 2..integers.len() - 1 {
        if !integers[i] { continue; }

        for integer_two in i + 1..integers.len() - 1 {
            if integer_two % i == 0 {
                integers[integer_two] = false;
            }
        }
    }
    for i in 2..integers.len() - 1 {
        if !integers[i] { continue; }
        primes.push(i.try_into().unwrap());
    }
    primes
}