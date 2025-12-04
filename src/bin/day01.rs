fn main() {
    let input = include_str!("../../assets/day01/input.txt");

    let instructions: Vec<Instruction> = input.lines().flat_map(|line| line.try_into()).collect();
    let mut part1_dial = SafeDial::<100>::new(50);

    for instruction in &instructions {
        match instruction {
            Instruction::Left(val) => part1_dial.move_dial(*val as isize),
            Instruction::Right(val) => part1_dial.move_dial(-(*val as isize)),
        }
    }

    println!("Part 01: {}", part1_dial.count);

    let mut part2_dial = SafeDial::<100>::new(50);

    for instruction in &instructions {
        match instruction {
            Instruction::Left(val) => part2_dial.move_dial_with_count(*val as isize),
            Instruction::Right(val) => part2_dial.move_dial_with_count(-(*val as isize)),
        }
    }

    println!("Part 02: {}", part2_dial.count);
}

struct SafeDial<const M: isize> {
    dial: isize,
    count: usize,
}

impl<const M: isize> SafeDial<M> {
    fn new(init: isize) -> Self {
        Self {
            dial: init,
            count: 0,
        }
    }

    fn move_dial(&mut self, val: isize) {
        let total = self.dial + val;
        self.dial = total.rem_euclid(M);

        if self.dial == 0 {
            self.count += 1;
        }
    }

    fn move_dial_with_count(&mut self, val: isize) {
        
        let left_turn = val.is_negative();
        let direction: isize = if left_turn {
            -1
        } else {
            1
        };
        
        let mut amount = val.abs(); 
        
        while amount > 0 {
            self.dial = (self.dial + direction).rem_euclid(M);
            
            if self.dial == 0 {
                self.count += 1;
            }
            
            amount -= 1
        }
        
    }
}

#[derive(Debug, PartialEq)]
enum Instruction {
    Left(usize),
    Right(usize),
}

impl TryFrom<&str> for Instruction {
    type Error = String;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        let get_val = |delim: char| {
            let (_, val) = value.split_once(delim).ok_or("Invalid input")?;
            let val = val.parse().map_err(|_| "Invalid number")?;

            Ok::<usize, String>(val)
        };

        match value.chars().peekable().peek() {
            Some(char) if *char == 'L' => {
                let val = get_val('L')?;

                Ok(Self::Left(val))
            }
            Some(char) if *char == 'R' => {
                let val = get_val('R')?;

                Ok(Self::Right(val))
            }
            _ => Err("Invalid input".to_string()),
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_instruction_parsing() {
        let input = "L68\nL30\nR48\nL5\nR60\nL55\nL1\nL99\nR14\nL82";

        let instructions: Vec<Instruction> =
            input.lines().flat_map(|line| line.try_into()).collect();

        assert_eq!(instructions.len(), 10);

        assert_eq!(instructions[0], Instruction::Left(68));
        assert_eq!(instructions[1], Instruction::Left(30));
        assert_eq!(instructions[2], Instruction::Right(48));
        assert_eq!(instructions[3], Instruction::Left(5));
        assert_eq!(instructions[4], Instruction::Right(60));
        assert_eq!(instructions[5], Instruction::Left(55));
        assert_eq!(instructions[6], Instruction::Left(1));
        assert_eq!(instructions[7], Instruction::Left(99));
        assert_eq!(instructions[8], Instruction::Right(14));
        assert_eq!(instructions[9], Instruction::Left(82));
    }

    #[test]
    fn test_wrapping_dial_ops() {
        let mut dial = SafeDial::<100>::new(50);
        dial.move_dial(-68);
        assert_eq!(dial.dial, 82);
        dial.move_dial(18);
        assert_eq!(dial.dial, 0);
    }

    #[test]
    fn test_counting_zeros() {
        let mut dial = SafeDial::<100>::new(50);
        dial.move_dial(-68);
        dial.move_dial(-30);
        dial.move_dial(48);
        dial.move_dial(-5);
        dial.move_dial(60);
        dial.move_dial(-55);
        dial.move_dial(-1);
        dial.move_dial(-99);
        dial.move_dial(14);
        dial.move_dial(82);

        assert_eq!(dial.count, 3);
    }

    #[test]
    fn test_counting_zeros_and_passing_zeros() {
        let mut dial = SafeDial::<100>::new(50);
        dial.move_dial_with_count(-68);
        assert_eq!(dial.dial, 82);
        assert_eq!(dial.count, 1);
        dial.move_dial_with_count(-30);
        assert_eq!(dial.count, 1);
        assert_eq!(dial.dial, 52);
        dial.move_dial_with_count(48);
        assert_eq!(dial.dial, 0);
        assert_eq!(dial.count, 2);
        dial.move_dial_with_count(-5);
        assert_eq!(dial.dial, 95);
        dial.move_dial_with_count(60);
        assert_eq!(dial.dial, 55);
        dial.move_dial_with_count(-55);
        assert_eq!(dial.dial, 0);
        dial.move_dial_with_count(-1);
        assert_eq!(dial.dial, 99);
        dial.move_dial_with_count(-99);
        assert_eq!(dial.dial, 0);
        dial.move_dial_with_count(14);
        assert_eq!(dial.dial, 14);
        dial.move_dial_with_count(-82);
        assert_eq!(dial.dial, 32);

        assert_eq!(dial.count, 6);
    }
}
