use std::collections::{HashSet, VecDeque};

fn main() {
    let input = include_str!("../../assets/day10/input.txt");
    let part1 = solve_part1(input);
    println!("Part 1: {}", part1);

    // let part2 = solve_part2(input);
    // println!("Part 2: {}", part2);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash)]
struct MachineLight(bool);

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
struct MachineJoltage(Vec<usize>);

impl MachineJoltage {
    fn new(val: Vec<usize>) -> Self {
        Self(val)
    }

    fn update(&mut self, idx_list: &[usize]) {
        for idx in idx_list {
            if let Some(val) = self.0.get_mut(*idx) {
                *val += 1
            }
        }
    }
}

impl MachineLight {
    fn toggle(&mut self) {
        self.0 = !self.0
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Button(Vec<usize>);

struct MachineSolver<'m> {
    machine: &'m Machine,
    queue: VecDeque<(Vec<MachineLight>, usize)>,
    visited: HashSet<Vec<MachineLight>>,
}

impl<'m> MachineSolver<'m> {
    fn new(machine: &'m Machine) -> Self {
        MachineSolver {
            machine,
            queue: VecDeque::from([(machine.state.clone(), 0)]),
            visited: HashSet::new(),
        }
    }
}

impl<'m> Iterator for MachineSolver<'m> {
    type Item = (Vec<MachineLight>, usize);

    fn next(&mut self) -> Option<Self::Item> {
        while let Some((current_state, pressed_buttons)) = self.queue.pop_front() {
            if current_state == self.machine.desired {
                return Some((current_state, pressed_buttons.clone()));
            }

            for button in self.machine.buttons.iter() {
                let mut state = current_state.clone();
                for idx in button.0.iter() {
                    if let Some(light) = state.get_mut(*idx) {
                        light.toggle();
                    }
                }

                if !self.visited.insert(state.clone()) {
                    continue;
                }

                let buttons = pressed_buttons + 1;
                self.queue.push_back((state.clone(), buttons));
            }
        }

        None
    }
}

struct Machine {
    desired: Vec<MachineLight>,
    joltage: MachineJoltage,
    target: MachineJoltage,
    state: Vec<MachineLight>,
    buttons: Vec<Button>,
}

impl Machine {
    fn new(desired: Vec<MachineLight>, buttons: Vec<Button>, target: MachineJoltage) -> Self {
        let len = desired.len();
        let jlen = target.0.len();
        Self {
            desired,
            joltage: MachineJoltage::new(vec![0; jlen]),
            target,
            state: vec![MachineLight(false); len],
            buttons,
        }
    }

    fn solve_iter<'m>(&'m self) -> MachineSolver<'m> {
        MachineSolver::new(self)
    }
}

fn parse_state(input: &str) -> Vec<MachineLight> {
    input
        .chars()
        .skip(1)
        .take(input.len() - 1)
        .flat_map(|c| match c {
            '#' => Some(MachineLight(true)),
            '.' => Some(MachineLight(false)),
            _ => None,
        })
        .collect()
}

fn parse_joltage(input: &str) -> MachineJoltage {
    let input = input.trim_end_matches('}').trim_start_matches('{');
    let vals: Vec<_> = input
        .split(',')
        .flat_map(|num_str| num_str.parse::<usize>())
        .collect();

    MachineJoltage::new(vals)
}

fn parse_input(input: &str) -> Vec<Machine> {
    input
        .lines()
        .flat_map(|line| {
            let desired = line.split_ascii_whitespace().next()?;
            let desired = parse_state(desired);

            let joltage = line
                .split_ascii_whitespace()
                .rev()
                .next()?;
            let joltage = parse_joltage(joltage);

            let buttons: Vec<_> = line
                .split_ascii_whitespace()
                .skip(1)
                .take_while(|val| val.starts_with('('))
                .map(|button_str| {
                    let commands: Vec<_> = button_str
                        .split(',')
                        .flat_map(|cmd| {
                            cmd.chars()
                                .filter(|c| c.is_ascii_digit())
                                .collect::<String>()
                                .parse::<usize>()
                        })
                        .collect();
                    Button(commands)
                })
                .collect();

            Some(Machine::new(desired, buttons, joltage))
        })
        .collect()
}

fn solve_part1(input: &str) -> usize {
    let machines = parse_input(input);
    machines
        .iter()
        .map(|machine| machine.solve_iter().next())
        .flat_map(|solution| solution)
        .map(|(_, button_cnt)| button_cnt)
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let machines = parse_input(input);

    unimplemented!()
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../../assets/day10/test.txt");
        let part1 = solve_part1(input);

        assert_eq!(7, part1);
    }

    #[test]
    fn test_solve_part2() {
        let input = include_str!("../../assets/day10/test.txt");
        let part1 = solve_part2(input);

        assert_eq!(33, part1);
    }
}
