use aoc::*;
#[test]
fn sonar_sweep() {
    let mut t = vec![199, 200, 208, 210, 200, 207, 240, 269, 260, 263];

    assert_eq!(aoc::sonar_sweep(&mut t, 1), 7);
    assert_eq!(aoc::sonar_sweep(&mut t, 3), 5);
}

#[test]
fn dive() {
    let t = vec![
        Move {
            dir: Direction::FORWARD,
            dist: 5,
        },
        Move {
            dir: Direction::DOWN,
            dist: 5,
        },
        Move {
            dir: Direction::FORWARD,
            dist: 8,
        },
        Move {
            dir: Direction::UP,
            dist: 3,
        },
        Move {
            dir: Direction::DOWN,
            dist: 8,
        },
        Move {
            dir: Direction::FORWARD,
            dist: 2,
        },
    ];

    let orig = Pos { x: 0, y: 0, aim: 0 };
    let dest = aoc::dive(orig.clone(), t.clone());
    let dest_aim = aoc::dive_aim(orig, t.clone());
    assert_eq!(dest.x, 15);
    assert_eq!(dest.y, 10);

    assert_eq!(dest_aim.x, 15);
    assert_eq!(dest_aim.y, 60);
}
