// Функція для зсуву рядка на n позицій
fn rotate(s: String, n: isize) -> String {
    let len = s.len() as isize;

    // Якщо рядок порожній, повертаємо його без змін
    if len == 0 {
        return s;
    }

    // Нормалізуємо значення зсуву, щоб воно знаходилося в межах довжини рядка
    let n = ((n % len) + len) % len;

    // Розбиваємо рядок на дві частини і об'єднуємо їх у зсунутий рядок
    let (left, right) = s.split_at(len as usize - n as usize);
    format!("{}{}", right, left)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test() {
        let s = "abcdefgh".to_string();
        let shifts = [
            (0,  "abcdefgh"),
            (8,  "abcdefgh"),
            (-8, "abcdefgh"),
            (1,  "habcdefg"),
            (2,  "ghabcdef"),
            (10, "ghabcdef"),
            (-1, "bcdefgha"),
            (-2, "cdefghab"),
            (-10,"cdefghab"),
        ];

        shifts.iter().for_each(|(n, exp)| {
            assert_eq!(
                rotate(s.clone(), *n),
                exp.to_string()
            );
        });
    }
}
