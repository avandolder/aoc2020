use std::error::Error;
use std::io::Read;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn cardinal(&self) -> (i32, i32) {
        match self {
            North => (0, 1),
            South => (0, -1),
            East => (1, 0),
            West => (-1, 0),
        }
    }

    fn forward(&self, value: i32, (x, y): (i32, i32)) -> (i32, i32) {
        let (dx, dy) = self.cardinal();
        (x + dx * value, y + dy * value)
    }

    fn turn(self, value: i32) -> Self {
        (0..value).fold(self, |d, _| d.turn_left())
    }

    fn turn_left(self) -> Self {
        match self {
            North => West,
            West => South,
            South => East,
            East => North,
        }
    }
}

use Direction::*;

fn part1(directions: &[(char, i32)]) -> i32 {
    let ((x, y), _) =
        directions.iter().fold(
            ((0, 0), East),
            |((x, y), d), (action, value)| match action {
                'N' => (North.forward(*value, (x, y)), d),
                'S' => (South.forward(*value, (x, y)), d),
                'E' => (East.forward(*value, (x, y)), d),
                'W' => (West.forward(*value, (x, y)), d),
                'L' => ((x, y), d.turn(value / 90)),
                'R' => ((x, y), d.turn((-value).rem_euclid(360) / 90)),
                'F' => (d.forward(*value, (x, y)), d),
                _ => panic!(),
            },
        );
    x.abs() + y.abs()
}

fn part2(directions: &[(char, i32)]) -> i32 {
    fn turn(amt: i32, x: i32, y: i32) -> (i32, i32) {
        (0..amt).fold((x, y), |(x, y), _| (-y, x))
    }

    let ((x, y), _) =
        directions
            .iter()
            .fold(((0, 0), (10, 1)), |((x, y), (wx, wy)), (action, value)| {
                dbg!((wx, wy));
                match action {
                    'N' => ((x, y), North.forward(*value, (wx, wy))),
                    'S' => ((x, y), South.forward(*value, (wx, wy))),
                    'E' => ((x, y), East.forward(*value, (wx, wy))),
                    'W' => ((x, y), West.forward(*value, (wx, wy))),
                    'L' => ((x, y), turn(value / 90, wx, wy)),
                    'R' => ((x, y), turn((-value).rem_euclid(360) / 90, wx, wy)),
                    'F' => ((x + wx * value, y + wy * value), (wx, wy)),
                    _ => panic!(),
                }
            });
    x.abs() + y.abs()
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let directions = input
        .lines()
        .map(|l| (l.chars().next().unwrap(), l[1..].parse().unwrap()))
        .collect::<Vec<_>>();

    println!("{}", part1(&directions));
    println!("{}", part2(&directions));

    Ok(())
}
