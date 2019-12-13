use advent_of_code_2019::*;
use std::collections::HashMap;
use std::ops::{Add, Sub};
use std::convert::TryFrom;
use std::fmt;
use std::iter::FromIterator;

fn main() -> Result<()> {
    let input = std::fs::read_to_string("input/day03.txt")?;

    let tracks = input.lines()
        .filter_map(|line| WireTrack::try_from(line).ok())
        .collect::<Vec<_>>();

    assert_eq!(tracks.len(), 2, "Looking for 2 Wire Tracks!");

    let min_dist = tracks[0].closest_intersection_distance(&tracks[1]);
    answer!(03, 1, min_dist);

    let min_steps = tracks[0].minimum_steps_to_intersect(&tracks[1]).expect("No intersections!");
    answer!(03, 2, min_steps);
    Ok(())
}

#[derive(Debug, Clone)]
struct WireTrack {
    map: HashMap<Point, Plot>,
    vec: Vec<Vector>,
}

impl Default for WireTrack {
    fn default() -> Self {
        let mut map = HashMap::new();
        map.insert(Point::default(), Plot::Start);
        WireTrack { map, vec: Vec::new() }
    }
}

impl WireTrack {
    fn new() -> Self {
        WireTrack::default()
    }

    fn intersections(&self, other: &Self) -> HashMap<Point, i16> {
        let mut matches = HashMap::new();
        for point in self.map.keys() {
            if point == &Point::default() { continue }
            if let Some(_plot) = other.map.get(point) {
                matches.insert(*point, point.distance(&Point::default()));
            }
        }
        matches
    }

    fn closest_intersection_distance(&self, other: &Self) -> i16 {
        self.intersections(other).into_iter()
            .min_by(|(_pa, da), (_pb, db)| da.cmp(db))
            .expect("NO MATCHES FOUND")
            .1
    }

    fn minimum_steps_to_intersect(&self, other: &Self) -> Option<usize> {
        self.intersections(other)
                .iter()
                .map(|(intersection, _len)| {
                    self.steps_to_point(intersection).expect("Point doesn't exist!") +
                    other.steps_to_point(intersection).expect("Point doesn't exist!")
                })
                .min()
    }

    fn steps_to_point(&self, point: &Point) -> Option<usize> {
        Some(self.points()
            .enumerate()
            .find(|(_i, p)| p == point)?
            .0 + 1
        )
    }

    fn concat(&mut self, other: Self) -> Option<Point> {
        for vector in other.vec {
            self.attach_vector(vector)?;
        }

        Some(self.end())
    }

    /// The last point in a WireTrack
    fn end(&self) -> Point {
        if self.vec.is_empty() {
            Point::default()
        } else {
            self.vec.get(self.vec.len() - 1)
                .expect("Couldn't get Point from WireTrack")
                .end()
        }
    }
    
    /// Attach a Vector to the end of a WireTrack.
    /// Returns None if the end of the current WireTrack is not the same point as the start of the
    /// new Vector.
    fn attach_vector(&mut self, vector: Vector) -> Option<Point> {
        if self.end() != vector.start {
            return None;
        }

        let plot = Plot::from(vector.turn.dir);

        let mut points = vector.points();
        if let Some(point) = points.next() {
            self.map.insert(point, Plot::Turn);
        }

        for point in points {
            self.map.insert(point, plot);
        }

        self.vec.push(vector);

        Some(vector.end())
    }

