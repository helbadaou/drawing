use rand::Rng;
use raster::{Color, Image};

pub trait Displayable {
   fn display(&mut self, x: i32, y: i32, color: Color);
}
pub trait Drawable {
    fn draw(&mut self,image : &mut Image);
    fn color() ->Color;
    
}
pub struct Point (i32, i32);
impl Point {
   pub fn new(x: i32, y:i32) -> Self{
     Point(x, y)   
    }
    pub fn random(x: i32, y:i32) -> Self{
        let mut rng = rand::thread_rng();
        Self::new(rng.gen_range(0..x), rng.gen_range(0..y))
    }
}

impl Drawable for Point  {
     fn draw(&mut self,image : &mut Image){
        image.display(self.0, self.1, Self::color());
     }
    fn color()->Color{
        let mut rng = rand::thread_rng();
        Color::rgba(rng.gen_range(0..255), rng.gen_range(0..255), rng.gen_range(0..255), 255)
    }
}