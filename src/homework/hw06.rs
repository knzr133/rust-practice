fn draw_tree(triangles: u32) {
    // Выводим информацию о количестве триугольников
    println!("Рисуем ёлку с {} треугольниками:", triangles);

    // Рисуем каждый треугольник
    (1..=triangles).for_each(|i| {
        let height = i; // Высота текущего треугольника
        (1..=height).for_each(|level| {
            // Рассчитываем количество пробелов и звездочек
            let spaces = (height - level) as usize;
            let stars = (2 * level - 1) as usize;

            // Формируем строку с пробелами и звездочками
            let line = format!("{:width$}{}", "", "*".repeat(stars), width = spaces);
            println!("{}", line); // Выводим текущую строку
        });
    });

    let tree_trunk_width = triangles as usize;
    let spaces = " ".repeat((tree_trunk_width - 1) / 2);
    (0..3).for_each(|_| println!("{}|{}", spaces, "|"));
}

fn main() {
    let triangles = 5;
    println!("Начинаем рисовать ёлку...");
    draw_tree(triangles);
    println!("Ёлка нарисована!");
}
