fn main() {
    let board = [
        "   ",
        "1*1",
        "111"
    ];
    let a = '*';
    let n = "   ".chars().count();
    let mut c = "*1".chars();
    println!("{:?} - {}",c,n);
    if Some(a) == c.next() {
        println!("ast!")
    }
    let c = "1 *";
    //add2(&mut c.chars().next());
    println!("{}",&c[0..1]);
}

fn add(index: usize, c: &mut String) {
    match c.chars().nth(index).unwrap() {
        ' ' => {
            c.remove(index);
            c.insert(index,'1');
        },
        '1' => {
            c.remove(index);
            c.insert(index,'2');
        },
        '2' => {
            c.remove(index);
            c.insert(index,'3');
        },
        '3' => {
            c.remove(index);
            c.insert(index,'4');
        },
        '4' => {
            c.remove(index);
            c.insert(index,'5');
        },
        '5' => {
            c.remove(index);
            c.insert(index,'6');
        },
        '6' => {
            c.remove(index);
            c.insert(index,'7');
        },
        '7' => {
            c.remove(index);
            c.insert(index,'8');
        },
        _ => (),
    }
}

#[test]
fn test_add() {
    let c = "1 *";
    let mut s = String::from(c);
    add(0,&mut s);
    assert_eq!(s,"2 *");
    add(1,&mut s);
    assert_eq!(s,"21*");
    add(2,&mut s);
    assert_eq!(s,"21*");
}