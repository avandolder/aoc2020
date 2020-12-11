use std::collections::HashMap;
use std::error::Error;
use std::io::Read;
use std::iter::repeat;

use itertools::iproduct;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Seat {
    Full,
    Empty,
}
use Seat::*;
type Seats = HashMap<(i64, i64), Seat>;

fn part1(mut seats: Seats) -> usize {
    loop {
        let next_seats = seats
            .iter()
            .map(|((i, j), seat)| {
                let adj_occ = iproduct!(i - 1..=i + 1, j - 1..=j + 1)
                    .filter(|(m, n)| m != i || n != j)
                    .filter(|(i, j)| matches!(seats.get(&(*i, *j)).unwrap_or(&Empty), Full))
                    .count();
                let seat = match seat {
                    Full if adj_occ >= 4 => Empty,
                    Empty if adj_occ == 0 => Full,
                    seat => *seat,
                };
                ((*i, *j), seat)
            })
            .collect::<Seats>();

        if next_seats == seats {
            break seats.values().filter(|seat| matches!(seat, Full)).count();
        }
        seats = next_seats;
    }
}

fn part2(mut seats: Seats) -> usize {
    fn adj(seats: &Seats, mut itr: impl Iterator<Item = (i64, i64)>) -> i64 {
        matches!(
            itr.find(|(i, j)| seats.contains_key(&(*i, *j)))
                .map(|(i, j)| seats[&(i, j)]),
            Some(Full)
        ) as i64
    }

    let w = seats.keys().max_by_key(|(_, j)| *j).unwrap().1;
    let h = seats.keys().max_by_key(|(i, _)| *i).unwrap().0;

    loop {
        let next_seats = seats
            .iter()
            .map(|((i, j), seat)| {
                let adj_occ = adj(&seats, (0..*i).rev().zip(repeat(*j)))
                    + adj(&seats, (*i + 1..=h).zip(repeat(*j)))
                    + adj(&seats, repeat(*i).zip((0..*j).rev()))
                    + adj(&seats, repeat(*i).zip(*j + 1..=w))
                    + adj(&seats, (0..*i).rev().zip((0..*j).rev()))
                    + adj(&seats, (*i + 1..=h).zip(*j + 1..=w))
                    + adj(&seats, (0..*i).rev().zip(*j + 1..=w))
                    + adj(&seats, (*i + 1..=h).zip((0..*j).rev()));
                let seat = match seat {
                    Full if adj_occ >= 5 => Empty,
                    Empty if adj_occ == 0 => Full,
                    seat => *seat,
                };
                ((*i, *j), seat)
            })
            .collect::<Seats>();

        if next_seats == seats {
            break seats.values().filter(|seat| matches!(seat, Full)).count();
        }
        seats = next_seats;
        // print_seats(&seats);
        // println!("");
    }
}

fn print_seats(seats: &Seats) {
    let w = seats.keys().max_by_key(|(_, j)| *j).unwrap().1;
    let h = seats.keys().max_by_key(|(i, _)| *i).unwrap().0;
    for i in 0..=h {
        for j in 0..=w {
            match seats.get(&(i, j)) {
                Some(Full) => print!("#"),
                Some(Empty) => print!("L"),
                None => print!("."),
            }
        }
        println!();
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let seats: Seats = input
        .lines()
        .enumerate()
        .flat_map(|(i, row)| {
            row.chars()
                .enumerate()
                .filter_map(move |(j, kind)| match kind {
                    'L' => Some(((i as i64, j as i64), Seat::Full)),
                    '.' => None,
                    _ => panic!(),
                })
        })
        .collect();

    println!("{}", part1(seats.clone()));
    println!("{}", part2(seats));

    Ok(())
}
