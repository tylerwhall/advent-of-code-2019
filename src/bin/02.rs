use std::convert::TryFrom;

fn parse(input: &str) -> Vec<i32> {
    input
        .split(",")
        .map(|i| i.parse::<i32>().unwrap())
        .collect()
}

#[test]
fn test_parse() {
    assert_eq!(
        parse("1,9,10,3,2,3,11,0,99,30,40,50"),
        vec![1, 9, 10, 3, 2, 3, 11, 0, 99, 30, 40, 50]
    )
}

fn get_ptr(mem: &mut [i32], i: usize) -> usize {
    usize::try_from(mem[i]).unwrap()
}

fn execute(mem: &mut [i32]) {
    let mut pc = 0;

    // Panics at end of input
    loop {
        let opc = mem[pc];
        if opc == 99 {
            return;
        }
        let op1 = mem[get_ptr(mem, pc + 1)];
        let op2 = mem[get_ptr(mem, pc + 2)];
        mem[get_ptr(mem, pc + 3)] = match opc {
            1 => op1 + op2,
            2 => op1 * op2,
            _ => panic!("invalid opcode {} at {}", opc, pc),
        };
        pc += 4;
    }
}

#[test]
fn examples() {
    let mut v = vec![1, 0, 0, 0, 99];
    execute(&mut v);
    assert_eq!(v, vec![2, 0, 0, 0, 99]);

    let mut v = vec![2, 3, 0, 3, 99];
    execute(&mut v);
    assert_eq!(v, vec![2, 3, 0, 6, 99]);

    let mut v = vec![2, 4, 4, 5, 99, 0];
    execute(&mut v);
    assert_eq!(v, vec![2, 4, 4, 5, 99, 9801]);

    let mut v = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
    execute(&mut v);
    assert_eq!(v, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
}

fn run_with_params(x: i32, y: i32) -> i32 {
    // Whee parsing every time
    let mut v = parse(include_str!("02.txt"));
    v[1] = x;
    v[2] = y;
    execute(&mut v);
    v[0]
}

fn main() {
    dbg!(run_with_params(12, 2));

    for x in 0..=99 {
        for y in 0..=99 {
            if run_with_params(x, y) == 19690720 {
                println!("{:02}{:02}", x, y);
                return;
            }
        }
    }
    println!("not found");
}
