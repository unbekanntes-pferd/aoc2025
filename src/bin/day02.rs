use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../assets/day02/input.txt");
    let ranges = parse(input);

    let result: usize = ranges
        .clone()
        .into_iter()
        .map(|range| range.get_symmetric_nums())
        .flatten()
        .sum();

    println!("Part 1: {result}");
    
    let result2: usize = ranges
        .into_iter()
        .map(|range| range.get_repeating_nums())
        .flatten()
        .sum();
    
    println!("Part 2: {result2}");
    
}

fn parse(input: &str) -> Vec<RangeInclusive<usize>> {
    input
        .split(',')
        .flat_map(|range_str| {
            let (first, second) = range_str.split_once('-').ok_or("invalid input")?;
            let first = first.parse().map_err(|_| "invalid number")?;
            let second = second.parse().map_err(|_| "invalid number")?;

            Ok::<_, String>(RangeInclusive::new(first, second))
        })
        .collect()
}

trait ValidNums {
    fn get_symmetric_nums(self) -> Vec<usize>;
    fn get_repeating_nums(self) -> Vec<usize>;
}

impl ValidNums for RangeInclusive<usize> {
    fn get_symmetric_nums(self) -> Vec<usize> {
        self.into_iter()
            .filter(|num| {
                let num_str = num.to_string();

                if num_str.len() % 2 == 0 {
                    let (first, second) = num_str.split_at(num_str.len() / 2);

                    return first == second;
                }

                false
            })
            .collect()
    }

    fn get_repeating_nums(self) -> Vec<usize> {
        self.into_iter()
            .filter(|num| {
                let num_str = num.to_string();
                let num_len = num_str.len();

                let mut is_repeating_pattern = false;
                let max_pattern_size = num_len / 2;
                let num_chars: Vec<_> = num_str.chars().collect();
            
                for pattern_size in 1..=max_pattern_size {
                    let pattern = num_chars.chunks(pattern_size).next().expect("not empty"); 
                    is_repeating_pattern = num_chars.chunks(pattern_size).skip(1).all(|other| other == pattern);
                    if is_repeating_pattern {
                        break
                    }
                }
                
                is_repeating_pattern
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parser() {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";

        let ranges = parse(input);

        assert_eq!(ranges[0], RangeInclusive::new(11, 22));
        assert_eq!(ranges[1], RangeInclusive::new(95, 115));
        assert_eq!(ranges[2], RangeInclusive::new(998, 1012));
        assert_eq!(ranges[3], RangeInclusive::new(1188511880, 1188511890));
        assert_eq!(ranges[4], RangeInclusive::new(222220, 222224));
        assert_eq!(ranges[5], RangeInclusive::new(1698522, 1698528));
    }

    #[test]
    fn test_symmetric_nums() {
        assert_eq!(
            vec![11, 22],
            RangeInclusive::new(11, 22).get_symmetric_nums()
        );
        assert_eq!(vec![99], RangeInclusive::new(95, 115).get_symmetric_nums());
        assert_eq!(
            vec![1010],
            RangeInclusive::new(998, 1012).get_symmetric_nums()
        );
        assert_eq!(
            vec![1188511885],
            RangeInclusive::new(1188511880, 1188511890).get_symmetric_nums()
        );
        assert_eq!(
            vec![222222],
            RangeInclusive::new(222220, 222224).get_symmetric_nums()
        );
        assert_eq!(
            Vec::<usize>::new(),
            RangeInclusive::new(1698522, 1698528).get_symmetric_nums()
        );
    }
    
    #[test]
    fn test_repeating_nums() {
        assert_eq!(
            vec![11, 22],
            RangeInclusive::new(11, 22).get_repeating_nums()
        );
        assert_eq!(vec![99, 111], RangeInclusive::new(95, 115).get_repeating_nums());
        assert_eq!(
            vec![999, 1010],
            RangeInclusive::new(998, 1012).get_repeating_nums()
        );
        assert_eq!(
            vec![1188511885],
            RangeInclusive::new(1188511880, 1188511890).get_repeating_nums()
        );
        assert_eq!(
            vec![222222],
            RangeInclusive::new(222220, 222224).get_repeating_nums()
        );
        assert_eq!(
            Vec::<usize>::new(),
            RangeInclusive::new(1698522, 1698528).get_repeating_nums()
        );
    }
}
