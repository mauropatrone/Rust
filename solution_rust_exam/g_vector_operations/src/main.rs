//! In the portion of the test, you will write a few functions that operate on Vectors.
//! The algorithms include sorts, searches, and reversals.
//! In addition to understanding the algorithm, this will test your understanding of Rust's
//! ownership model.

/// This is an in-place sort, so it moves data around in the original slice.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Bubble_sort)
/// But you may not look up implementations of it in Rust.
pub fn bubble_sort(items: &mut [u32]) {
    if items.len() == 0 {
        return;
    }
    
    let mut index = 0;
    let mut index_bound = items.len() - 1;
    let mut swapped = false;

    'sortloop: loop {
        if items[index] > items[index+1] {
            items.swap(index,index+1);
            swapped = true;
        }
        index += 1;

        if index > index_bound {
            break 'sortloop;
        } else if index == index_bound {
            if swapped {
                index = 0;
                index_bound -= 1; 
                swapped = false;
            } else {
                break 'sortloop;
            }
        }
    }
}

/// This is a recursive sort, so you must use recursion.
/// This is NOT an in-place sort, so it will return a copy of the data in a new Vec.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Merge_sort)
/// But you may not look up implementations of it in Rust.
pub fn merge_sort(items: &[u32]) -> Vec<u32> {
    if items.len() <= 1 {
        return vec![items[0]];
    }

    let (left, right) = items.split_at(items.len()/2);

    merge(merge_sort(&left),merge_sort(&right))
}

pub fn merge(l: Vec<u32>, r: Vec<u32>) -> Vec<u32>{
    let mut left = l;
    let mut right = r;
    let mut v: Vec<u32> =  Vec::new();
    while !left.is_empty() && !right.is_empty() {
        if left[0] <= right[0] {
            v.push(left.remove(0))
        } else {
            v.push(right.remove(0))
        }
    }
    while !left.is_empty() {
        v.push(left.remove(0))
    }
    while !right.is_empty() {
        v.push(right.remove(0))
    }
    v
}

/// Reverse a slice in-place.
/// This is what the built-in `reverse` method does. You may NOT use that method here
/// https://doc.rust-lang.org/std/primitive.slice.html#method.reverse
pub fn reverse_in_place(items: &mut [u32]) {
    if items.len() == 0 {
        return;
    }
    
    let mut first = 0;
    let mut last = items.len() - 1;
    let mut temp: u32;

    while first < last {
        temp = items[first];
        items[first] = items[last];
        items[last] = temp;
        first += 1;
        last -= 1;
    }
}

/// Create and return a Vec containing the same data as the parameter slice in reverse order.
pub fn reverse_copy(items: &[u32]) -> Vec<u32> {
	let mut v: Vec<u32> = Vec::new();
    items.iter()
         .rev()
         .for_each(|x| v.push(*x));
    v
}

/// Create and return a Vec containing the same data as the parameter slice in reverse order.
pub fn reverse_copy_2(items: &[u32]) -> Vec<u32> {
	let mut v: Vec<u32> = Vec::new();
    for x in items {
        v.insert(0,*x);
    }
    v
}

/// Search a slice for a particular item. Return the index of the first occurrence of that item.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Linear_search)
/// But you may not look up implementations of it in Rust.
/// This is what the built-in `contains` method does. You may NOT use that method here.
/// https://doc.rust-lang.org/std/primitive.slice.html#method.contains
pub fn linear_search(items: &[u32], target: &u32) -> Option<usize> {
	for (i,x) in items.iter().enumerate() {
        if x == target {
            return Some(i);
        }
    }
    None
}

/// Search a slice for a particular item. Return the index of any occurrence of that item.
/// You may look up how the algorithm works (eg here https://en.wikipedia.org/wiki/Binary_search)
/// You may (and must) assume that the data is sorted.
/// But you may not look up implementations of it in Rust.
/// This is what the built-in `binary_search` method does. You may NOT use that method here.
/// https://doc.rust-lang.org/std/primitive.slice.html#method.binary_search
pub fn binary_search(items: &[u32], target: &u32) -> Option<usize> {
	if items.len() == 0 {
        return None;
    }
    
    let mut first: usize = 0;
    let mut last: usize = items.len() - 1;
    let mut index: usize;

    while first<=last {
        index = (first+last)/2;
        if items[index] > *target {
            last = index - 1;
        } else if items[index] < *target {
            first = index + 1;
        } else {
            return Some(index);
        }
    }
    None
}

fn main() {
    let mut d: [u32;4] = [8,4,6,2];
    let mut s: [u32;10] = [0,1,1,1,2,2,3,5,7,9];
    let m: [u32;8] = [2,8,1,7,8,4,6,2];
    //let mut v = s;
    //let rv = &mut v;

    bubble_sort(&mut d);
    println!("bubble_sort: {:?}",d);
    println!("merge_sort: {:?}",merge_sort(&m));
    reverse_in_place(&mut s);
    println!("reverse_in_place: {:?}",s);
    println!("reverse_copy_2: {:?}",reverse_copy_2(&s));
    println!("linear_search: {:?}",linear_search(&s,&s[3]));
    println!("binary_search: {:?}",binary_search(&s,&s[3]));
}
