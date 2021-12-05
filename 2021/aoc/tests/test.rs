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

#[test]
fn binary_diagnostic() {
    let t = vec![
        "00100", "11110", "10110", "10111", "10101", "01111", "00111", "11100", "10000", "11001",
        "00010", "01010",
    ];
}

#[test]
fn transpose() {
    let t = vec![1, 2, 3, 4, 5, 6, 7, 8];
    let mut out = vec![];
    let mut out2 = vec![];
    aoc::transpose(&t, &mut out, 4, 2);
    aoc::transpose(&out, &mut out2, 2, 4);

    println!("{:?}", t);
    println!("{:?}", out);
    println!("{:?}", out2);

    assert_eq!(out[0], 1);
    assert_eq!(out[1], 5);
    assert_eq!(out[2], 2);
    assert_eq!(out[3], 6);
    assert_eq!(out[4], 3);
    assert_eq!(out[5], 7);
    assert_eq!(out[6], 4);
    assert_eq!(out[7], 8);

    for i in 0..out2.len() {
        assert_eq!(t[i], out2[i]);
    }
}
