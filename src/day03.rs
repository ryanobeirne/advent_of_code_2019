use advent_of_code_2019::*;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::convert::TryFrom;
use std::fmt;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/day03.txt")?;

    for line in input.lines() {
        let wire_track = WireTrack::try_from(line)?;
        println!("{}", wire_track);
    }


    answer!(03, 1, 42);
    Ok(())
}

#[derive(Debug, Clone)]
struct WireTrack {
    map: HashMap<Point, Plot>,
}

impl Default for WireTrack {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(Point::default(), Plot::Start);
        WireTrack { map }
    }
}

impl WireTrack {
    fn new() -> Self {
        WireTrack::default()
    }

    fn concat(&mut self, other: Self) {
        for (point, plot) in other.map.into_iter() {
            if let Some(_plot) = self.map.get(&point) {
                self.map.insert(point, Plot::Intersect);
            } else {
                self.map.insert(point, plot);
            }
        }
    }

    fn left(&self) -> i16 {
        self.map.iter()
            .min_by(|(point_a, _plot_a), (point_b, _plot_b)| point_a.x.cmp(&point_b.x))
            .expect("Empty WireTrack!")
            .0.x
    }

    fn top(&self) -> i16 {
        self.map.iter()
            .min_by(|(point_a, _plot_a), (point_b, _plot_b)| point_a.y.cmp(&point_b.y))
            .expect("Empty WireTrack!")
            .0.y
    }

    fn top_left(&self) -> Point {
        Point {
            x: self.left(),
            y: self.top(),
        }
    }

    fn bot(&self) -> i16 {
        self.map.iter()
            .max_by(|(point_a, _plot_a), (point_b, _plot_b)| point_a.y.cmp(&point_b.y))
            .expect("Empty WireTrack!")
            .0.y
    }

    fn right(&self) -> i16 {
        self.map.iter()
            .max_by(|(point_a, _plot_a), (point_b, _plot_b)| point_a.x.cmp(&point_b.x))
            .expect("Empty WireTrack!")
            .0.x
    }

    fn bot_right(&self) -> Point {
        Point {
            x: self.right(),
            y: self.bot(),
        }
    }
}

impl fmt::Display for WireTrack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let right = self.right();

        for y in self.top()..=self.bot() {
            for x in self.left()..=right {
                let point = Point { x, y, };

                if let Some(plot) = self.map.get(&point) {
                    write!(f, "{}", plot)?;
                } else {
                    write!(f, ".")?;
                }

                if x >= right {
                    write!(f, "\n")?;
                }

            }
        }

        Ok(())
    }
}

#[test]
fn wire_track() -> Result<()> {
    let track = WireTrack::try_from("R8,U5,L5,D3")?;
    println!("WIRETRACK:\n{}", track);
    Ok(())
}

impl TryFrom<&str> for WireTrack {
    type Error = BoxError;
    fn try_from(s: &str) -> Result<Self> {
        let mut wire_track = WireTrack::new();
        let mut point = Point::default(); // zero, zero
        for turn in s.split(',')
            .filter_map(|s| Turn::try_from(s).ok())
        {
            let vector = Vector::new(point, turn);
            let track = WireTrack::from(vector);
            wire_track.concat(track);
            point = vector.end();
        }

        Ok(wire_track)
    }
}

