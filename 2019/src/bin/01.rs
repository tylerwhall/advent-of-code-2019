use advent_of_code_2019::fuel;

fn main() {
    let input = include_str!("01.txt");
    let sum: i32 = input
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .map(fuel)
        .sum();
    assert_eq!(sum, 3563458); // Determined after the first run
    println!("Sum: {}", sum);
}
