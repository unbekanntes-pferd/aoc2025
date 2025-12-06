use std::ops::RangeInclusive;

fn main() {
    let input = include_str!("../../assets/day05/input.txt");

    let (ranges, ids) = parse(input);

    let part1 = solve_part1(&ranges, &ids);

    println!("Part 1: {part1}");

    let part2 = solve_part2(ranges);

    println!("Part 2: {part2}");
}

fn parse(input: &str) -> (Vec<RangeInclusive<usize>>, Vec<usize>) {
    let (ranges, ids) = input.split_once("\n\n").expect("malformed input");

    let ranges: Vec<_> = ranges
        .lines()
        .flat_map(|line| line.split_once('-').ok_or("malformed range"))
        .flat_map(|(first, second)| {
            let first: usize = first.parse().map_err(|_| "invalid number")?;
            let second = second.parse().map_err(|_| "invalid number")?;

            Ok::<_, String>(RangeInclusive::new(first, second))
        })
        .collect();

    let ids = ids.lines().flat_map(|line| line.parse()).collect();

    (ranges, ids)
}

fn solve_part1(ranges: &Vec<RangeInclusive<usize>>, ids: &Vec<usize>) -> usize {
    ids.iter()
        .filter(|num| ranges.iter().any(|range| range.contains(num)))
        .count()
}

trait MergeSortedRanges<T: Sized> {
    fn merge(&self, other: &T) -> Option<T>;
}

impl MergeSortedRanges<RangeInclusive<usize>> for RangeInclusive<usize> {
    fn merge(&self, other: &Self) -> Option<Self> {
        // if not sorted correctly or no overlap
        if self.start() > other.start() || other.start() > self.end() {
            return None;
        };
        let new_end = self.end().max(other.end());

        Some(RangeInclusive::new(*self.start(), *new_end))
    }
}

fn solve_part2(ranges: Vec<RangeInclusive<usize>>) -> usize {
    let mut ranges = ranges;
    ranges.sort_by_key(|range| *range.start());

    let mut ranges = ranges.into_iter();

    let mut result = Vec::new();

    let Some(mut current) = ranges.next() else {
        return 0;
    };

    while let Some(next) = ranges.next() {
        // if current overlaps with next - merge
        if let Some(merged) = current.merge(&next) {
            current = merged;
        } else {
            // push current and update next
            result.push(current);
            current = next;
        }
    }
    result.push(current);

    result
        .iter()
        .map(|range| range.end() - range.start() + 1)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (ranges, ids) = parse(input);

        assert_eq!(ranges[0], RangeInclusive::new(3, 5));
        assert_eq!(ranges[1], RangeInclusive::new(10, 14));
        assert_eq!(ranges[3], RangeInclusive::new(12, 18));
        assert_eq!(ranges[2], RangeInclusive::new(16, 20));

        assert_eq!(ids, vec![1, 5, 8, 11, 17, 32]);
    }

    #[test]
    fn test_part1() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (ranges, ids) = parse(input);

        let result = solve_part1(&ranges, &ids);

        assert_eq!(3, result);
    }

    #[test]
    fn test_part2() {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32";
        let (ranges, _) = parse(input);

        let result = solve_part2(ranges);

        assert_eq!(14, result);
    }
}
