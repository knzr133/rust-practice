use rand::Rng;

// Генерація випадкового вектора
fn gen_random_vector(n: usize) -> Vec<i32> {
    let mut rng = rand::thread_rng();
    (0..n).map(|_| rng.gen_range(10..99)).collect()
}

// Знаходження мінімальної пари сусідніх елементів
fn min_adjacent_sum(data: &[i32]) -> (i32, usize, usize) {
    let mut min_sum = i32::MAX;
    let mut min_indexes = (0, 0);

    for i in 0..data.len() - 1 {
        let sum = data[i] + data[i + 1];
        if sum < min_sum {
            min_sum = sum;
            min_indexes = (i, i + 1);
        }
    }

    (min_sum, min_indexes.0, min_indexes.1)
}

// Виведення результату
fn print_result(data: &[i32]) {
    println!("indexes: {:?}", (0..data.len()).collect::<Vec<_>>());
    println!("data: {:?}", data);
    let (min_sum, idx1, idx2) = min_adjacent_sum(data);
    println!(
        "min adjacent sum={}+{}={} at indexes:{},{}",
        data[idx1], data[idx2], min_sum, idx1, idx2
    );
}

fn main() {
    let data = gen_random_vector(20);

    print_result(&data);
}
