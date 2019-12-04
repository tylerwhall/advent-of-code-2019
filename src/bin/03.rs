#[derive(Debug, PartialEq, Eq, Clone)]
enum Move {
    X(i32),
    Y(i32),
}
use Move::*;

type Point = (i32, i32);
type Vector = (Point, Move);

fn swap(p: Point) -> Point {
    (p.1, p.0)
}

fn sort(a: i32, b: i32) -> (i32, i32) {
    if a > b {
        (b, a)
    } else {
        (a, b)
    }
}

/// Two segments of same direction. X is the direction of travel
fn colinear(y1: i32, y2: i32, x1: i32, x2: i32, dx1: i32, dx2: i32) -> Option<Point> {
    if y1 != y2 {
        return None;
    }
    let l1 = sort(x1, x1 + dx1);
    let l2 = sort(x2, x2 + dx2);
    let (l1, l2) = if l1.0 <= l2.0 { (l1, l2) } else { (l2, l1) };
    if l1.1 >= l2.0 {
        if (l2.0, y1) == (0, 0) {
            return Some((0, 0)); // Special case for exiting origin in opposite directions since general case below is incorrect
        }
        //Some((l2.0, y1)) // Doesn't account for absolute value from origin
        unimplemented!()
    } else {
        None
    }
}

fn intersects(v1: Vector, v2: Vector) -> Option<Point> {
    let (x, y) = match (v1, v2) {
        ((p1, X(d1)), (p2, Y(d2))) => ((p1, d1), (p2, d2)),
        ((p1, Y(d1)), (p2, X(d2))) => ((p2, d2), (p1, d1)),
        ((p1, X(d1)), (p2, X(d2))) => return colinear(p1.1, p2.1, p1.0, p2.0, d1, d2),
        ((p1, Y(d1)), (p2, Y(d2))) => return colinear(p1.0, p2.0, p1.1, p2.1, d1, d2).map(swap),
    };
    let x1 = (x.0).0;
    let y1 = (y.0).1;
    let (xmin, xmax) = sort(x1, x1 + x.1);
    let (ymin, ymax) = sort(y1, y1 + y.1);
    let xi = (y.0).0;
    let yi = (x.0).1;

    if xi >= xmin && xi <= xmax && yi >= ymin && yi <= ymax {
        Some((xi, yi))
    } else {
        None
    }
}

#[test]
fn test_intersects() {
    assert_eq!(intersects(((1, 0), Y(10)), ((0, 5), X(20))), Some((1, 5)));
    assert_eq!(intersects(((2, 0), Y(10)), ((0, 5), X(1))), None);
    //assert_eq!(intersects(((0, 2), X(10)), ((-5, 2), X(6))), Some((0, 2)));
    assert_eq!(intersects(((2, 2), X(10)), ((-5, 2), X(6))), None);
}

fn segments(moves: impl Iterator<Item = Move>) -> impl Iterator<Item = Vector> {
    moves.scan((0, 0), |pos, m| {
        let orig = *pos;
        *pos = match m {
            X(d) => (pos.0 + d, pos.1),
            Y(d) => (pos.0, pos.1 + d),
        };
        Some((orig, m))
    })
}

#[test]
fn test_segments() {
    assert_eq!(
        segments([X(10), Y(9)].iter().cloned()).collect::<Vec<_>>(),
        vec![((0, 0), X(10)), ((10, 0), Y(9))]
    )
}

fn parse<'a>(s: &'a str) -> impl Iterator<Item = Move> + 'a {
    s.split(",").map(|m| {
        let dist = m[1..].parse::<i32>().unwrap();
        match &m[0..1] {
            "U" => Move::Y(dist),
            "D" => Move::Y(-dist),
            "L" => Move::X(-dist),
            "R" => Move::X(dist),
            _ => panic!("invalid input"),
        }
    })
}

#[test]
fn test_parse() {
    assert_eq!(
        parse("R75,D30,R83,U83,L12,D49,R71,U7,L72").collect::<Vec<_>>(),
        vec![
            X(75),
            Y(-30),
            X(83),
            Y(83),
            X(-12),
            Y(-49),
            X(71),
            Y(7),
            X(-72)
        ]
    );
}

fn mdist(input: &str) -> i32 {
    let mut lines = input.lines();
    let s1 = lines.next().unwrap();
    let s2 = lines.next().unwrap();
    let s1 = segments(parse(s1)).collect::<Vec<_>>();
    let s2 = segments(parse(s2)).collect::<Vec<_>>();

    let mut result = None;
    for s1 in s1.iter() {
        for s2 in s2.iter() {
            if let Some(p) = intersects(s1.clone(), s2.clone()) {
                dbg!(p);
                if p == (0, 0) {
                    continue;
                }
                let mdist = p.0.abs() + p.1.abs();
                dbg!(mdist);
                result = result
                    .take()
                    .map(|res| std::cmp::min(res, mdist))
                    .or(Some(mdist))
            }
        }
    }
    result.unwrap()
}

#[test]
fn examples() {
    assert_eq!(mdist("R8,U5,L5,D3\nU7,R6,D4,L4"), 6);
    assert_eq!(
        mdist("R75,D30,R83,U83,L12,D49,R71,U7,L72\nU62,R66,U55,R34,D71,R55,D58,R83"),
        159
    );
    assert_eq!(
        mdist("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51\nU98,R91,D20,R16,D67,R40,U7,R15,U6,R7"),
        135
    );
}

fn main() {
    let r = mdist(include_str!("03.txt"));
    dbg!(r);
}
