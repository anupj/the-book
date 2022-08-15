use std::ops::Add;

#[derive(Debug, PartialEq)]
struct Millimeters(u32);

#[derive(Debug, PartialEq)]
struct Meters(u32);

impl Add<Meters> for Millimeters {
    type Output = Millimeters;

    fn add(self, other: Meters) -> Millimeters {
        Millimeters(self.0 + (other.0 * 1000))
    }
}

fn main() {
    let r1_mm: Millimeters = Millimeters(100);
    let r2_m: Meters = Meters(20);

    assert_eq!(r1_mm + r2_m, Millimeters(20100));
}
