#[derive(Clone, Copy)]
pub enum Distance {
	Meters(f32),
	Feet(f32),
	Miles(f32),
}

const METERS_PER_MILE: f32 = 1609.344;
const FEET_PER_METER: f32 = 3.281;

impl Distance {
	/// Convert the given distance to meters
	pub fn to_meters(&self) -> Self {
		match self {
            Distance::Feet(x) => Distance::Meters((*x)/FEET_PER_METER),
            Distance::Miles(x) => Distance::Meters((*x)*METERS_PER_MILE),
            _ => *self,
        }
	}

	/// Convert the given distance to feet
	pub fn to_feet(&self) -> Self {
		match self {
            Distance::Meters(x) => Distance::Feet((*x)*FEET_PER_METER),
            Distance::Miles(x) => Distance::Feet((*x)*METERS_PER_MILE*FEET_PER_METER),
            _ => *self,
        }
	}

	/// Convert the given distance to miles
	pub fn to_miles(&self) -> Self {
		match self {
            Distance::Meters(x) => Distance::Miles((*x)/METERS_PER_MILE),
            Distance::Feet(x) => Distance::Miles((*x)/FEET_PER_METER/METERS_PER_MILE),
            _ => *self,
        }
	}

    pub fn get(self) -> f32 {
        match self {
            Distance::Meters(x) => x,
            Distance::Feet(x) => x,
            Distance::Miles(x) => x,
        }
    }
}


fn main() {
    let a = Distance::Meters(1.0);
    let b = Distance::Feet(1.0);
    let c = Distance::Miles(1.0);

    println!("a to m: {:?}",a.to_meters().get());
    println!("a to ft: {:?}",a.to_feet().get());
    println!("a to ml: {:?}",a.to_miles().get());

    println!("b to m: {:?}",b.to_meters().get());
    println!("b to ft: {:?}",b.to_feet().get());
    println!("b to ml: {:?}",b.to_miles().get());

    println!("c to m: {:?}",c.to_meters().get());
    println!("c to ft: {:?}",c.to_feet().get());
    println!("c to ml: {:?}",c.to_miles().get());
}