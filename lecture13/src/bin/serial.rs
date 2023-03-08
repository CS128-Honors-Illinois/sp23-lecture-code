fn main() {
    let data = (0..100_000).collect::<Vec<usize>>();
    let total: usize = data
        .into_iter()
        .map(lecture13::expensive_computation)
        .sum();

    println!("Total sum is: {}", total);
}