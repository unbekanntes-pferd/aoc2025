use std::collections::HashSet;

fn main() {
    let input = include_str!("../../assets/day08/input.txt");
    let part1 = solve_part1(input, 1000);
    println!("Part 1: {part1}");
    
    let part2 = solve_part2(input);
    println!("Part 2: {part2}");
}

#[derive(PartialEq, Eq, Hash, Debug, Clone, Copy)]
struct JunctionBox(isize, isize, isize);

impl Eq for OrderedFloat {}

impl Ord for OrderedFloat {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("all numbers valid")
    }
}

#[derive(Debug, PartialEq, PartialOrd, Clone, Copy)]
struct OrderedFloat(f64);

trait EuclidianDistance {
    fn distance(&self, other: &Self) -> OrderedFloat;
}

impl EuclidianDistance for JunctionBox {
    fn distance(&self, other: &Self) -> OrderedFloat {
        let first = (self.0 - other.0).pow(2);
        let second = (self.1 - other.1).pow(2);
        let third = (self.2 - other.2).pow(2);

        OrderedFloat(((first + second + third) as f64).sqrt())
    }
}

fn parse_junction_boxes(input: &str) -> Vec<JunctionBox> {
    input
        .lines()
        .map(|line| line.split(',').collect::<Vec<_>>())
        .flat_map(|num_strs| {
            let first = num_strs
                .first()
                .ok_or("invalid input")?
                .parse()
                .map_err(|_| "invalid number")?;
            let second = num_strs
                .iter()
                .nth(1)
                .ok_or("invalid input")?
                .parse()
                .map_err(|_| "invalid number")?;
            let third = num_strs
                .last()
                .ok_or("invalid input")?
                .parse()
                .map_err(|_| "invalid number")?;
            Ok::<JunctionBox, String>(JunctionBox(first, second, third))
        })
        .collect()
}

fn calculate_distances(boxes: &[JunctionBox]) -> Vec<((JunctionBox, JunctionBox), OrderedFloat)> {
    let mut distance_map = Vec::new();

    for idx in 0..boxes.len() {
        for other_idx in idx + 1..boxes.len() {
            let this = boxes[idx];
            let that = boxes[other_idx];
            let distance = this.distance(&that);
            distance_map.push(((this, that), distance));
        }
    }
    distance_map.sort_by_key(|(_, distance)| *distance);

    distance_map
}

fn solve_part1(input: &str, cnt_distances: usize) -> usize {
    let boxes = parse_junction_boxes(input);
    let distances = calculate_distances(&boxes);
    let mut circuits: Vec<Circuit> = Vec::new();

    for ((first_box, second_box), _) in distances.iter().take(cnt_distances) {
        // check if first belongs to a circuit, get it
        let first_circuit_idx = circuits
            .iter()
            .position(|circuit| circuit.contains_box(&first_box));
        let second_circuit_idx = circuits
            .iter()
            .position(|circuit| circuit.contains_box(&second_box));

        match (first_circuit_idx, second_circuit_idx) {
            // first already part of circuit - add second box to it
            (Some(idx), None) => {
                let circuit = circuits.get_mut(idx).expect("checked index");
                circuit.add_box(second_box.clone());
            }
            // second already part of circuit - add first box to it
            (None, Some(idx)) => {
                let circuit = circuits.get_mut(idx).expect("checked index");
                circuit.add_box(first_box.clone());
            }
            // none part of circuit - create a new one
            (None, None) => {
                let mut circuit = Circuit::new();
                circuit.add_box(first_box.clone());
                circuit.add_box(second_box.clone());
                circuits.push(circuit);
            }
            // both part of circuits - merge them
            (Some(first_idx), Some(second_idx)) => {
                // already in same circuit - bye
                if first_idx == second_idx {
                    continue
                }
                let first_circuit = circuits.get(first_idx).expect("checked index");
                let second_circuit = circuits.get(second_idx).expect("checked index");

                let merged_circuit = first_circuit.merge(second_circuit);
                
                let mut indeces = [first_idx, second_idx];
                indeces.sort();
                
                for idx in indeces.iter().rev() {
                    circuits.remove(*idx);
                }
                
                circuits.push(merged_circuit);
            }
        }
    }

    circuits.sort_by_key(|circuit| circuit.len());
    
    circuits
        .iter()
        .rev()
        .take(3)
        .map(|circuit| circuit.len())
        .product()
}

