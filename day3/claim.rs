#[macro_use] extern crate scan_fmt;

use std::string::String;
use std::collections::LinkedList;
use std::cmp;
use std::fmt;

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Rectangle {
    x: u32,
    y: u32,
    width: u32,
    height: u32
}

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Claim {
    pub id: u32,
    rectangle: Rectangle,
}

impl Claim {
    pub fn new(from: &str) -> Claim {
        let (id,x,y,w,h) = scan_fmt!(
            from,
            "#{} @ {},{}: {}x{}",
            u32, u32, u32, u32, u32
        );

        return Claim{
            id: id.unwrap(),
            rectangle: Rectangle {
                x: x.unwrap(),
                y: y.unwrap(),
                width: w.unwrap(),
                height: h.unwrap()
            },
        }
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "#{} @ {},{}: {}x{}", self.id, self.rectangle.x, self.rectangle.y, self.rectangle.width, self.rectangle.height)
    }
}
// impl fmt::Display for HashSet<Claim> {
//     fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
//         for claim in self {
//             claim.fmt(f);
//         }
//     }
// }

pub fn parse_claim(claim_str: String) -> Claim {
    return Claim::new(&claim_str);
}

pub fn intersects(a: &Claim, b: &Claim) -> bool {
    return intersects_rectangle(&a.rectangle, &b.rectangle);
}

pub fn intersects_rectangle(a: &Rectangle, b: &Rectangle) -> bool {
    let max_x = cmp::max(a.x, b.x);
    let min_x = cmp::min(a.x+a.width, b.x+b.width);
    let max_y = cmp::max(a.y, b.y);
    let min_y = cmp::min(a.y+a.height, b.y+b.height);

    if max_x < min_x && max_y < min_y {
        return true;
    }
    return false;
}

pub fn iterate_squares(claim: Claim) -> LinkedList<String> {
    let mut list: LinkedList<String> = LinkedList::new();
    let x_range = 0..claim.rectangle.width;
    for over_x in x_range {
        let mut y_range = 0..claim.rectangle.height;
        for over_y in y_range {
            let mut r: String = (over_x + claim.rectangle.x).to_string();
            r.push_str("x");
            r.push_str(&(over_y + claim.rectangle.y).to_string());
            list.push_back(r);
        }
    }
    return list;
}
