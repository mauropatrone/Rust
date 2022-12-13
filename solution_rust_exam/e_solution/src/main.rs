//! This portion will test your familiarity with some of Rust's common traits
//! and your ability to implement those traits in interesting ways
//! You need to remove all of the `todo!()`s. Most of them will need to be replaced
//! by some code, but some may be able to simply be deleted.

// NOTE: You will need to `use` something from the standard library to implement `Ord` and
// `PartialOrd` here.

use std::cmp::Ordering;

/// A record of an employee at a particular company
#[derive(Debug)]
pub struct Employee {
	/// The name the person likes to be called. Doesn't have to be their "passport name"
	pub name: String,
	/// Amount of experience (in months) the person has working at this company
	pub experience: u32,
	/// Hourly wage paid to this employee
	pub wage: u32,
	/// Unique identifier for this employee
	pub uid: u32,
}

// We want to consider two employee instances equal iff they have the same `uid`.

impl PartialEq for Employee {
    fn eq(&self, other: &Self) -> bool {
        self.uid == other.uid
    }
}

impl Eq for Employee {
}

// We want to sort employees. First and foremost, employees are equal if they have the same
// `uid`, as explained above. For employees who are not equal, we sort by the value they
// bring to the company. Value is defined as the quotient of the experience they've acquired
// at the company divided by their wage. Use integer division for the purpose of this calculation.

impl PartialOrd for Employee {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Employee {
    fn cmp(&self, other: &Self) -> Ordering {
        (self.experience/self.wage).cmp(&(other.experience/other.wage))
    }
}

// We want to parse employee information from a string data
// The string data should be comma separated. Here are a few examples:
//
// * "Billy, 4, 5, 345" - Billy has been working here for 4 months and earns 5 token per hour. She
//   is employee #345
// * "Jose, 12, 6, 1" - Jose has been working here for 1 year (12 months) and earns 6
// tokens per hour. He is employee #1
//
// Any strings that have the wrong number of commas of numbers too big for `u32` may return `None`

impl TryFrom<String> for Employee  {
    type Error = ();

    fn try_from(input: String) -> Result<Self, Self::Error> {
        if input.is_empty() {return Err(())};

        let v: Vec<&str> = input.split(", ").collect();
        if v.len() != 4 {return Err(())};

        let employee = Self {
            name: String::from(v[0]),
            experience: v[1].parse().unwrap(),
            wage: v[2].parse().unwrap(),
            uid: v[3].parse().unwrap(),
        };
        Ok(employee)
    }
}

// We also want to convert employees back into strings in the same format

impl From<Employee> for String {
    fn from(e: Employee) -> String {
        let mut ret = e.name.to_string();
        ret.push_str(", ");
        ret.push_str(&(e.experience.to_string())[..]);
        ret.push_str(", ");
        ret.push_str(&(e.wage.to_string())[..]);
        ret.push_str(", ");
        ret.push_str(&(e.uid.to_string())[..]);
        ret   
    }
}

fn main() {
    let s = String::from("Billy, 4, 5, 345");
    let a = String::from("Billy, 4, 5, 345");

    let mut e1 = Employee::try_from(s);
    e1.name = String::from("Mauro");
    e1.experience = 2;
    e1.wage = 1;
    e1.uid = 11;

    let mut e2 = Employee::try_from(a);
    e2.name = String::from("Jaz");
    e2.experience = 8;
    e2.wage = 5;
    e2.uid = 12;

    if e1 == e2 {
        println!("Equals!");
    } else if e1 > e2 {
        println!("e1 > e2");
    } else {
        println!("e1 < e2");
    }

    //let s = String::from("Billy, 4, 5, 345");
    
    match Employee::try_from(s) {
        Ok(x) => println!("{:?}",x),
        Err(_) => println!("Error Employee::try_from()"),
    }

    let g: String = e2.into();

    println!("{}",g);
    println!("{:?}",g);
    


}
