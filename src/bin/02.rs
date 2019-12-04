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

fn main() {
    let mut v = parse(include_str!("02.txt"));
    v[1] = 12;
    v[2] = 2;
    execute(&mut v);
    dbg!(&v);
    dbg!(v[0]);
}
