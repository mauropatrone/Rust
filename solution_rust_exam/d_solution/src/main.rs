//! Complete the following functions using the pattern matching syntax. That includes the `match`
//! statement of the `matches!()` marco, if you feel like have a 1-liner.
//!
//! You can try and write them imperatively at first as well, but at the end of the day, we highly
//! prefer you to write them using the `match` or `matches!`.

/// Returns true if the last two strings in the vector start with `PBA`.
pub fn match_1(input: Vec<String>) -> bool {
    let size = input.len();
    if size < 2 {
        false
    } else {
	    matches!(&input[size-2][0..3], "PBA") && matches!(&input[size-1][0..3], "PBA")
    }
}

/// Returns true if the first and last string in the vector start with `PBA`.
pub fn match_2(input: Vec<String>) -> bool {
    if input.len() < 2 {
        return false
    }
    let first = match input.first() {
        Some(x) => &x[0..3],
        None => return false,
    };
    let last = match input.last() {
        Some(x) => &x[0..3],
        None => return false,
    };
	matches!(first, "PBA") && matches!(last, "PBA")
}

/// Returns true if the first item in `input` is true.
pub fn match_3(input: (bool, bool, bool)) -> bool {
	match input.0 {
        true => true,
        _ => false,
    }
}

/// Returns true if the input is `Ok(x)` of some even `x`.
pub fn match_4(input: Result<u32, &'static str>) -> bool {
	match input {
        Ok(x) => {
            if x % 2 == 0 {
                true
            } else {
                false
            }
        },
        Err(_) => false,
    }
}


fn main() {
    let mut vs: Vec<String> = Vec::new();
    vs.push(String::from("PBA Roberto carlos"));
    vs.push(String::from("PBA LTA"));
    vs.push(String::from("PABPBA"));
    vs.push(String::from("PBARoberto carlos"));
    vs.push(String::from("PBAires"));
    let mut v: Vec<String> = Vec::new();
    v.push(String::from("PBA LTA"));

    //println!("{:?}",vs);
    if match_1(vs) {
        println!("match_1 is true");
    } else {
        println!("match_1 is false");
    }
    if match_2(v) {
        println!("match_2 is true");
    } else {
        println!("match_2 is false");
    }
    if match_3((false,true,true)) {
        println!("match_3 is true");
    } else {
        println!("match_3 is false");
    }

    if match_4(Err("Some error msg")) {
        println!("match_4 is even");
    } else {
        println!("match_4 is odd");
    }
    //println!("Hello, world!");
}
