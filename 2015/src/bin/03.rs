fn unique(mut visited: Vec<(isize, isize)>, i: impl Iterator<Item = u8>) -> Vec<(isize, isize)> {
    let mut pos = (0, 0);

    for dir in i {
        match dir {
            b'>' => pos.0 += 1,
            b'<' => pos.0 -= 1,
            b'^' => pos.1 += 1,
            b'v' => pos.1 -= 1,
            _ => panic!("invalid input"),
        }

        // yay linear search
        if !visited.contains(&pos) {
            visited.push(pos);
        }
    }
    visited
}

fn main() {
    let input = include_bytes!("03.txt");
    let santa = unique(Vec::new(), input.iter().cloned()).len();
    dbg!(santa);

    let santa1 = unique(Vec::new(), input.iter().cloned().step_by(2));
    let santa2 = unique(santa1, input.iter().cloned().skip(1).step_by(2)).len();
    dbg!(santa2);
}
