use anyhow::Error;
use std::str::FromStr;

pub fn solve(input: String) {
    let games = match input.lines().map(|s| s.parse::<Game>()).collect::<Result<Vec<_>, _>>() {
        Ok(v) => v,
        Err(e) => panic!("Couldn't parse input file: {:?}", e)
    };

    println!("{}", games.iter().filter_map(part1).sum::<usize>());
    println!("{}", games.iter().map(part2).sum::<usize>());
}

fn part1(game: &Game) -> Option<usize> {
    if game.cube_sets.iter().all(|s| s.red <= 12 && s.green <= 13 && s.blue <= 14) {
        Some(game.id)
    } else {
        None
    }
}

fn part2(game: &Game) -> usize {
    let (minr, ming, minb) = game.cube_sets.iter()
        .map(|c| (c.red, c.green, c.blue))
        .reduce(|(accr, accg, accb), (r, g, b)| (accr.max(r), accg.max(g), accb.max(b)))
        .unwrap();

    minr * ming * minb
}

struct CubeSet {
    red: usize,
    green: usize,
    blue: usize
}

impl FromStr for CubeSet {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut result = CubeSet { red: 0, green: 0, blue: 0 };

        s.split(", ")
            .map(|c| c.split_once(" ")
                .ok_or(Error::msg("Invalid colour format"))
                .and_then(|i| Ok((i.1, i.0.parse::<usize>().map_err(Error::new)?))
                ))
            .collect::<Result<Vec<_>, _>>()?
            .iter()
            .map(|(c, v)| match *c {
                "red" => { result.red = *v; Ok(()) },
                "green" => { result.green = *v; Ok(()) },
                "blue" => { result.blue = *v; Ok(()) },
                c => Err(Error::msg(format!("Unknown color: {c}")))
            }).collect::<Result<Vec<_>, _>>()?;

        Ok(result)
    }
}

struct Game {
    id: usize,
    cube_sets: Vec<CubeSet>
}

impl FromStr for Game {
    type Err = Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let [header, sets]: [&str; 2] = s.split(": ")
            .collect::<Vec<_>>()
            .try_into()
            .map_err(|_| Error::msg("Invalid game format"))?;
        let id = header.split(" ")
            .last()
            .ok_or(Error::msg("Invalid game header format"))?
            .parse::<usize>()?;

        let cube_sets = sets
            .split("; ")
            .map(|s| s.parse::<CubeSet>())
            .collect::<Result<Vec<CubeSet>, _>>()?;

        Ok(Game { id, cube_sets })
    }
}