use macroquad::color::Color;
use crate::geometry::rectangle::Rectangle;
pub struct Bar {
    outline: Rectangle,
    parts: Vec<Rectangle>,
    pub numparts: u32,
    pub numenabled: u32,
}
impl Bar {
    pub fn new(x:f32, y:f32, w: f32, h: f32, intern_width: f32, numparts: u32) -> Bar {
        let mut parts: Vec<Rectangle> = Vec::new();
        for i in 0..numparts {
            parts.push(Rectangle::new(x + (((intern_width-numparts as f32)/numparts as f32) + 1.0) * i as f32 + 2.5, y + 2.0, (intern_width-numparts as f32) / numparts as f32, h - 4.0));
        }
        Bar {
            outline: Rectangle::new(x,y,w,h),
            parts: parts,
            numenabled: 0,
            numparts,
        }
    }
    pub fn draw(&self, x:f32, y:f32, color:Color) {
        self.outline.draw_outline(x,y,2.0,color);
        let mut i = 1;
        for part in self.parts.iter() {
            if self.numenabled >= i {
                part.draw(x,y,color);
                i += 1;
            } else {
                part.draw(x,y,Color::new(0.1,0.1,0.1,1.0));
            }
        }
    }
    pub fn buy(&mut self) -> bool {
        let ret = self.numparts > self.numenabled;
        if ret {
            self.numenabled+=1;
        }
        ret
    }
}