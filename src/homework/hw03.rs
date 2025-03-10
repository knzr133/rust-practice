const WIDTH: usize = 10;  // Ширина конверта
const HEIGHT: usize = 5;  // Висота конверта
fn main() {
    let mut output = String::new();

    // Верхня лінія
    output.push('+');
    output.push_str(&"-".repeat(WIDTH));
    output.push('+');
    output.push('\n');

    // Діагональні лінії
    for i in 0..HEIGHT {
        output.push('|');
        output.push_str(&" ".repeat(i));
        output.push('\\');
        output.push_str(&" ".repeat(WIDTH - 2 * i - 2));
        output.push('/');
        output.push_str(&" ".repeat(i));
        output.push('|');
        output.push('\n');
    }

    // Горизонтальна середня лінія
    output.push('|');
    output.push_str(&"-".repeat(WIDTH));
    output.push('|');
    output.push('\n');

    // Нижня лінія
    output.push('+');
    output.push_str(&"-".repeat(WIDTH));
    output.push('+');

    println!("{}", output);
}