    fn points(&self) -> VectorPoints {
        self.into_iter()
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
fn vector_points() {
    use std::iter::once;
    let vector1 = Vector::new(Point::new(0,0), Turn::new(Direction::Right, 5));
    let vector2 = Vector::new(vector1.end(), Turn::new(Direction::Down, 5));

    let wire_track = once(vector1).chain(once(vector2)).collect::<WireTrack>();
    assert_eq!(wire_track.steps_to_point(&Point::new(5,5)).unwrap(), 10);
}

#[test]
fn steps_to_point() -> Result<()> {
    let track1 = WireTrack::try_from("R8,U5,L5,D3")?;
    let track2 = WireTrack::try_from("U7,R6,D4,L4")?;

    dbg!(track1.points());

    let steps_to_point1 = track1.steps_to_point(&Point::new(6, -5)).unwrap();
    let steps_to_point2 = track2.steps_to_point(&Point::new(6, -5)).unwrap();

    assert_eq!(steps_to_point1, 15);
    assert_eq!(steps_to_point2, 15);

    Ok(())
}

#[test]
fn min_steps() -> Result<()> {
    let track1 = WireTrack::try_from("R75,D30,R83,U83,L12,D49,R71,U7,L72")?;
    let track2 = WireTrack::try_from("U62,R66,U55,R34,D71,R55,D58,R83")?;
    let mut track1_clone = track1.clone();
    track1_clone.concat(track2.clone());
    println!("TRACK1:\n{}", &track1);
    println!("TRACK1:\n{}", &track2);
    println!("TRACK1+2:\n{}", track1_clone);

    assert_eq!(track1.minimum_steps_to_intersect(&track2).unwrap(), 610);

    let track3 = WireTrack::try_from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")?;
    let track4 = WireTrack::try_from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")?;
    println!("TRACK1:\n{}", &track3);
    println!("TRACK1:\n{}", &track4);

    assert_eq!(track3.minimum_steps_to_intersect(&track4).unwrap(), 410);

    Ok(())
}

#[test]
fn non_repeating_points() -> Result<()> {
    let track1 = WireTrack::try_from("R75,D30,R83,U83,L12,D49,R71,U7,L72")?.points().peekable();
    let track2 = WireTrack::try_from("U62,R66,U55,R34,D71,R55,D58,R83")?.points().peekable();
    let track3 = WireTrack::try_from("R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51")?.points().peekable();
    let track4 = WireTrack::try_from("U98,R91,D20,R16,D67,R40,U7,R15,U6,R7")?.points().peekable();

    for track in [track1, track2, track3, track4].iter_mut() {
        while let Some(next) = track.next() {
            if let Some(peek) = track.peek() {
                assert_ne!(&next, peek, "Consecutive points repeat!");
            }
        }
    }

    Ok(())
}

#[test]
fn intersections() -> Result<()> {
    let track1 = WireTrack::try_from("R8,U5,L5,D3")?;
    let track2 = WireTrack::try_from("U7,R6,D4,L4")?;

    let intersections = track1.intersections(&track2);
    assert_eq!(intersections.len(), 2);

    let min_steps = track1.minimum_steps_to_intersect(&track2).expect("No intersections");
    assert_eq!(min_steps, 30);

    Ok(())
}

#[test]
fn wire_track() -> Result<()> {
    let mut track1 = WireTrack::try_from("R8,U5,L5,D3")?;
    let track2 = WireTrack::try_from("U7,R6,D4,L4")?;
    println!("WIRETRACK:\n{}", &track1);
    println!("WIRETRACK:\n{}", &track2);
    track1.concat(track2);
    println!("WIRETRACK:\n{}", &track1);

    assert_eq!(
        track1.clone().points(),
        track1.vec.into_iter()
            .flat_map(|v| v.points())
            .collect()
        );
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

        wire_track.vec.push(vector.clone());

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

#[derive(Debug, Clone, Eq, PartialEq, Ord, PartialOrd, Hash,)]
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

impl IntoIterator for &Vector {
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

impl IntoIterator for &WireTrack {
    type Item = Point;
    type IntoIter = VectorPoints;
    fn into_iter(self) -> Self::IntoIter {
        self.vec.iter()
            .flat_map(|v| v.points())
            .collect()
    }
}

impl FromIterator<Point> for VectorPoints {
    fn from_iter<I: IntoIterator<Item=Point>>(iter: I) -> Self {
        let points: Vec<Point> = iter.into_iter().collect();
        let len = points.len();
        VectorPoints {
            points,
            len,
            index: 0,
        }
    }
}

impl FromIterator<Vector> for WireTrack {
    fn from_iter<I: IntoIterator<Item=Vector>>(iter: I) -> Self {
        let mut wire_track = WireTrack::default();
        for vector in iter.into_iter() {
            wire_track.concat(WireTrack::from(vector));
        }

        wire_track
    }
}

#[test]
fn points_iter() {
    let vector = Vector::new(Point::new(0,0), Turn::new(Direction::Right, 5));
    assert_eq!(vector.points().count(), 5);
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

impl From<Direction> for Plot {
    fn from(dir: Direction) -> Plot {
        match dir {
            Direction::Left | Direction::Right => Plot::Horiz,
            Direction::Up   | Direction::Down  => Plot::Vert,
        }
    }
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

