
trait Area {
    fn area(&self) -> f32;
}

#[derive(Debug)]
struct Rectangle {
    w: f32,
    h: f32,
}

impl Area for Rectangle {
    fn area(&self) -> f32 {
        self.w * self.h
    }
}

#[derive(Debug)]
struct Triangle{
    w: f32,
    h: f32,
}

impl Area for Triangle {
    fn area(&self) -> f32 {
        self.h * self.w*0.5
    }
}

#[derive(Debug)]
struct Circle{
    r: f32,
}

impl Area for Circle{
    fn area(&self) -> f32 {
        self.r * self.r
    }
}

fn static_dispatch<T>(t:&T)
    where T:Area,
    {
        //println!("{:?}",*t);
        println!("The shape area is {}",t.area());
    }

fn main(){
    let rectangle = Rectangle{ w: 10.0, h: 2.5 };
    static_dispatch(&rectangle);
    let circle = Circle{ r: 10.0};
    static_dispatch(&circle);
    let trianlge = Triangle{ w: 10.0, h: 4.0 };
    static_dispatch(&trianlge);

}