impl From<Vector> for WireTrack {
    fn from(vector: Vector) -> WireTrack {
        let mut wire_track = WireTrack::new();

        let plot = match vector.turn.dir {
            Direction::Left | Direction::Right => Plot::Horiz,
            Direction::Down | Direction::Up => Plot::Vert,
        };

        for point in vector.points() {
            wire_track.map.insert(point, plot);
        }

        wire_track
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
    fn new(x: i16, y: i16) -> Self {
        Point { x, y }
    }

    fn turn(&self, turn: Turn) -> Self {
        match turn.dir {
            Direction::Left => Point {
                x: self.x - turn.dist,
                y: self.y,
            },
            Direction::Right => Point {
                x: self.x + turn.dist,
                y: self.y,
            },
            Direction::Up => Point {
                x: self.x,
                y: self.y - turn.dist,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y + turn.dist,
            }
        }
    }

}

impl Add for Point {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Point {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Point {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Point {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}


/// ```rust
/// struct Vector {
///     start: Point,
///     turn: Turn,
/// }
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,)]
struct Vector {
    start: Point,
    turn: Turn,
}

impl Vector {
    fn new(start: Point, turn: Turn) -> Vector {
        Vector { start, turn, }
    }

    fn len(&self) -> i16 {
        self.turn.dist
    }

    fn end(&self) -> Point {
        self.start.turn(self.turn)
    }

    fn points(&self) -> VectorPoints {
        self.into_iter()
    }
}

struct VectorPoints {
    points: Vec<Point>,
    index: usize,
    len: usize,
}

impl Iterator for VectorPoints {
    type Item = Point;
    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.len {
            self.index += 1;
            Some(self.points[self.index - 1])
        } else {
            None
        }
    }
}

impl IntoIterator for Vector {
    type Item = Point;
    type IntoIter = VectorPoints;
    fn into_iter(self) -> Self::IntoIter {
        let mut points = Vec::new();
        let mut len = 0;

        let mut start = self.start;
        let end = self.end();
        let dir = self.turn.dir;

        while start != end {
            start = start.turn(Turn::new(dir, 1));
            points.push(start);
            len += 1;
        }

        VectorPoints {
            points,
            len,
            index: 0,
        }
    }
}

#[test]
fn vector_end() {
    let vector = Vector::new(
        Point::default(),
        Turn::new(Direction::Right, 5),
    );
    assert_eq!(vector.end(), Point::new(5, 0));

    let vector = Vector::new(
        Point::default(),
        Turn::new(Direction::Left, 5),
    );
    assert_eq!(vector.end(), Point::new(-5, 0));

    let vector = Vector::new(
        Point::default(),
        Turn::new(Direction::Up, 5),
    );
    assert_eq!(vector.end(), Point::new(0, -5));

    let vector = Vector::new(
        Point::default(),
        Turn::new(Direction::Down, 5),
    );
    assert_eq!(vector.end(), Point::new(0, 5));
}


#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,)]
enum Plot {
    Start,
    Horiz,
    Vert,
    Intersect,
}

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",  match self {
            Plot::Horiz => '-',
            Plot::Vert => '|',
            Plot::Intersect => '+',
            Plot::Start => 'O',
        })
    }
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,)]
enum Direction {
    Left,
    Right,
    Up,
    Down,
}

#[derive(Debug, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,)]
struct Turn {
    dir: Direction,
    dist: i16,
}

impl Turn {
    fn new(dir: Direction, dist: i16) -> Self {
        Turn { dir, dist }
    }
}

impl TryFrom<&str> for Turn {
    type Error = BoxError;
    fn try_from(s: &str) -> Result<Self> {
        let mut chars = s.chars();
        let first = chars.next();
        let dist = chars.collect::<String>().parse::<i16>()?;

        if let Some(f) = first {
            let dir = match f {
                'L' => Direction::Left,
                'R' => Direction::Right,
                'U' => Direction::Up,
                'D' => Direction::Down,
                _ => return ioerr!(),
            };
            Ok(Turn { dir, dist, })
        } else {
            ioerr!()
        }
    }       
}

trait Distance {
    fn distance(&self, other: &Self) -> i16;
}

impl Distance for Point {
    fn distance(&self, other: &Self) -> i16 {
        (self.x - other.x).abs() + (self.y - other.y).abs()
    }
}

impl Distance for Vector {
    fn distance(&self, other: &Self) -> i16 {
        self.start.distance(&other.start)
    }
}

