use itertools::Itertools;
use regex::Regex;
use std::cell::RefCell;

pub fn part_one(input: &str) -> Option<u64> {
    const DIVIDE_WORRY: i64 = 3;
    const ROUNDS: i64 = 20;
    let monkey_business_level = calc_monkey_business(input, DIVIDE_WORRY, ROUNDS);
    Some(monkey_business_level as u64)
}

pub fn part_two(input: &str) -> Option<u64> {
    const DIVIDE_WORRY: i64 = 1;
    const ROUNDS: i64 = 10000;
    let monkey_business_level = calc_monkey_business(input, DIVIDE_WORRY, ROUNDS);
    Some(monkey_business_level as u64)
}

fn calc_monkey_business(input: &str, divide_worry: i64, rounds: i64) -> i64 {
    let monkeys = parse_monkeys(input).unwrap();
    let modulus = monkeys.iter().fold(1, |a, m| a * m.test_num);
    for _ in 0..rounds {
        for monkey in &monkeys {
            while monkey.has_items() {
                let (item, target) = monkey.throw_item(divide_worry, modulus);
                monkeys[target].catch_item(item);
            }
        }
    }

    let monkey_business_level = monkeys
        .iter()
        .map(|m| m.inspected_count())
        .sorted_by(|a, b| b.cmp(a))
        .take(2)
        .fold(1, |a, x| a * x);

    monkey_business_level
}

/// Returns the string contents of the first matched capture group.
pub fn re_capture_group<'a>(re: &Regex, text: &'a str) -> Option<&'a str> {
    re.captures(text)
        .and_then(|c| c.get(1))
        .and_then(|g| Some(g.as_str()))
}

/// Returns the string contents of the matched capture groups.
pub fn re_capture_groups<'a>(re: &Regex, text: &'a str) -> Option<Vec<&'a str>> {
    re.captures(text).map(|c| {
        c.iter()
            .skip(1)
            .filter_map(|x| x)
            .map(|x| x.as_str())
            .collect_vec()
    })
}

fn parse_monkeys(input: &str) -> Result<Vec<Monkey>, Box<dyn std::error::Error>> {
    let err = "invalid input";
    let items_re = Regex::new(r"Starting items: (.*)\n")?;
    let operation_re = Regex::new(r"Operation: new = (\S+) (\S) (\S+)")?;
    let test_re = Regex::new(r"Test: divisible by (\d+)")?;
    let true_re = Regex::new(r"If true: throw to monkey (\d+)")?;
    let false_re = Regex::new(r"If false: throw to monkey (\d+)")?;

    let definitions = input.split("\n\n");
    let mut monkeys = Vec::new();
    for def in definitions {
        let items = re_capture_group(&items_re, def)
            .ok_or(err)?
            .split(", ")
            .map(|x| x.parse())
            .collect::<Result<Vec<_>, _>>()?;
        let operation_parts = re_capture_groups(&operation_re, def)
            .ok_or(err)?
            .into_iter()
            .map(|x| x.to_owned())
            .collect_tuple::<(_, _, _)>()
            .ok_or(err)?;
        let test_num = re_capture_group(&test_re, def).ok_or(err)?.parse()?;
        let true_target = re_capture_group(&true_re, def).ok_or(err)?.parse()?;
        let false_target = re_capture_group(&false_re, def).ok_or(err)?.parse()?;

        let operation = move |old: i64, modulus: i64| {
            let parse_num = |x: &String| match x.as_str() {
                "old" => old,
                num_str => num_str.parse().unwrap(),
            };
            let (a, op, b) = &operation_parts;
            let a = parse_num(a);
            let b = parse_num(b);

            match op.as_str() {
                "+" => (a + b) % modulus,
                _ => (a * b) % modulus,
            }
        };

        monkeys.push(Monkey {
            items: RefCell::new(items),
            inspected_count: RefCell::new(0),
            operation: Box::new(operation),
            test_num,
            true_target,
            false_target,
        });
    }

    Ok(monkeys)
}

struct Monkey {
    items: RefCell<Vec<i64>>,
    inspected_count: RefCell<i64>,
    operation: Box<dyn Fn(i64, i64) -> i64>,
    test_num: i64,
    true_target: usize,
    false_target: usize,
}
impl Monkey {
    fn throw_item(&self, divide_worry: i64, modulus: i64) -> (i64, usize) {
        *self.inspected_count.borrow_mut() += 1;
        let mut worry = self.items.borrow_mut().remove(0);
        worry = (self.operation)(worry, modulus) / divide_worry;
        if worry % self.test_num == 0 {
            (worry, self.true_target)
        } else {
            (worry, self.false_target)
        }
    }

    fn catch_item(&self, item: i64) {
        self.items.borrow_mut().push(item);
    }

    fn has_items(&self) -> bool {
        self.items.borrow().len() > 0
    }

    fn inspected_count(&self) -> i64 {
        *self.inspected_count.borrow()
    }
}

fn main() {
    let input = &advent_of_code::read_file("inputs", 11);
    advent_of_code::solve!(1, part_one, input);
    advent_of_code::solve!(2, part_two, input);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_one(&input), Some(10605));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::read_file("examples", 11);
        assert_eq!(part_two(&input), Some(2713310158));
    }
}
