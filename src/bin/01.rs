fn fuel(mass: u32) -> u32 {
    mass / 3 - 2
}

fn main() {
    let input = include_str!("01.txt");
    let sum: u32 = input
        .lines()
        .map(|i| i.parse::<u32>().unwrap())
        .map(fuel)
        .sum();
    println!("Sum: {}", sum);
}

#[test]
fn examples() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);
}
