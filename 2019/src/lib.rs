pub fn fuel(mass: i32) -> i32 {
    mass / 3 - 2
}

#[test]
fn examples() {
    assert_eq!(fuel(12), 2);
    assert_eq!(fuel(14), 2);
    assert_eq!(fuel(1969), 654);
    assert_eq!(fuel(100756), 33583);
}
