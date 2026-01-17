use std::collections::HashMap;

fn main() {
    let input = include_str!("../../assets/day06/input.txt");

    let (numbers, operations) = parse(input);
    let part1 = solve(numbers, operations);

    println!("Part 1: {part1}");
    
    let operations = parse_operations(input);
    let rows = parse_matrix(input);
    let cols = transpose(rows);
    let col_nums = parse_transposed_nums(cols);
    let col_nums = col_nums.iter().rev().enumerate().fold(HashMap::new(), |mut map, (idx, val)| {
        map.insert(idx, val.clone());
        map
    });
    
    let part2 = solve(col_nums, operations);
    println!("Part 2: {part2}");
    
}

#[derive(Debug, PartialEq)]
enum Operation {
    Add,
    Multiply,
}

fn parse_operations(input: &str) -> HashMap<usize, Operation> {
    input
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
        })
}

fn parse_matrix(input: &str) -> Vec<Vec<char>> {
    input
        .lines()
        .map(|line| line.chars().collect::<Vec<_>>())
        .filter(|line| line.iter().all(|c| *c != '+' && *c != '*'))
        .collect()
}

fn parse(input: &str) -> (HashMap<usize, Vec<usize>>, HashMap<usize, Operation>) {
    let operations = parse_operations(input);

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

fn transpose(matrix: Vec<Vec<char>>) -> Vec<Vec<char>> {
    let mut columns = Vec::new();
    let len = matrix[0].len();

    for idx in 0..len {
        let mut col = Vec::new();
        for row in &matrix {
            if let Some(val) = row.get(idx) {
                col.push(*val)
            }
        }
        columns.push(col);
    }

    columns
}

fn parse_transposed_nums(transposed_nums: Vec<Vec<char>>) -> Vec<Vec<usize>> {
    let mut cols = Vec::new();
    let mut current = Vec::new();

    for col in transposed_nums.iter().rev() {
        if col.iter().all(|c| c.is_whitespace()) {
            cols.push(current.clone());
            current.clear();
            continue
        }

        let num: usize = col
            .iter()
            .filter_map(|c| c.to_digit(10))
            .fold(0, |acc, digit| acc * 10 + digit as usize);

        current.push(num);
    }
    
    cols.push(current.clone());

    cols
}

fn solve(
    numbers: HashMap<usize, Vec<usize>>,
    operations: HashMap<usize, Operation>,
) -> usize {
    numbers
        .keys()
        .flat_map(|idx| match operations.get(idx) {
            None => Err("invalid input"),
            Some(op) => match op {
                Operation::Add => Ok(numbers.get(idx).ok_or("invalid input")?.iter().sum::<usize>()),
                Operation::Multiply => Ok(numbers
                    .get(idx)
                    .ok_or("invalid_input")?
                    .iter()
                    .product())
            },
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "123 328  51 64\n45 64  387 23\n6 98  215 314\n*   +   *   +  ";
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
    fn test_solve_part1() {
        let input = "123 328  51 64\n45 64  387 23\n6 98  215 314\n*   +   *   +";
        let (numbers, operations) = parse(input);
        let part1 = solve(numbers, operations);
        assert_eq!(4277556, part1);
    }

    #[test]
    fn test_transpose() {
        let input = "123 328  51 64 \n 45 64  387 23\n  6 98  215 314\n*   +   *   +  ";
        let rows = parse_matrix(input);
        let cols = transpose(rows);

        assert_eq!(cols[0], vec!['1', ' ', ' ']);
        assert_eq!(cols[1], vec!['2', '4', ' ']);
        assert_eq!(cols[2], vec!['3', '5', '6']);
        assert_eq!(cols[3], vec![' ', ' ', ' ']);
        assert_eq!(cols[4], vec!['3', '6', '9']);
        assert_eq!(cols[5], vec!['2', '4', '8']);
        assert_eq!(cols[6], vec!['8', ' ', ' ']);
        assert_eq!(cols[7], vec![' ', ' ', ' ']);
        assert_eq!(cols[8], vec![' ', '3', '2']);
        assert_eq!(cols[9], vec!['5', '8', '1']);
        assert_eq!(cols[10], vec!['1', '7', '5']);
        assert_eq!(cols[11], vec![' ', ' ', ' ']);
        assert_eq!(cols[12], vec!['6', '2', '3']);
        assert_eq!(cols[13], vec!['4', '3', '1']);
        assert_eq!(cols[14], vec![' ', '4']);
    }
    
    #[test]
    fn test_parse_transposed_nums() {
        let input = "123 328  51 64 \n 45 64  387 23\n  6 98  215 314\n*   +   *   +  ";
        let rows = parse_matrix(input);
        let cols = transpose(rows);
        let cols = parse_transposed_nums(cols);
        
        assert_eq!(cols[0], vec![4, 431, 623]);
        assert_eq!(cols[1], vec![175, 581, 32]);
        assert_eq!(cols[2], vec![8, 248, 369]);
        assert_eq!(cols[3], vec![356, 24, 1]);
    }

    #[test]
    fn test_solve_part2() {
        let input = "123 328  51 64 \n 45 64  387 23\n  6 98  215 314\n*   +   *   +  ";
        let operations = parse_operations(input);
        let rows = parse_matrix(input);
        let cols = transpose(rows);
        let col_nums = parse_transposed_nums(cols);
        let col_nums = col_nums.iter().rev().enumerate().fold(HashMap::new(), |mut map, (idx, val)| {
            map.insert(idx, val.clone());
            map
        });

        let result = solve(col_nums, operations);

        assert_eq!(3263827, result);
    }
}
