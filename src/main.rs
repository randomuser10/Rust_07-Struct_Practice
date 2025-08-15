//adding Debug trait to display the struct
#[derive(Debug)]
// struct for rectangle 
    struct Rectangle {
        width: u32,
        length: u32,
    }

fn main() {
    //creating instance of the rectangle
    let rect1 = Rectangle{
        width: 32,
        length: 40,
    };
// calculating the area of the rectangle based on the width and length passed from the 
// struct
    let the_area = _area(&rect1);    
    println!("The area of the rectangle is {}", the_area);
    println!("Rect1 is {:#?}", rect1);
}
// function to calculate the rect and return the area 
fn _area(rectangle: &Rectangle) -> u32 {
    rectangle.width * rectangle.length
}
