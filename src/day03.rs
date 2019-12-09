use advent_of_code_2019::*;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::convert::TryFrom;
use std::fmt;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/day03.txt")?;
    let wires: Vec<WireTrack> = input.lines()
        .filter_map(|s| WireTrack::try_from(s).ok())
        .collect();

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
            .0.x
    }

    fn right(&self) -> i16 {
        self.map.iter()
            .max_by(|(point_a, _plot_a), (point_b, _plot_b)| point_a.x.cmp(&point_b.x))
            .expect("Empty WireTrack!")
            .0.y
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

        for x in self.left()..=right {
            for y in self.top()..=self.bot() {
                let point = Point { x, y, };

                if let Some(plot) = self.map.get(&point) {
                    write!(f, "{}", plot)?;
                } else {
                    write!(f, ".")?;
                }
            }

            if x == right {
                writeln!(f)?;
            }
        }

        Ok(())
    }
}

#[test]
fn wire_track() -> Result<()> {
    let track = WireTrack::try_from("R8,U5,L5,D3")?;
    println!("{}", track);
    Ok(())
}

impl TryFrom<&str> for WireTrack {
    type Error = BoxError;
    fn try_from(s: &str) -> Result<Self> {
        let mut wire_track = WireTrack::new();
        let mut point = Point::default(); // zero, zero
        for turn in s.split(',').filter_map(|s| Turn::try_from(s).ok()) {
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

        

        wire_track
    }
}

#[derive(Debug, Default, Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,)]
struct Point {
    x: i16,
    y: i16,
}

impl Point {
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
                y: self.y + turn.dist,
            },
            Direction::Down => Point {
                x: self.x,
                y: self.y - turn.dist,
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

trait NextPoint {
    fn next_point(&self) -> Point;
}

