use advent_of_code_2019::*;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::convert::TryFrom;
use std::fmt;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/day03.txt")?;

    let tracks = input.lines()
        .filter_map(|line| WireTrack::try_from(line).ok())
        .collect::<Vec<_>>();

    assert_eq!(tracks.len(), 2, "Looking for 2 Wire Tracks!");

    let min_dist = tracks[0].closest_intersection(&tracks[1]);

    answer!(03, 1, min_dist);
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

    fn closest_intersection(&self, other: &Self) -> i16 {
        let mut matches = HashMap::new();
        for point in self.map.keys() {
            if point == &Point::default() { continue }
            if let Some(_plot) = other.map.get(point) {
                matches.insert(point, point.distance(&Point::default()));
            }
        }

        matches.into_iter()
            .min_by(|(_pa, da), (_pb, db)| da.cmp(db))
            .expect("NO MATCHES FOUND")
            .1
    }

    fn concat(&mut self, other: Self) {
        for (point, plot) in other.map.into_iter() {
            let checkplot = if point == Point::default() {
                Plot::Start
            } else if let Some(_plot) = self.map.get(&point) {
               Plot::Intersect
            } else {
                plot
            };

            self.map.insert(point, checkplot);
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

    #[allow(unused)]
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

    #[allow(unused)]
    fn bot_right(&self) -> Point {
        Point {
            x: self.right(),
            y: self.bot(),
        }
    }
}

impl fmt::Display for WireTrack {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let right = self.right() + 1;

        for y in (self.top() - 1)..=(self.bot() + 1) {
            for x in (self.left() - 1)..=right {
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
    let mut track1 = WireTrack::try_from("R8,U5,L5,D3")?;
    let track2 = WireTrack::try_from("U7,R6,D4,L4")?;
    println!("WIRETRACK:\n{}", &track1);
    println!("WIRETRACK:\n{}", &track2);
    track1.concat(track2);
    println!("WIRETRACK:\n{}", &track1);
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

        let mut points = vector.points().peekable();
        while let Some(point) = points.next() {
            if let None = points.peek() {
                wire_track.map.insert(point, Plot::Turn);
            } else {
                wire_track.map.insert(point, plot);
            }
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
    #[cfg(test)]
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

    #[allow(unused)]
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
fn points_iter() {
    let vector = Vector::new(Point::new(0,0), Turn::new(Direction::Right, 5));
    for point in vector.points() {
        dbg!(point);
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
    Turn,
    Intersect,
}

impl fmt::Display for Plot {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}",  match self {
            Plot::Horiz => '-',
            Plot::Vert => '|',
            Plot::Intersect => 'X',
            Plot::Turn => '+',
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
                _ => return ioerr!(f),
            };
            Ok(Turn { dir, dist, })
        } else {
            ioerr!(s.to_owned())
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

