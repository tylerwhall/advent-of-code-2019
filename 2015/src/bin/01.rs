fn main() {
    let mut floor = 0;
    let mut pos = 1;
    let mut basement_found = false;
    for b in include_bytes!("01.txt").iter() {
        match b {
            b'(' => floor += 1,
            b')' => floor -= 1,
            _ => panic!("invalid input"),
        }
        if !basement_found && floor == -1 {
            println!("Basement char {}", pos);
            basement_found = true;
        }
        pos += 1;
    }

    dbg!(floor);
}