fn solve_part2(input: &str) -> usize {
    let boxes = parse_junction_boxes(input);
    let distances = calculate_distances(&boxes);
    let mut circuits: Vec<Circuit> = Vec::new();
    let mut solution = 0;

    for ((first_box, second_box), _) in distances {
        // check if first belongs to a circuit, get it
        let first_circuit_idx = circuits
            .iter()
            .position(|circuit| circuit.contains_box(&first_box));
        let second_circuit_idx = circuits
            .iter()
            .position(|circuit| circuit.contains_box(&second_box));

        match (first_circuit_idx, second_circuit_idx) {
            // first already part of circuit - add second box to it
            (Some(idx), None) => {
                let circuit = circuits.get_mut(idx).expect("checked index");
                circuit.add_box(first_box.clone());
                circuit.add_box(second_box.clone());
                // everything is connected
                if circuits.len() == 1 && circuits[0].len() == boxes.len() {
                    solution = (first_box.0 * second_box.0) as usize;
                    break
                }
            }
            // second already part of circuit - add first box to it
            (None, Some(idx)) => {
                let circuit = circuits.get_mut(idx).expect("checked index");
                circuit.add_box(first_box.clone());
                circuit.add_box(second_box.clone());
                // everything is connected
                if circuits.len() == 1 && circuits[0].len() == boxes.len() {
                    solution = (first_box.0 * second_box.0) as usize;
                    break
                }
            }
            // none part of circuit - create a new one
            (None, None) => {
                let mut circuit = Circuit::new();
                circuit.add_box(first_box.clone());
                circuit.add_box(second_box.clone());
                circuits.push(circuit);
            }
            // both part of circuits - merge them
            (Some(first_idx), Some(second_idx)) => {
                // already in same circuit - bye
                if first_idx == second_idx {
                    continue
                }
                
                let first_circuit = circuits.get(first_idx).expect("checked index");
                let second_circuit = circuits.get(second_idx).expect("checked index");

                let merged_circuit = first_circuit.merge(second_circuit);
                
                let mut indeces = [first_idx, second_idx];
                indeces.sort();
                
                for idx in indeces.iter().rev() {
                    circuits.remove(*idx);
                }
                
                circuits.push(merged_circuit);
                // everything is connected (all boxes included)
                if circuits.len() == 1 && circuits[0].len() == boxes.len() {
                    solution = (first_box.0 * second_box.0) as usize;
                    break
                }
            }
        }
        
    }   
    return solution
}

#[derive(Debug)]
struct Circuit(HashSet<JunctionBox>);


impl Circuit {
    fn new() -> Self {
        Self(HashSet::new())
        
    }

    fn contains_box(&self, other: &JunctionBox) -> bool {
        self.0.contains(other)
    }

    fn add_box(&mut self, jbox: JunctionBox) {
        self.0.insert(jbox);
    }

    fn merge(&self, other: &Self) -> Self {
        let mut new_boxes = HashSet::new();

        new_boxes.extend(self.0.clone());
        new_boxes.extend(other.0.clone());

        Self(new_boxes)
    }

    fn len(&self) -> usize {
        self.0.len()
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input = include_str!("../../assets/day08/test.txt");

        let boxes = parse_junction_boxes(input);
        assert_eq!(boxes.len(), 20);
        assert_eq!(boxes[0], JunctionBox(162, 817, 812));
        assert_eq!(boxes[19], JunctionBox(425, 690, 689));
    }

    #[test]
    fn test_calculating_distances() {
        let input = include_str!("../../assets/day08/test.txt");
        let boxes = parse_junction_boxes(input);

        let distances = calculate_distances(&boxes);

        let mut iter_distances = distances.iter();

        assert_eq!(
            (JunctionBox(162, 817, 812), JunctionBox(425, 690, 689)),
            iter_distances.next().unwrap().0
        );
        assert_eq!(
            (JunctionBox(162, 817, 812), JunctionBox(431, 825, 988)),
            iter_distances.next().unwrap().0
        );
    }

    #[test]
    fn test_solve_part1() {
        let input = include_str!("../../assets/day08/test.txt");
        let result = solve_part1(input, 10);

        assert_eq!(40, result);
    }
    
    #[test]
    fn test_solve_part2() {
        let input = include_str!("../../assets/day08/test.txt");
        let result = solve_part2(input);

        assert_eq!(25272, result);
    }
}
