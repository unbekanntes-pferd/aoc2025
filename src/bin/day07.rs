use std::collections::HashMap;

fn main() {
    let input = include_str!("../../assets/day07/input.txt");
    
    let grid = parse_grid(input);
    let part1 = solve_part1(grid);
    
    println!("Part 1: {}", part1);
}

#[derive(Debug, PartialEq, Eq, Hash)]
enum Field {
    Beam,
    Splitter,
    Empty,
}

impl TryFrom<char> for Field {
    type Error = String;
    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Field::Beam),
            '^' => Ok(Field::Splitter),
            '.' => Ok(Field::Empty),
            _ => Err("invalid grid".to_string()),
        }
    }
}

struct Grid(HashMap<Coord, Field>);
impl From<HashMap<Coord, Field>> for Grid {
    fn from(value: HashMap<Coord, Field>) -> Self {
        Self(value)
    }
}

impl Grid {
    fn done(&self) -> bool {
        self.0.iter().all(|(_, field)| field != &Field::Beam)
    }
    
    fn get_max_y(&self) -> usize {
        self.0.iter().map(|(coord, _)| coord.1).max().unwrap_or(0)
    }
    
    fn get_max_x(&self) -> usize {
        self.0.iter().map(|(coord, _)| coord.0).max().unwrap_or(0)
    }

    fn get_beams(&self) -> Vec<Coord> {
        self.0
            .iter()
            .filter(|(_, field)| *field == &Field::Beam)
            .map(|(coord, _)| coord)
            .cloned()
            .collect()
    }

    fn move_beams(&mut self) -> usize {
        let beams = self.get_beams();
        let mut count_split = 0;
        
        for beam_coord in beams {
            let next_coord = Coord(beam_coord.0, beam_coord.1 + 1);
            
            // first clear current field
            if let Some(curr_field) = self.0.get_mut(&beam_coord) {
                *curr_field = Field::Empty
            }
   
            // check what next field is
            if let Some(next_field) = self.0.get_mut(&next_coord) {
                match next_field {
                    // if empty, fill with beam
                    Field::Empty => {
                        *next_field = Field::Beam;
                    },
                    Field::Splitter => {
                        count_split += 1;
                        let left_possible = next_coord.0 > 0;
                        let right_possible = next_coord.0 < self.get_max_x();
                        let split_left_coord = Coord(next_coord.0 - 1, next_coord.1);
                        let split_right_coord = Coord(next_coord.0 + 1, next_coord.1);
                        
                        if left_possible {
                            self.0.insert(split_left_coord, Field::Beam);
                        }
                        
                        if right_possible {
                            self.0.insert(split_right_coord, Field::Beam);
                        }
                    },
                    Field::Beam => ()
                }
            }
            
            
        }
        count_split
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Coord(usize, usize);

fn parse_grid(input: &str) -> Grid {
    input
        .lines()
        .enumerate()
        .map(|(y_idx, row)| {
            row.chars()
                .enumerate()
                .flat_map(|(x_idx, c)| {
                    let coord = Coord(x_idx, y_idx);
                    let field: Field = c.try_into()?;
                    Ok::<_, String>((coord, field))
                })
                .collect::<Vec<_>>()
        })
        .flatten()
        .fold(HashMap::new(), |mut field_map, (coord, field)| {
            field_map.insert(coord, field);
            field_map
        })
        .into()
}

fn solve_part1(mut grid: Grid) -> usize {
    let mut count = 0;
    
    loop {
        if grid.done() {
            break
        }
        
        count += grid.move_beams();
    }
    
    count
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_parsing() {
        let input = "...S...\n...^...\n..^.^..";

        let grid = parse_grid(input);

        // first row
        assert_eq!(grid.0.get(&Coord(0, 0)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(1, 0)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(2, 0)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(3, 0)), Some(&Field::Beam));
        assert_eq!(grid.0.get(&Coord(4, 0)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(5, 0)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(6, 0)), Some(&Field::Empty));

        // second row
        assert_eq!(grid.0.get(&Coord(0, 1)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(1, 1)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(2, 1)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(3, 1)), Some(&Field::Splitter));
        assert_eq!(grid.0.get(&Coord(4, 1)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(5, 1)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(6, 1)), Some(&Field::Empty));

        // third row
        assert_eq!(grid.0.get(&Coord(0, 2)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(1, 2)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(2, 2)), Some(&Field::Splitter));
        assert_eq!(grid.0.get(&Coord(3, 2)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(4, 2)), Some(&Field::Splitter));
        assert_eq!(grid.0.get(&Coord(5, 2)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(6, 2)), Some(&Field::Empty));
    }
    
    #[test]
    fn test_movement() {
        let input = "...S...\n...^...\n..^.^..";
        let mut grid = parse_grid(input);
        
        let cnt = grid.move_beams();
        assert_eq!(grid.0.get(&Coord(3, 0)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(2, 1)), Some(&Field::Beam));
        assert_eq!(grid.0.get(&Coord(4, 1)), Some(&Field::Beam));
        assert_eq!(cnt, 1);
        
        let cnt = grid.move_beams();
        assert_eq!(grid.0.get(&Coord(2, 1)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(4, 1)), Some(&Field::Empty));
        assert_eq!(cnt, 2);
        assert_eq!(grid.0.get(&Coord(1, 2)), Some(&Field::Beam));
        assert_eq!(grid.0.get(&Coord(3, 2)), Some(&Field::Beam));
        assert_eq!(grid.0.get(&Coord(5, 2)), Some(&Field::Beam));
        
        let cnt = grid.move_beams();
        assert_eq!(cnt, 0);
        assert!(grid.done());
        assert_eq!(grid.0.get(&Coord(1, 2)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(3, 2)), Some(&Field::Empty));
        assert_eq!(grid.0.get(&Coord(5, 2)), Some(&Field::Empty));
    }
    
    #[test]
    fn test_part1() {
        let input = include_str!("../../assets/day07/test.txt");
        
        let grid = parse_grid(input);
        let count_splits = solve_part1(grid);
        
        assert_eq!(21, count_splits);
    }
}
