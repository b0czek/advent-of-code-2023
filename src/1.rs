use std::fs::read_to_string;

static WORDS: &'static [&'static str] = &[
    "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
];

trait FirstAndLast {
    fn first_and_last(&self) -> Self;
    fn first_and_last2(&self) -> Self;
}
impl FirstAndLast for String {
    fn first_and_last(&self) -> Self {
        let first = self.chars().nth(0).unwrap().to_string();
        let last = self.chars().nth(self.len() - 1).unwrap().to_string();
        let chars = [first, last];
        chars.into_iter().collect()
    }

    fn first_and_last2(&self) -> Self {
        let from_words = WORDS
            .iter()
            .map(|word| self.match_indices(word))
            .flatten()
            .map(|(i, num)| {
                (
                    i,
                    WORDS
                        .iter()
                        .position(|word| *word == num)
                        .and_then(|digit| char::from_digit(digit as u32, 10))
                        .unwrap(),
                )
            });

        let from_digits = self.char_indices().filter(|(_, c)| c.is_numeric());

        let joined_iter = from_words.chain(from_digits);
        let mut v: Vec<_> = joined_iter.collect();
        v.sort_by(|a, b| a.0.cmp(&b.0));

        let first = v.first().unwrap().1;
        let last = v.last().unwrap().1;

        let chars = [first, last];
        chars.into_iter().collect()
    }
}

fn main() {
    let contents: Vec<_> = read_to_string("input/1.txt")
        .unwrap()
        .lines()
        .map(String::from)
        .collect();

    let sum1 = contents
        .iter()
        .map(|l| {
            l.chars()
                .filter(|c| c.is_numeric())
                .collect::<String>()
                .first_and_last()
                .parse::<i32>()
                .unwrap()
        })
        .sum::<i32>();

    println!("{:?}", sum1);

    let sum2 = contents
        .iter()
        .map(|l| l.first_and_last2().parse::<i32>().unwrap())
        .sum::<i32>();
    println!("{:?}", sum2);
}
