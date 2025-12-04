use std::{cmp::Ordering, collections::HashSet};

fn main() {
    let input = include_str!("../../assets/day03/input.txt");
    let result: usize = input
        .lines()
        .map(Bank::from)
        .map(|bank| bank.find_max_two_digit_num())
        .sum();

    println!("Part 1: {result}");
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug, Clone, Copy)]
struct Battery(usize, usize);

struct Bank(Vec<Battery>);

impl From<&str> for Bank {
    fn from(input: &str) -> Self {
        let batteries: Vec<_> = input
            .chars()
            .enumerate()
            .flat_map(|(idx, c)| {
                let num = c.to_string().parse().map_err(|_| "invalid number")?;
                Ok::<_, String>((idx, num))
            })
            .map(|(idx, num)| Battery(num, idx))
            .collect();
        Bank(batteries)
    }
}

impl Bank {
    fn sort(&mut self) {
        self.0.sort_by(|first, second| {
            let order = first.1.cmp(&second.1);

            if order == Ordering::Equal {
                return first.1.cmp(&second.1);
            }

            order
        });
        
        self.0 = self.0.iter().rev().cloned().collect()
    }

    fn find_max_two_digit_num(&self) -> usize {
        let mut possible_nums = HashSet::<usize>::new();

        for (idx, this) in self.0.iter().enumerate() {
            for other_idx in idx + 1..self.0.len() {
                let other = self.0.get(other_idx).expect("checked");

                let first_num = this.0.to_string();
                let second_num = other.0.to_string();
                let num_str = format!("{first_num}{second_num}");
                possible_nums.insert(num_str.parse().unwrap_or(0));
            }
        }

        let result = possible_nums.iter().max().unwrap_or(&0);
        *result
    }

    fn find_max_twelve_digit_num(&mut self) -> usize {
        todo!()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_max_in_bank() {
        let mut bank: Bank = "987654321111111".into();
        
        assert_eq!(98, bank.find_max_two_digit_num());

        let bank: Bank = "811111111111119".into();
        assert_eq!(89, bank.find_max_two_digit_num());

        let bank: Bank = "234234234234278".into();
        assert_eq!(78, bank.find_max_two_digit_num());

        let bank: Bank = "818181911112111".into();
        assert_eq!(92, bank.find_max_two_digit_num());
    }
    
    #[test]
    fn test_sort_bank() {
        let mut bank: Bank = "987654321111111".into();
        
        bank.sort();
        
        assert_eq!(Battery(9,0), bank.0[0]);
        assert_eq!(Battery(8,1), bank.0[1]);

        let mut bank: Bank = "811111111111119".into();
        bank.sort();
        
        assert_eq!(Battery(8,0), bank.0[0]);
        assert_eq!(Battery(8,1), bank.0[1]);
        

        let mut bank: Bank = "234234234234278".into();

        bank.sort();
        
        assert_eq!(Battery(9,0), bank.0[0]);
        assert_eq!(Battery(8,1), bank.0[1]);
        
        let mut bank: Bank = "818181911112111".into();

        bank.sort();
        
        assert_eq!(Battery(9,0), bank.0[0]);
        assert_eq!(Battery(8,1), bank.0[1]);
    }
}
