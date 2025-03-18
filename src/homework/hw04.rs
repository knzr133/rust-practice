const HEIGHT: usize = 5; // Висота ромба (має бути непарною)
const WIDTH: usize = 9;  // Ширина ромба

fn main() {
    let mut output = String::new(); // Змінна для формування виводу

    // Верхня частина ромба
    for i in 0..HEIGHT {
        let spaces = HEIGHT - i - 1; // Пробіли перед *
        let stars = 2 * i + 1;        // Кількість зірочок

        output.push_str(&" ".repeat(spaces));
        output.push_str(&"*".repeat(stars));
        output.push('\n');
    }

    // Нижня частина ромба
    for i in (0..HEIGHT - 1).rev() {
        let spaces = HEIGHT - i - 1;
        let stars = 2 * i + 1;

        output.push_str(&" ".repeat(spaces));
        output.push_str(&"*".repeat(stars));
        output.push('\n');
    }

    print!("{}", output); // Один виклик print!
}
