pub fn reverse(input: &str) -> String {
    //let mut f: Vec<_> = input.split("").collect();
    let mut f = String::from(input);
    let mut r: String = String::from("");
    while f.len() > 0 {
        r.push(f.pop().unwrap());
    }
    r
}

#[test]
fn test_reverse_string() {
    let mut s = String::from("hello world");
    let mut r: String = String::from("");

    while s.len() > 0 {
        r.push(s.pop().unwrap());
    }
    assert_eq!(r,String::from("dlrow olleh"))
}

#[test]
fn test_split() {
    let s = "rust";
    let f: Vec<_> = s.split("").collect();
    assert_eq!(f, &["", "r", "u", "s", "t", ""]);
}

#[test]
fn test_reverse() {
    let s = "rust";
    assert_eq!(reverse(s), String::from("tsur"));
}

fn main() {
    println!("Hello, world!");
}
