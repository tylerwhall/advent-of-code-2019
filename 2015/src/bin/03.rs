fn main() {
    let mut pos = (0, 0);
    let mut visited = Vec::new();

    for dir in include_bytes!("03.txt").iter() {
        match dir {
            b'>' => pos.0 += 1,
            b'<' => pos.0 -= 1,
            b'^' => pos.1 += 1,
            b'v' => pos.1 -= 1,
            _ => panic!("invalid input"),
        }

        if !visited.contains(&pos) {
            visited.push(pos);
        }
    }

    dbg!(visited.len());
}
