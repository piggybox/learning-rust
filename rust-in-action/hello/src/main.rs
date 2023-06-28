use std::io;

fn main() {
    let mut nth = String::new();

    io::stdin()
        .read_line(&mut nth)
        .expect("Failed to read line");

    let mut nth: u32 = nth.trim().parse().expect("Please type positive number!");

    println!("Your input is: {}", nth);

    if nth == 0 {
        println!("{} th number of fibonacci is: {}", nth, 0);
    } else if nth == 1 {
        println!("{} th number of fibonacci is: {}", nth, 1);
    } else {
        let mut _fibo1 = 0;
        let mut _fibo2 = 1;
        while nth > 1 {
            let next: u32 = _fibo1 + _fibo2;
            _fibo1 = _fibo2;
            _fibo2 = next;
            nth -= 1;
        }

        println!("{} th number of fibonacci is: {}", nth, _fibo2);
    };

    struct Number {
        odd: bool,
        value: i32,
    }

    let n = Number {
        odd: true,
        value: 51,
    };
    let _m = n; // `n` is moved into `m`
    let _o = n; // error: use of moved value: `n`
}
