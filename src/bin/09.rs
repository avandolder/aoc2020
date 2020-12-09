use std::collections::HashSet;
use std::error::Error;
use std::io::Read;

fn part1(data: &[i32]) -> Option<i32> {
    let mut prev = data[..25].iter().collect::<HashSet<_>>();
    for (i, n) in data[25..].iter().enumerate() {
        if !prev.iter().any(|m| prev.contains(&(*n - *m))) {
            return Some(*n);
        }
        prev.remove(&data[i]);
        prev.insert(n);
    }
    None
}

fn part2(data: &Vec<i32>, invalid: i32) -> Option<i32> {
    let mut sum = data[0];
    let (mut i, mut j) = (0, 1);

    for n in data[1..].iter() {
        while sum + *n > invalid {
            sum -= data[i];
            i += 1;
        }

        if sum + *n < invalid {
            j += 1;
            sum += *n;
        } else if sum + *n == invalid {
            j += 1;
            return data[i..=j]
                .iter()
                .min()
                .and_then(|n| data[i..=j].iter().max().map(|m| m + n));
        }
    }

    None
}

fn main() -> Result<(), Box<dyn Error>> {
    let mut input = String::new();
    std::io::stdin()
        .read_to_string(&mut input)
        .expect("Couldn't read from stdin");

    let data: Vec<i32> = input.lines().flat_map(str::parse).collect();

    let invalid = part1(&data).ok_or("no invalid found")?;
    println!("{}", invalid);

    let weakness = part2(&data, invalid).ok_or("no weakness found")?;
    println!("{}", weakness);

    Ok(())
}
