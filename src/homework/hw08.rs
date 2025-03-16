pub fn is_prime(n: u32) -> bool {
    if n < 2 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u32) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

fn main() {
    let test_numbers = [1, 2, 3, 4, 5, 16, 17, 19, 20, 23, 29, 30];
    for &num in &test_numbers {
        println!("{} is prime: {}", num, is_prime(num));
    }
}
