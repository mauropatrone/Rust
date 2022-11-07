//#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn area(&self) -> u32 {
        self.width * self.height
    }
    
    fn can_hold(&self,r: &Rectangle) -> bool {
        self.width > r.width && self.height > r.height
    }
    
    fn square(size: u32) -> Rectangle {
        Rectangle {width: size, height: size}
    }
}

fn main() {
    let rect1 = Rectangle {width: 30, height: 50};
    let rect2 = Rectangle {width: 10, height: 40};
    let rect3 = Rectangle {width: 60, height: 45};
    let sq = Rectangle::square(10);
    
    //println!("The area is {}",area(&rect));
    println!("The area is {}",rect1.area());
    println!("Can rect1 hold rect2? {}",rect1.can_hold(&rect2));
    println!("Can rect1 hold rect3? {}",rect1.can_hold(&rect3));
    println!("The area of sq is {}",sq.area());
}

/*
fn area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
*/
