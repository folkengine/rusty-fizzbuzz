
fn is_fizz(x: u32) -> bool {
    (x % 3) == 0
}

fn is_buzz(x: u32) -> bool {
    (x % 5) == 0
}

fn is_fizzbuzz(x: u32) -> bool {
    is_fizz(x) && is_buzz(x)
}

fn fb_string(x: u32) -> String {
    if is_fizzbuzz(x) {
        return "FizzBuzz!".to_string();
    }
    if is_fizz(x) {
        return "Fizz".to_string();
    }
    if is_buzz(x) {
        return "Buzz".to_string();
    }
    return x.to_string();
}

fn fizzy_print(x: u32) {
    println!("{}", fb_string(x));
}

fn main() {
    for i in 1..101 {
        fizzy_print(i);
    }
}

#[test]
fn is_fizz_test() {
    let fizzies = [3, 6, 9, 12, 300, 60];

    for x in fizzies.iter() {
        assert!(is_fizz(*x));
    }
}

#[test]
fn is_buzz_test() {

    let buzzies = [5, 10, 15, 20];

    for x in buzzies.iter() {
        assert!(is_buzz(*x));
    }

    assert!(!is_buzz(4))
}

#[test]
fn is_fizzbuzz_test() {
    let fizzbuzzies = [15, 30, 45, 60];

    for x in fizzbuzzies.iter() {
        assert!(is_fizzbuzz(*x));
    }
}
