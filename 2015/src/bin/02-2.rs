use std::cmp::min;

fn main() {
    let sum: u32 = include_str!("02.txt")
        .lines()
        .map(|line| {
            let v = line
                .split("x")
                .map(|i| i.parse::<u32>().unwrap())
                .collect::<Vec<u32>>();
            let x = v[0];
            let y = v[1];
            let z = v[2];

            let a = x + y;
            let b = y + z;
            let c = x + z;

            let small = min(min(a, b), c);

            (x * y * z) + small * 2
        })
        .sum();
    dbg!(sum);
}
