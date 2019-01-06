#[macro_use]
extern crate scan_fmt;

use std::cmp;
use std::cmp::Ordering;
use std::collections::LinkedList;
use std::fmt;
use std::num::Wrapping;
use std::string::String;

#[derive(Hash, Eq, PartialEq, PartialOrd, Debug, Clone, Copy)]
pub struct Point {
    x: i64,
    y: i64,
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Rectangle {
    tl: Point, // Top left anchor
    tr: Point,
    br: Point,
    bl: Point, // Bottom right anchor
}

#[derive(Hash, Eq, PartialEq, Debug, Clone, Copy)]
pub struct Claim {
    pub id: u32,
    rectangle: Rectangle,
}

impl Rectangle {
    pub fn intersects(&self, other: &Rectangle) -> bool {
        // Do any of our lines intersect their lines?
        // Checking horizontal vs vertical.
        let has_lines_intersection = lines_intersect(self.top(), other.left())
            || lines_intersect(self.top(), other.right())
            || lines_intersect(self.bottom(), other.left())
            || lines_intersect(self.bottom(), other.right())
            || lines_intersect(self.left(), other.top())
            || lines_intersect(self.left(), other.bottom())
            || lines_intersect(self.right(), other.top())
            || lines_intersect(self.right(), other.bottom());

        if has_lines_intersection {
            return true;
        }

        // Does one rectangle entirely encompass the other?
        // if (self.tl <= other.tl && self.br >= other.br) || (other.tl <= self.tl && other.br >= self.br) {
        //     return true;
        // }

        return false;
    }
    pub fn width(&self) -> i64 {
        return self.br.x - self.tl.x;
    }
    pub fn height(&self) -> i64 {
        return self.br.y - self.tl.y;
    }
    pub fn top(&self) -> (&Point, &Point) {
        return (&self.tl, &self.tr);
    }
    pub fn right(&self) -> (&Point, &Point) {
        return (&self.tr, &self.br);
    }
    pub fn bottom(&self) -> (&Point, &Point) {
        return (&self.br, &self.bl);
    }
    pub fn left(&self) -> (&Point, &Point) {
        return (&self.bl, &self.tl);
    }
}

impl Claim {
    pub fn new(from: &str) -> Claim {
        let (id, x, y, w, h) = scan_fmt!(from, "#{} @ {},{}: {}x{}", u32, i64, i64, i64, i64);
        let tl = Point {
            x: x.unwrap() + 1,
            y: y.unwrap() + 1,
        };
        let br = Point {
            x: x.unwrap() + w.unwrap() + 1,
            y: y.unwrap() + h.unwrap() + 1,
        };
        let r = Rectangle {
            tl: tl,
            tr: Point { x: tl.x, y: br.y },
            br: br,
            bl: Point { x: br.x, y: tl.y },
        };
        if r.width() != w.unwrap() {
            panic!("Failed to compute width {} {}", r.width(), w.unwrap());
        }
        if r.height() != h.unwrap() {
            panic!("Failed to compute heigth {} {}", r.height(), h.unwrap());
        }
        return Claim {
            id: id.unwrap(),
            rectangle: r,
        };
    }
    pub fn intersects(&self, other: &Claim) -> bool {
        return self.rectangle.intersects(&other.rectangle);
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        // Format is #{id} @ {x},{y}: {width}x{heigth}
        // where {x} and {y} denote the left and top MARGINS
        write!(
            f,
            "#{} @ {},{}: {}x{}",
            self.id,
            self.rectangle.tl.x - 1,
            self.rectangle.tl.y - 1,
            self.rectangle.width(),
            self.rectangle.height()
        )
        // write!(f, "#{} @ [{} {} {} {}]",
        //     self.id,
        //     self.rectangle.tl,
        //     self.rectangle.tr,
        //     self.rectangle.br,
        //     self.rectangle.bl)
    }
}

impl fmt::Display for Point {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{},{}", self.x, self.y)
    }
}

// To find orientation of ordered triplet (p, q, r).
// The function returns following values
// 0 --> p, q and r are colinear
// 1 --> Clockwise
// 2 --> Counterclockwise
fn orientation(p: &Point, q: &Point, r: &Point) -> i32 {
    let ord = (q.y - p.y) * (r.x - q.x) - (q.x - p.x) * (r.y - q.y);
    if ord > 0 {
        return 1; // The points are clockwise
    }
    if ord < 0 {
        return 2; // The points are anti-clockwise
    }
    return 0; // The points are on a line
}

// Does the line x intersect the line z?
fn lines_intersect(x: (&Point, &Point), z: (&Point, &Point)) -> bool {
    let (p1, q1) = x;
    let (p2, q2) = z;
    let o1 = orientation(p1, q1, p2);
    let o2 = orientation(p1, q1, q2);
    let o3 = orientation(p2, q2, p1);
    let o4 = orientation(p2, q2, q1);

    // General case
    if o1 != o2 && o3 != o4 {
        return true;
    }

    // Special Cases
    // p1, q1 and p2 are colinear and p2 lies on segment p1q1
    if o1 == 0 && on_segment(p1, p2, q1) {
        return true;
    }

    // p1, q1 and q2 are colinear and q2 lies on segment p1q1
    if o2 == 0 && on_segment(p1, q2, q1) {
        return true;
    }

    // p2, q2 and p1 are colinear and p1 lies on segment p2q2
    if o3 == 0 && on_segment(p2, p1, q2) {
        return true;
    }

    // p2, q2 and q1 are colinear and q1 lies on segment p2q2
    if o4 == 0 && on_segment(p2, q1, q2) {
        return true;
    }

    return false; // Doesn't fall in any of the above cases
}

fn on_segment(p: &Point, q: &Point, r: &Point) -> bool {
    if q.x <= std::cmp::max(p.x, r.x)
        && q.x >= std::cmp::min(p.x, r.x)
        && q.y <= std::cmp::max(p.y, r.y)
        && q.y >= std::cmp::min(p.y, r.y)
    {
        return true;
    }

    return false;
}

pub fn iterate_squares(claim: Claim) -> LinkedList<String> {
    let mut list: LinkedList<String> = LinkedList::new();
    let x_range = 0..claim.rectangle.width();
    for over_x in x_range {
        let mut y_range = 0..claim.rectangle.height();
        for over_y in y_range {
            let mut r: String = (over_x + claim.rectangle.tl.x).to_string();
            r.push_str("x");
            r.push_str(&(over_y + claim.rectangle.tl.y).to_string());
            list.push_back(r);
        }
    }
    return list;
}
