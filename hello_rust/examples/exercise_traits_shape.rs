

fn main(){
    create_shape();
}

trait Shape {
    fn area(& self)->f64;
}

struct Rectangle{
    length:f32,
    breadth:f32
}
impl Shape for Rectangle{
    fn area(&self)->f64{
        return (self.breadth * self.length) as f64 ;
    }
}

struct Circle{
    radius:f32,
}

impl Shape for Circle{
    fn area(&self)->f64{
        return ((3.142857 as f32) * self.radius * self.radius ) as f64;
    }
}

fn create_shape(){
    let length=12.2f32;
    let breadth=24.2f32;
    let radius=12.21f32;
    let rec=Rectangle{length,breadth};
    let circ=Circle{radius};

    println!("Area of the rectangle is {:.2}",rec.area());
    println!("Area of the Circle is {:.2}",circ.area());

}




