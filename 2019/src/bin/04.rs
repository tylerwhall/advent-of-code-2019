fn digits(i: u32) -> [u32; 6] {
    [
        i / 100000 % 10,
        i / 10000 % 10,
        i / 1000 % 10,
        i / 100 % 10,
        i / 10 % 10,
        i / 1 % 10,
    ]
}

type Combo = [u32; 6];

#[test]
fn test_digits() {
    assert_eq!(digits(123456), [1, 2, 3, 4, 5, 6]);
}

fn incrementing(c: &Combo) -> bool {
    let mut start = 0;

    for i in c.iter() {
        if *i < start {
            return false;
        }
        start = *i;
    }
    true
}

#[test]
fn test_incrementing() {
    assert!(incrementing(&[1, 2, 3, 4, 5, 6]));
    assert!(incrementing(&[1, 2, 3, 4, 5, 5]));
    assert!(!incrementing(&[1, 2, 3, 4, 5, 4]));
}

fn adjacent(c: &Combo) -> bool {
    for w in c.windows(2) {
        if w[0] == w[1] {
            return true;
        }
    }
    false
}

#[test]
fn test_adjacent() {
    assert!(!adjacent(&[1, 2, 3, 4, 5, 6]));
    assert!(adjacent(&[1, 2, 3, 4, 6, 6]));
}

fn not_3_adjacent(c: &Combo) -> bool {
    let mut last = c[0];
    let mut run_count = 1;

    for i in 1..c.len() {
        if c[i] == last {
            run_count += 1;
        } else {
            if run_count == 2 {
                return true;
            }
            run_count = 1;
            last = c[i];
        }
    }
    if run_count == 2 {
        return true;
    }
    false
}

#[test]
fn test_not_3() {
    assert!(not_3_adjacent(&[1, 1, 2, 2, 3, 3]));
    assert!(!not_3_adjacent(&[1, 2, 3, 4, 4, 4]));
    assert!(not_3_adjacent(&[1, 1, 1, 1, 2, 2]));
    assert!(!not_3_adjacent(&[6, 7, 8, 8, 8, 9]));
}

fn main() {
    let count = (264793..=803935)
        .map(digits)
        .filter(incrementing)
        .filter(adjacent)
        .count();
    dbg!(count);

    let count = (264793..=803935)
        .map(digits)
        .filter(incrementing)
        .filter(not_3_adjacent)
        .inspect(|i| {
            dbg!(i);
        })
        .count();
    dbg!(count);
}
