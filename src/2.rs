use std::{fs::read_to_string, iter::Map};

static MAX_RED: u32 = 12;
static MAX_GREEN: u32 = 13;
static MAX_BLUE: u32 = 14;

struct Game {
    id: u32,
    r: u32,
    g: u32,
    b: u32,
}

trait GameParse: Iterator {
    fn parse_game<'a>(self) -> Map<Self, fn(&String) -> Game>
    where
        Self: Iterator<Item = &'a String> + Sized,
    {
        fn parse_game(str: &String) -> Game {
            let split: Vec<_> = str.split(':').collect();

            let id: u32 = split[0].split(' ').collect::<Vec<_>>()[1].parse().unwrap();

            fn max_if_matched_color((c, v1): (&str, u32), expected_color: &str, v2: u32) -> u32 {
                if c == expected_color {
                    v1.max(v2)
                } else {
                    v2
                }
            }

            let (r, g, b) = split[1]
                .split(';')
                .map(|handful| {
                    handful
                        .split(',')
                        .map(|cubes| cubes.trim().split(' ').collect::<Vec<_>>())
                        .map(|cube| (cube[1], cube[0].parse::<u32>().unwrap()))
                })
                .flatten()
                .fold((0, 0, 0), |(r, g, b), pair| {
                    (
                        max_if_matched_color(pair, "red", r),
                        max_if_matched_color(pair, "green", g),
                        max_if_matched_color(pair, "blue", b),
                    )
                });

            Game { id, r, g, b }
        }

        self.map(parse_game)
    }
}

impl<I: Iterator> GameParse for I {}

fn main() {
    let contents: Vec<_> = read_to_string("input/2.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let result1: u32 = contents
        .iter()
        .parse_game()
        .filter(|game| game.b <= MAX_BLUE && game.r <= MAX_RED && game.g <= MAX_GREEN)
        .map(|game| game.id)
        .sum();

    println!("result: {}", result1);

    let result2: u32 = contents
        .iter()
        .parse_game()
        .map(|game| game.r * game.g * game.b)
        .sum();

    println!("result: {}", result2);
}
