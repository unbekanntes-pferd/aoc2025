use std::{collections::HashSet};

fn main() {
    let input = include_str!("../../assets/day03/input.txt");
    let result: usize = input
        .lines()
        .map(Bank::from)
        .map(|bank| bank.find_max_two_digit_num())
        .sum();

    println!("Part 1: {result}");
    
    let result2: usize = input
        .lines()
        .map(Bank::from)
        .map(|mut bank| bank.find_max_twelve_digit_num())
        .sum();
    
    println!("Part 2: {result2}");
}

#[derive(PartialOrd, PartialEq, Eq, Ord, Debug, Clone, Copy)]
struct Battery(u8, usize);

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
        let mut idx_list = [0usize; 12];
        let mut curr_pos = 0usize;
        let mut skips = self.0.len() - 12;
        
        for idx in 0..12 {
            let slice = &self.0[curr_pos..=curr_pos + skips];
            let max_val = slice.iter().rev().max_by_key(|battery| battery.0).expect("no max value");
            idx_list[idx] = max_val.1;
            skips -= max_val.1 - curr_pos;
            curr_pos = max_val.1 + 1;
        }
        
        self.build_number_from_idx_list(&idx_list)
    }
    
    fn build_number_from_idx_list(&mut self, idx_list: &[usize; 12]) -> usize {
        let mut nums = [0u8; 12];
        
        for (i, idx) in idx_list.iter().enumerate() {
            let num = self.0[*idx].0;
            nums[i] = num;
        }
        
        nums.into_iter().fold(0, |acc, num| acc * 10 + num as usize)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_find_max_in_bank() {
        let bank: Bank = "987654321111111".into();
        
        assert_eq!(98, bank.find_max_two_digit_num());

        let bank: Bank = "811111111111119".into();
        assert_eq!(89, bank.find_max_two_digit_num());

        let bank: Bank = "234234234234278".into();
        assert_eq!(78, bank.find_max_two_digit_num());

        let bank: Bank = "818181911112111".into();
        assert_eq!(92, bank.find_max_two_digit_num());
    }
    
    #[test]
    fn test_max_12_digit_numbers() {
        let mut bank: Bank = "987654321111111".into();
        
        assert_eq!(bank.find_max_twelve_digit_num(), 987654321111);
        
        let mut bank: Bank = "811111111111119".into();
        
        assert_eq!(bank.find_max_twelve_digit_num(), 811111111119); 
    }
}
