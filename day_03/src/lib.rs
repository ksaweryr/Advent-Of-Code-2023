use std::collections::HashMap;

pub fn solve(input: String) {
    let map: Vec<Vec<char>> = input.lines().map(|l| l.chars().collect()).collect();
    

    println!("{}", part1(&map));
    println!("{}", part2(&map));
}

fn part1(map: &Vec<Vec<char>>) -> usize {
    let mut result: usize = 0;

    for (y, row) in map.iter().enumerate() {
        let mut current_number = 0;
        let mut is_part_number = false;

        for (x, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                current_number *= 10;
                current_number += *c as usize - '0' as usize;

                if !is_part_number {
                    is_part_number = has_symbol_left_to(&map, (x, y));
                }
            } else {
                if !is_part_number {
                    is_part_number = has_symbol_left_to(&map, (x, y)) || has_symbol_left_to(&map, (x + 1, y));
                }

                if current_number != 0 && is_part_number {
                    result += current_number;
                }

                current_number = 0;
                is_part_number = false;
            }
        }

        if current_number != 0 && (is_part_number || has_symbol_left_to(map, (row.len(), y))) {
            result += current_number;
        }
    }

    result
}

fn has_symbol_left_to(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> bool {
    if x == 0 {
        false
    } else {
        (y > 0 && is_symbol(map[y - 1][x - 1])) || is_symbol(map[y][x - 1]) || (y < map.len() - 1 && is_symbol(map[y + 1][x - 1]))
    }
}

fn is_symbol(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn part2(map: &Vec<Vec<char>>) -> usize {
    let mut adjacent_numbers = HashMap::<(usize, usize), Vec<usize>>::new();

    for (y, row) in map.iter().enumerate() {
        let mut current_number = 0;
        let mut gears = Vec::<(usize, usize)>::new();

        for (x, c) in row.iter().enumerate() {
            if c.is_digit(10) {
                current_number *= 10;
                current_number += *c as usize - '0' as usize;
                gears.append(&mut gears_left_to(map, (x, y)));
            } else {
                if current_number != 0 {
                    gears.append(&mut gears_left_to(map, (x, y)));
                    gears.append(&mut gears_left_to(map, (x + 1, y)));
                    for gear in gears.iter() {
                        adjacent_numbers.entry(*gear).or_default().push(current_number);
                    }
                }

                current_number = 0;
                gears.clear();
            }
        }

        if current_number != 0 {
            for gear in gears.iter() {
                adjacent_numbers.entry(*gear).or_default().push(current_number);
            }
        }
    }

    adjacent_numbers.values().filter(|v| v.len() == 2).map(|v| v[0] * v[1]).sum()
}

fn gears_left_to(map: &Vec<Vec<char>>, (x, y): (usize, usize)) -> Vec<(usize, usize)> {
    let mut result = Vec::new();
    
    if x != 0 {
        if y > 0 && map[y - 1][x - 1] == '*' {
            result.push((x - 1, y - 1));
        }

        if map[y][x - 1] == '*' {
            result.push((x - 1, y));
        }

        if y < map.len() - 1 && map[y + 1][x - 1] == '*' {
            result.push((x - 1, y + 1));
        }
    }

    result
}