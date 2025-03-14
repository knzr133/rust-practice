fn main() {
    let upper_case = "HELLO, WORLD!";
    let lower_case = "hello, world!";

    // Перетворення з верхнього регістру в нижній
    let lower = upper_case.to_lowercase();
    println!("Upper to lower: {}", lower);

    // Перетворення з нижнього регістру в верхній
    let upper = lower_case.to_uppercase();
    println!("Lower to upper: {}", upper);
}
