
struct ModGuard {              
    val: u32,              
    display: &'static str           
}

fn get_fizzbuzz_guards() -> Vec<ModGuard> {
    vec![
        ModGuard{val: 3, display: "Fizz"}, 
        ModGuard{val: 5, display: "Buzz"}]
}

fn generate_str(x: u32) -> String {
    let v = get_fizzbuzz_guards();
    let mut s = String::from("");
                
    for i in 0..v.len() {
        if x % &v[i].val == 0 {
            s.push_str(v[i].display);
        }
    }
    if s.len() == 0 {
        x.to_string()
    } else {
        s.to_string()
    }
}

fn doit()  {
    for i in 1..101 {
        println!("{}", generate_str(i));
    }
}

fn main() {
    doit() ;
}

#[test]
fn is_fizz_test() {
    let fizzies = [3, 6, 9, 12];

    for x in fizzies.iter() {
        assert_eq!(generate_str(*x), "Fizz");
    }
}

#[test]
fn is_buzz_test() {

    let buzzies = [5, 10, 20];

    for x in buzzies.iter() {
        assert_eq!(generate_str(*x), "Buzz");
    }

}

#[test]
fn is_fizzbuzz_test() {
    let fizzbuzzies = [15, 30, 45, 60];

    for x in fizzbuzzies.iter() {
        assert_eq!(generate_str(*x), "FizzBuzz");
    }
}
