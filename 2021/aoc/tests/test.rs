#[test]
fn sonar_sweep() {
    let mut t = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    assert_eq!(aoc::do_sonar_sweep(&mut t, 1), 7);
    assert_eq!(aoc::do_sonar_sweep(&mut t, 3), 5);
}
