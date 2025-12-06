use std::collections::HashMap;

fn main() {
    let input = include_str!("../../assets/day06/input.txt");
    
    let (numbers, operations) = parse(input);
    let part1 = part1(numbers, operations);
    
    println!("Part 1: {part1}");
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

fn parse(input: &str) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Operation>) {
    let operations = input
        .lines()
        .rev()
        .next()
        .expect("invalid input - empty")
        .split_ascii_whitespace()
        .flat_map(|c| match c {
            "+" => Ok(Operation::Add),
            "*" => Ok(Operation::Multiply),
            _ => Err("invalid input"),
        })
        .enumerate()
        .fold(HashMap::new(), |mut map, (idx, op)| {
            map.insert(idx, op);
            map
        });

    let numbers = input
        .lines()
        .map(|line| {
            line.split_ascii_whitespace()
                .flat_map(|num_str| num_str.parse::<usize>())
                .collect::<Vec<_>>()
        })
        .fold(
            HashMap::new(),
            |mut num_map: HashMap<usize, Vec<usize>>, num_vec| {
                num_vec.into_iter().enumerate().for_each(|(idx, num)| {
                    if let Some(numbers) = num_map.get_mut(&idx) {
                        numbers.push(num);
                    } else {
                        num_map.insert(idx, vec![num]);
                    }
                });
                num_map
            },
        );

    (numbers, operations)
}

fn part1(numbers: HashMap<usize, Vec<usize>>, operations: HashMap<usize, Operation>) -> usize {
    numbers.keys().flat_map(|idx| {
        match operations.get(idx) {
            None => Err("invalid input"),
            Some(op) => {
                match op {
                    Operation::Add => Ok(numbers.get(idx).expect("invalid input").iter().sum()),
                    Operation::Multiply => Ok(numbers.get(idx).expect("invalid input").iter().fold(1,|val, num| val * num))
                }
            }
        }
    }).sum()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "123 328  51 64\n45 64  387 23\n6 98  215 314\n*   +   *   +";
        let (numbers, operations) = parse(input);

        assert_eq!(numbers.get(&0), Some(&vec![123, 45, 6]));
        assert_eq!(numbers.get(&1), Some(&vec![328, 64, 98]));
        assert_eq!(numbers.get(&2), Some(&vec![51, 387, 215]));
        assert_eq!(numbers.get(&3), Some(&vec![64, 23, 314]));

        assert_eq!(operations.get(&0), Some(&Operation::Multiply));
        assert_eq!(operations.get(&1), Some(&Operation::Add));
        assert_eq!(operations.get(&2), Some(&Operation::Multiply));
        assert_eq!(operations.get(&3), Some(&Operation::Add));
    }
    
    #[test]
    fn test_part1() {
        let input = "123 328  51 64\n45 64  387 23\n6 98  215 314\n*   +   *   +";
        let (numbers, operations) = parse(input);
        let part1 = part1(numbers, operations);
        assert_eq!(4277556, part1);
    }
}
