fn main() {
    let input = include_str!("../../assets/day09/input.txt");
    let part1 = solve_part1(input);

    println!("Part 1: {}", part1);

    let part2 = solve_part2(input);

    println!("Part 2: {}", part2);
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Point(usize, usize);

impl Point {
    fn x(&self) -> usize {
        self.0
    }

    fn y(&self) -> usize {
        self.1
    }

    fn trace_right(&self, line: &Line) -> bool {
        if line.is_horizontal() {
            return false;
        }

        self.y() > line.start().y() && self.y() <= line.end().y() && line.start().x() > self.x()
    }
    
    fn on_line(&self, line: &Line) -> bool {
        if line.is_horizontal() {
            return self.x() >= line.start().x() && self.x() <= line.end().x() && line.start().y() == self.y()
        }

        if line.is_vertical() {
            return self.y() >= line.start().y() && self.y() <= line.end().y() && line.start().x() == self.x()
        }

        false
    }
}

#[derive(Debug)]
struct Square(Point, Point);

impl Square {
    fn length(&self) -> usize {
        self.0.0.abs_diff(self.1.0) + 1
    }

    fn width(&self) -> usize {
        self.0.1.abs_diff(self.1.1) + 1
    }

    fn area(&self) -> usize {
        self.length() * self.width()
    }

    fn corners(&self) -> Vec<Point> {
        if self.width() == 1 || self.length() == 1 {
            return vec![self.0, self.1];
        }

        let first_mirrored = Point(self.1.x(), self.0.y());
        let second_mirrored = Point(self.0.x(), self.1.y());

        vec![self.0, first_mirrored, self.1, second_mirrored]
    }

    fn lines(&self) -> Vec<Line> {
        if self.corners().len() == 2 {
            return vec![build_line(self.0, self.1)];
        }
        
        let corners = self.corners();
        
        vec![build_line(corners[0], corners[1]), build_line(corners[1], corners[2]), build_line(corners[2], corners[3]), build_line(corners[3], corners[0])]
    }
}

fn parse_points(input: &str) -> Vec<Point> {
    input
        .lines()
        .flat_map(|line| line.split_once(','))
        .flat_map(|(first, second)| {
            Ok::<Point, String>(Point(
                first.parse().map_err(|_| "invalid number")?,
                second.parse().map_err(|_| "invalid number")?,
            ))
        })
        .collect()
}

struct Shape(Vec<Line>);

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Line(Point, Point);

impl Line {
    fn is_vertical(&self) -> bool {
        self.0.x() == self.1.x()
    }

    fn is_horizontal(&self) -> bool {
        self.0.y() == self.1.y()
    }

    fn start(&self) -> Point {
        self.0
    }

    fn end(&self) -> Point {
        self.1
    }

    fn intersects(&self, other: &Line) -> bool {
        if self.is_horizontal() && other.is_horizontal()
            || self.is_vertical() && other.is_vertical()
        {
            return false;
        }

        if self.is_horizontal() && other.is_vertical() {
            return self.start().y() > other.start().y()
                && self.start().y() < other.end().y()
                && other.start().x() > self.start().x()
                && other.start().x() < self.end().x();
        }

        if self.is_vertical() && other.is_horizontal() {
            return other.start().y() > self.start().y()
                && other.start().y() < self.end().y()
                && self.start().x() > other.start().x()
                && self.start().x() < other.end().x();
        }

        false
    }
}

impl Shape {
    fn get_horizontal_lines(&self) -> Vec<Line> {
        self.0
            .iter()
            .filter(|line| line.is_horizontal())
            .cloned()
            .collect()
    }

    fn get_vertical_lines(&self) -> Vec<Line> {
        self.0
            .iter()
            .filter(|line| line.is_vertical())
            .cloned()
            .collect()
    }

    fn point_within(&self, point: &Point) -> bool {
        
        if self.0.iter().any(|line| point.on_line(line)) {
            return true
        }
        
        
        !self
            .get_vertical_lines()
            .iter()
            .filter(|line| point.trace_right(line))
            .count()
            .is_multiple_of(2)
    }
}

fn calculate_possible_squares(points: &[Point]) -> Vec<Square> {
    let mut squares = Vec::new();
    for idx in 0..points.len() {
        for other_idx in idx + 1..points.len() {
            let this = points[idx];
            let that = points[other_idx];
            squares.push(Square(this, that))
        }
    }

    squares
}

fn build_line(a: Point, b: Point) -> Line {
    let horizontal = a.y() == b.y();
    if horizontal {
        let (start, end) = if a.x() < b.x() { (a, b) } else { (b, a) };
        Line(start, end)
    } else {
        let (start, end) = if a.y() < b.y() { (a, b) } else { (b, a) };
        Line(start, end)
    }
}

fn build_shape(points: &[Point]) -> Shape {
    let mut lines = Vec::new();

    for idx in 0..points.len() {
        let this = points[idx];
        let that = if idx < points.len() - 1 {
            points[idx + 1]
        } else {
            points[0]
        };
        
        let line = build_line(this, that);

        lines.push(line);
    }

    Shape(lines)
}

fn solve_part1(input: &str) -> usize {
    let points = parse_points(input);
    let mut squares = calculate_possible_squares(&points);
    squares.sort_by_key(|square| square.area());

    dbg!(squares.len());

    squares
        .iter()
        .rev()
        .take(1)
        .map(|square| square.area())
        .sum()
}

fn solve_part2(input: &str) -> usize {
    let points = parse_points(input);
    let shape = build_shape(&points);
    let squares = calculate_possible_squares(&points);
    let mut squares: Vec<_> = squares
        .iter()
        .filter(|square| {
            let corners = square.corners();
            if corners.len() == 2 {
                return true;
            }
            let second_within = shape.point_within(&corners[1]);
            let fourth_within = shape.point_within(&corners[3]);
            second_within && fourth_within
        })
        .filter(|square| square.lines().iter().all(|line| {
            if line.is_horizontal() {
                return !shape.get_vertical_lines().iter().any(|other| line.intersects(other))
            }
            
            if line.is_vertical() {
                return !shape.get_horizontal_lines().iter().any(|other| line.intersects(other))
            }
            
            false
        }))
        .collect();

    squares.sort_by_key(|square| square.area());
    dbg!(squares.len());

    squares
        .iter()
        .rev()
        .take(1)
        .map(|square| square.area())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parsing() {
        let input = include_str!("../../assets/day09/test.txt");
        let points = parse_points(input);
        assert_eq!(points.len(), 8);

        assert_eq!(points[0], Point(7, 1));
        assert_eq!(points[1], Point(11, 1));
        assert_eq!(points[2], Point(11, 7));
        assert_eq!(points[3], Point(9, 7));
        assert_eq!(points[4], Point(9, 5));
        assert_eq!(points[5], Point(2, 5));
        assert_eq!(points[6], Point(2, 3));
        assert_eq!(points[7], Point(7, 3));
    }

    #[test]
    fn test_calculate_area() {
        let square = Square(Point(2, 5), Point(11, 1));
        assert_eq!(square.area(), 50);

        let square = Square(Point(7, 1), Point(11, 7));
        assert_eq!(square.area(), 35);

        let square = Square(Point(7, 3), Point(2, 3));
        assert_eq!(square.area(), 6);
    }

    #[test]
    fn test_part1() {
        let input = include_str!("../../assets/day09/test.txt");
        assert_eq!(solve_part1(input), 50);
    }

    #[test]
    fn test_build_shape() {
        let input = include_str!("../../assets/day09/test.txt");
        let points = parse_points(input);
        let shape = build_shape(&points);

        assert_eq!(shape.0.len(), 8);
        assert_eq!(shape.0[0], Line(Point(7, 1), Point(11, 1)));
        assert_eq!(shape.0[1], Line(Point(11, 1), Point(11, 7)));
        assert_eq!(shape.0[2], Line(Point(9, 7), Point(11, 7)));
        assert_eq!(shape.0[3], Line(Point(9, 5), Point(9, 7)));
        assert_eq!(shape.0[4], Line(Point(2, 5), Point(9, 5)));
        assert_eq!(shape.0[5], Line(Point(2, 3), Point(2, 5)));
        assert_eq!(shape.0[6], Line(Point(2, 3), Point(7, 3)));
        assert_eq!(shape.0[7], Line(Point(7, 1), Point(7, 3)));

        assert_eq!(shape.get_horizontal_lines().len(), 4);
        assert_eq!(shape.get_vertical_lines().len(), 4);
    }

    #[test]
    fn test_get_corners() {
        let square = Square(Point(7, 3), Point(2, 5));
        let corners = square.corners();
        assert_eq!(corners.len(), 4);

        assert_eq!(corners[0], Point(7, 3));
        assert_eq!(corners[1], Point(2, 3));
        assert_eq!(corners[2], Point(2, 5));
        assert_eq!(corners[3], Point(7, 5));

        let square = Square(Point(7, 3), Point(11, 1));
        let corners = square.corners();
        assert_eq!(corners.len(), 4);

        assert_eq!(corners[0], Point(7, 3));
        assert_eq!(corners[1], Point(11, 3));
        assert_eq!(corners[2], Point(11, 1));
        assert_eq!(corners[3], Point(7, 1));

        let square = Square(Point(7, 1), Point(11, 1));
        let corners = square.corners();
        assert_eq!(corners.len(), 2);

        assert_eq!(corners[0], Point(7, 1));
        assert_eq!(corners[1], Point(11, 1));

        let square = Square(Point(11, 1), Point(11, 7));
        let corners = square.corners();
        assert_eq!(corners.len(), 2);

        assert_eq!(corners[0], Point(11, 1));
        assert_eq!(corners[1], Point(11, 7));
    }
    
    #[test]
    fn test_intersect() {
        let this = build_line(Point(8,6), Point(10,6));
        let that = build_line(Point(9,5), Point(9,7));
        
        assert!(this.intersects(&that));
        
        let this = build_line(Point(8,4), Point(10,4));
        let that = build_line(Point(9,5), Point(9,7));
        assert!(!this.intersects(&that));
        let this = build_line(Point(3,6), Point(8,6));
        let that = build_line(Point(9,5), Point(9,7));
        assert!(!this.intersects(&that));
        let this = build_line(Point(8,5), Point(10,5));
        let that = build_line(Point(9,5), Point(9,7));
        assert!(!this.intersects(&that));
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../../assets/day09/test.txt");
        let part2 = solve_part2(input);

        assert_eq!(24, part2);
    }
}
