use advent_of_code_2019::fuel;

fn total_fuel(mut mass: i32) -> i32 {
    let mut total = 0;

    loop {
        mass = fuel(mass);
        if mass < 0 {
            break;
        }
        total += mass;
    }
    total
}

fn main() {
    let input = include_str!("01.txt");
    let sum: i32 = input
        .lines()
        .map(|i| i.parse::<i32>().unwrap())
        .map(total_fuel)
        .sum();
    assert_eq!(sum, 5342292); // Determined after the first run
    println!("Sum: {}", sum);
}

#[test]
fn examples() {
    assert_eq!(total_fuel(14), 2);
    assert_eq!(total_fuel(1969), 966);
    assert_eq!(total_fuel(100756), 50346);
}
