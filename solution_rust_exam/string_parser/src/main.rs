fn main() {
    let mut s = String::from("Billy, 5, 6,     132");

    let mut name = String::from("");
    let mut exp = String::from("");
    let mut wage = String::from("");
    let mut uid = String::from("");

    let mut v = Vec::new();
    
    'uidloop: loop {
        match s.pop() {
            Some(',') => break 'uidloop,
            Some(' ') => (),
            Some(x) => v.push(x),
            None => break 'uidloop,
        }
    }

    v.reverse();
    for x in &v{
        uid.push(*x);
    }
    v.clear();
    println!("{}",uid);

    'wageloop: loop {
        match s.pop() {
            Some(',') => break 'wageloop,
            Some(' ') => (),
            Some(x) => v.push(x),
            None => break 'wageloop,
        }
    }

    v.reverse();
    for x in &v{
        wage.push(*x);
    }
    v.clear();
    println!("{}",wage);

    'exploop: loop {
        match s.pop() {
            Some(',') => break 'exploop,
            Some(' ') => (),
            Some(x) => v.push(x),
            None => break 'exploop,
        }
    }

    v.reverse();
    for x in &v{
        exp.push(*x);
    }
    v.clear();
    println!("{}",exp);

    'nameloop: loop {
        match s.pop() {
            Some(',') => break 'nameloop,
            Some(' ') => (),
            Some(x) => v.push(x),
            None => break 'nameloop,
        }
    }

    v.reverse();
    for x in v{
        name.push(x);
    }

    println!("{}",name);

    let mut num: u32 = 0; 
    match name.parse::<u32>() {
        Ok(x) => num = x,
        Err(_) => (),
    }

    println!("{}",num);


    match uid.parse::<u32>() {
        Ok(x) => num = x,
        Err(_) => (),
    }

    println!("{}",num);
}
