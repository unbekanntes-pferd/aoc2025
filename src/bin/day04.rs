use std::collections::HashSet;

fn main() {
    let input = include_str!("../../assets/day04/input.txt");
    let mut grid: Grid = input.into();

    let result = grid.get_removable().iter().count();

    println!("Part 1: {result}");
    
    let mut total = 0;
    
    loop {
        let removable_points = grid.get_removable();
        if removable_points.len() == 0 {
            break
        }
        total += removable_points.len();
        grid.remove(removable_points);
    }
    
    println!("Part 2: {total}");
    
}

struct Grid(HashSet<Point>);

impl Grid {
    fn get_removable(&self) -> Vec<Point> {
               self
                .0
                .iter()
                .filter(|Point(x, y)| {
                    let left = Point(x - 1, *y);
                    let right = Point(x + 1, *y);
                    let top_left = Point(x - 1, y - 1);
                    let top = Point(*x, y - 1);
                    let top_right = Point(x + 1, y - 1);
                    let bottom_left = Point(x - 1, y + 1);
                    let bottom = Point(*x, y + 1);
                    let bottom_right = Point(x + 1, y + 1);
                    let neighbors = vec![
                        left,
                        right,
                        top,
                        bottom,
                        top_left,
                        top_right,
                        bottom_right,
                        bottom_left,
                    ];

                    let cnt_neighbors = neighbors
                        .iter()
                        .filter(|point| self.0.get(point).is_some())
                        .count();

                    cnt_neighbors < 4
                })
                .map(|point| Point(point.0, point.1))
                .collect()
    }
    
    fn remove(&mut self, points: Vec<Point>) {
        for point in points {
            self.0.remove(&point);
        }
    }
}

impl From<&str> for Grid {
    fn from(value: &str) -> Self {
        let points = value
            .lines()
            .enumerate()
            .map(|(row_idx, row)| {
                row.chars()
                    .enumerate()
                    .filter(|(_, c)| *c == '@')
                    .map(|(col_idx, _)| Point(col_idx as isize, row_idx as isize))
                    .collect::<Vec<_>>()
            })
            .flatten()
            .collect();

        Self(points)
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Point(isize, isize);


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_part_1() {

        let grid = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        let grid: Grid = grid.into();

        let result = grid.get_removable().iter().count();

        assert_eq!(13, result);
    }

    #[test]
    fn test_part2() {
        let grid = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";

        let mut grid: Grid = grid.into();
        let mut total = 0;
        
        loop {
            let removable_points = grid.get_removable();
            if removable_points.len() == 0 {
                break
            }
            total += removable_points.len();
            grid.remove(removable_points);
        }
        
        assert_eq!(43, total);

    }
}
