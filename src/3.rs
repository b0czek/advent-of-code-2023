use std::collections::{HashMap, HashSet};
use std::fmt::Debug;
use std::{
    cmp::{max, min},
    fs::read_to_string,
    ops::{Range, RangeInclusive},
};

#[derive(PartialEq, Eq, Hash)]
struct Field(usize, usize, char);

impl Field {
    fn find_neighbours<'a>(&self, sch: &'a Schematic) -> Vec<&'a Field> {
        fn neighbour_range(val: usize) -> RangeInclusive<usize> {
            max(0isize, val as isize - 1) as usize..=val + 1
        }
        neighbour_range(self.0)
            .into_iter()
            .map(|y| {
                neighbour_range(self.1)
                    .into_iter()
                    .map(move |x| sch.get_field(x, y))
            })
            .flatten()
            .filter_map(|field| field)
            .filter(|field| field.0 != self.0 || field.1 != self.1)
            .collect()
    }

    fn number_fields<'a>(&self, sch: &'a Schematic) -> Vec<&'a Field> {
        sch.0
            .get(self.0)
            .unwrap()
            .iter()
            .skip(self.1)
            .take_while(|field| field.2.to_digit(10).is_some())
            .collect::<Vec<_>>()
    }

    fn try_into_number(&self, sch: &Schematic) -> Option<usize> {
        self.number_fields(sch)
            .iter()
            .map(|field| field.2)
            .collect::<String>()
            .parse()
            .ok()
    }

    fn adjacent_to_symbol(&self, sch: &Schematic) -> bool {
        self.number_fields(sch).iter().any(|number| {
            number
                .find_neighbours(sch)
                .iter()
                .any(|neighbour| "&$=-%*/@#+".contains(neighbour.2))
        })
    }

    fn gears<'a>(&self, sch: &'a Schematic) -> HashSet<&'a Field> {
        self.number_fields(sch)
            .iter()
            .map(|number| {
                number
                    .find_neighbours(sch)
                    .into_iter()
                    .filter(|neigbour| neigbour.2 == '*')
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect::<HashSet<_>>()
    }
}

impl Debug for Field {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("Field")
            .field(&self.0)
            .field(&self.1)
            .field(&self.2)
            .finish()
    }
}

struct Schematic(Vec<Vec<Field>>);

impl Schematic {
    fn get_field<'a>(&'a self, x: usize, y: usize) -> Option<&'a Field> {
        self.0.get(y).and_then(|row| row.get(x))
    }
}

fn main() {
    let contents: Vec<_> = read_to_string("input/3.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let schematic = Schematic(
        contents
            .iter()
            .enumerate()
            .map(|(y, line)| {
                line.char_indices()
                    .map(|(x, char)| Field(y, x, char))
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<_>>(),
    );

    let numbers = schematic
        .0
        .iter()
        .map(|row| {
            row.iter().rev().filter(|x| x.2.is_numeric()).fold(
                vec![],
                |mut vec: Vec<&Field>, field| {
                    if vec
                        .last()
                        .filter(|last_field| last_field.1 == field.1 + 1)
                        .is_some()
                    {
                        vec.pop();
                    }
                    vec.push(field);
                    vec
                },
            )
        })
        .flatten()
        .collect::<Vec<_>>();

    let sum: usize = numbers
        .iter()
        .filter(|field| field.adjacent_to_symbol(&schematic))
        .map(|field| field.try_into_number(&schematic).unwrap_or_default())
        .sum();

    println!("sum: {}", sum);

    let gears = numbers
        .iter()
        .map(|number| {
            number
                .gears(&schematic)
                .into_iter()
                .map(move |gear| (gear, number))
        })
        .flatten()
        .fold(HashMap::new(), |mut acc, (gear, number)| {
            acc.entry(gear).or_insert(vec![]).push(number);
            acc
        });

    let sum_gears: usize = gears
        .iter()
        .filter(|(key, values)| values.len() == 2)
        .map(|(_, values)| {
            values.iter().fold(1, |acc, val| {
                acc * val.try_into_number(&schematic).unwrap_or_default()
            })
        })
        .sum();

    println!("{:?}", sum_gears);
}
