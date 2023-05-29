//! Here we test your ability to write macros.
//!
//! Make sure these macros are importable from the root of your
//! crate, and usable in an external crate.

// A common Rust macro is `vec![...]` used for creating vectors of literal values. Without this
// macro, one would need to create an empty vector, and push each item to it individually.
//
// Your task here is to create an analogous macro for creating hashmaps pre-populated with literal
// values. The macro should be called like follows:
//
// let map1: HashMap<u32, u32> = map![1 =>2, 3 => 4, 5 => 6)];

#[macro_export]
macro_rules! map {
	($($x:expr=>$y:expr),*) => {
		{
            let mut temp_hash_map = HashMap::new();
            $(
                temp_hash_map.insert($x, $y);
            )*
            temp_hash_map
        }
	};
}

// OPTIONAL
// Next, write a macro that implements a `get` function on a type. See the test case below for the
// expected syntax.
//
// We want to same macro to be able to parse multiple items, and each item can be either `type Foo
// ..` or `const Foo ..`. The former should generate a `fn get()`, and the latter a `const fn
// get()`.
#[macro_export]
macro_rules! impl_get {
	($type:ty) => {
		fn get(x: $type) -> $type{
            x
        }
	};
    (mut $type:ty) => {
		fn get(x: mut $type) -> mut $type{
            x
        }
	};
}

macro_rules! myvec {
    ($($x:expr),*) => {
        {
            let mut temp_vec = Vec::new();
            $(
                temp_vec.push($x);
            )*
            temp_vec
        }
    }
}

use std::collections::HashMap;

impl_get!(u32);

fn main() {
    let v: Vec<u32> = myvec![0,1,2];
    println!("{:?}",v);
    let map1: HashMap<u32, u32> = map![1 =>2, 3 => 4, 5 => 6];
    println!("{:?}",map1);
    let mut x: u32 = 4;
    println!("{:?}",get(x));
}
