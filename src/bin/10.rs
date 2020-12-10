use std::error::Error;
use std::io::Read;

fn part1(data: &[i64]) -> i64 {
    let (diff1, diff3) =
        data.iter()
            .zip(data[1..].iter())
            .fold((0, 0), |(diff1, diff3), (m, n)| match n - m {
                1 => (diff1 + 1, diff3),
                3 => (diff1, diff3 + 1),
                _ => (diff1, diff3),
            });

    diff1 * diff3
}

fn part2(data: &[i64]) -> i64 {
    data.iter().enumerate().skip(1).rev().fold((1, 0, 0),
        |(i, j, k), (p, n)| (
            j + i,
            if p >= 2 && n - data[p - 2] <= 3 { k + i } else { k },
            if p >= 3 && n - data[p - 3] <= 3 { i } else { 0 },
        )).0
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin().read_to_string(&mut input)?;

    let mut data: Vec<i64> = input.lines().flat_map(str::parse).collect();
    data.push(0);
    data.sort();
    data.push(data[data.len() - 1] + 3);

    println!("{}", part1(&data));
    println!("{}", part2(&data));

    Ok(())
}
