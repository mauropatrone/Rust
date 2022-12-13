//! This portion of the exam tests your abilities to work with iterators and their functional-style
//! methods.
//!
//! Throughout this portion of the test, you may refer to https://doc.rust-lang.org/std/iter/trait.Iterator.html
//! and other docs about iterators. You may NOT look up specific implementations for these problems
//! in Rust or any other Functional language.
//!
//! If you find that you simply cannot write these methods in the functional style using iterator
//! methods, writing them in the imperative style with loops will still earn partial credit.

/// This function takes an iterator of u32 values, squares each value, and returns the sum of the
/// squares. You may assume that no individual square, nor the entire sum, overflows the u32 type.
pub fn sum_of_squares(vals: impl Iterator<Item = u32>) -> u32 {
    vals.fold(0, |acc, x| acc + x*x)
}

/// This function takes an iterator of i32 values, calculates the absolute value of each, and throws
/// away any values that are greater than 100. The remaining positive values are returned as an
/// iterator of u32s.
pub fn bounded_absolute_values(vals: impl Iterator<Item = i32>) -> impl Iterator<Item = u32> {
    let mut v: Vec<u32> = Vec::new();
    vals.filter(|x| (*x).abs()<=100)
        .for_each(|x| v.push(x.abs() as u32));
    v.into_iter()
}

// We allow `unused_mut` only so that there is no build warning on the starter code.
// You should remove this line once you have completed the following function
//#[allow(unused_mut)]
/// This function takes an iterator of u32 values. The first value in the iterator, call it n, is
/// special: it represents the maximum count of the resultant iterator. Once n is known, create an
/// iterator that yields the first n even values from the remainder of the input iterator.
///
/// If the input iterator is empty, return None
/// If there are fewer than n even values left in the input, return as many as possible
pub fn first_n_even(mut vals: impl Iterator<Item = u32>) -> Option<impl Iterator<Item = u32>> {
    let mut v: Vec<u32> = Vec::new();

    match vals.nth(0){
        None => return None,
        Some(x) => {
            let n = x;
            vals.filter(|x| *x % 2 == 0)
                .take(n as usize)
                .for_each(|x| v.push(x));	
            Some(v.into_iter())
        },
    }
}

/// Return an "infinite" iterator that yields the squares of the whole numbers.
/// For example, the first few values should be 0, 1, 4, 9, 16, 25, ...
///
/// The iterator should be bounded only by the u32 type, not by your code
pub fn square_whole_numbers() -> impl Iterator<Item = u32> {
    let mut v: Vec<u32> = Vec::new();
    (0..u32::MAX).try_for_each(|x: u32| {
        match x.checked_pow(2) {
            Some(pow) => Some(v.push(pow)),
            None => return None,
        }
    });
    v.into_iter()
}

pub fn square_whole_numbers_u8() -> impl Iterator<Item = u8> {
    let mut v: Vec<u8> = Vec::new();
    (0..u8::MAX).try_for_each(|x: u8| {
        match x.checked_pow(2) {
            Some(pow) => Some(v.push(pow)),
            None => return None,
        }
    });
    v.into_iter()
}

/// An iterator that generates the Fibonacci sequence.
#[derive(Default)]
pub struct Fibonacci {
	/// The most recent value this iterator has yielded
	prev: Option<u32>,
	/// The second most recent value that this iterator has yielded
	prev_prev: Option<u32>,
}

impl Iterator for Fibonacci {
	type Item = u32;

	fn next(&mut self) -> Option<u32> {
        let temp: u32;
        match self.prev {
            None => {self.prev = Some(0); temp = 0},
            Some(0) => temp = 1,
            Some(p) => match self.prev_prev {
                Some(pp) => temp = p + pp,
                None => return None,
            },
        };
        self.prev_prev = self.prev;
        self.prev = Some(temp);
        self.prev
	}
}

fn main() {
    let v: Vec<u32> = vec![0,0,2,0];
    let w: Vec<i32> = vec![-120,-40,125,-210,10,200,0];
    let q: Vec<u32> = vec![8,1,2,3,4,5,6];
    //let q2: Vec<u32> = Vec::new();

    println!("sum_of_squares: {}",sum_of_squares(v.into_iter()));
    println!("---------");
    bounded_absolute_values(w.into_iter()).for_each(|x| println!("{}",x));
    println!("---------");
    match first_n_even(q.into_iter()) {
        Some(i) => i.for_each(|x| println!("{}",x)),
        None => println!("None"),
    }
    println!("---------");
    if let Some(x) = square_whole_numbers().last(){
        println!("{}",x);
    }

    println!("----Fibonacci-----");
    let mut fibo = Fibonacci{
        prev: None,
        prev_prev: None,
    };

    println!("p: {:?}, pp: {:?}", fibo.prev, fibo.prev_prev);
    for _ in 0..9 {
        println!("next: {:?}, p: {:?}, pp: {:?}",fibo.next(), fibo.prev, fibo.prev_prev);
    }
}