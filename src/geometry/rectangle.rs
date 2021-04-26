use macroquad::shapes::draw_rectangle;
use macroquad::shapes::draw_rectangle_lines;
use macroquad::color::Color;
pub struct Rectangle {
    pub x:f32,
    pub y:f32,
    pub w:f32,
    pub h:f32,
}
impl Rectangle {
    pub fn new(x:f32,y:f32,w:f32,h:f32) -> Rectangle {
        Rectangle {
            x,y,w,h
        }
    }
    pub fn new_centered(x:f32,y:f32,w:f32,h:f32) -> Rectangle {
        Rectangle {
            x:x-w/2.0,
            y:y-h/2.0,
            w,h
        }
    }
    pub fn draw(&self, x:f32, y:f32, color:Color) {
        draw_rectangle(self.x + x, self.y + y, self.w, self.h, color);
    }
    pub fn draw_outline(&self, x:f32, y:f32, thickness:f32, color:Color) {
        draw_rectangle_lines(self.x+x,self.y+y,self.w,self.h,thickness,color);
    }
    pub fn check_collision(&self, other: &Rectangle) -> bool {
        !(self.x > other.x + other.w || other.x > self.x + self.w || self.y > other.y + other.h || other.y > self.y + self.h)
    }
    pub fn collides_pt(&self, x: f32, y:f32) -> bool {
        x >= self.x && y >= self.y && x <= self.x + self.w && y <= self.y + self.h
    }
}
pub struct ColoredRectangle {
    pub x:f32,
    pub y:f32,
    pub w:f32,
    pub h:f32,
    pub c:Color,
}
impl ColoredRectangle {
    pub fn new(x:f32,y:f32,w:f32,h:f32,c:Color) -> ColoredRectangle {
        ColoredRectangle {
            x,y,w,h,c
        }
    }
    pub fn draw(&self, x:f32, y:f32) {
        draw_rectangle(self.x + x, self.y + y, self.w, self.h, self.c);
    }
    pub fn check_collision(&self, other: &Rectangle) -> bool {
        !(self.x > other.x + other.w || other.x > self.x + self.w || self.y > other.y + other.h || other.y > self.y + self.h)
    }
}