use macroquad::color::*;

use crate::WIDTH;
use crate::geometry::rectangle::Rectangle;

use crate::CYAN;

pub struct Layer {
    pub body1: Rectangle,
    pub body2: Rectangle,
    pub value: u64,
    layer: u64,
    color: Color,
    pub mark: bool,
}
impl Layer {
    pub fn new(space:Rectangle,value:u64,layer:u64,hw:f32) -> Layer {
        Layer {
            value,layer,
            body1: Rectangle::new(hw,space.y,space.x-hw,space.h),
            body2: Rectangle::new(space.x+space.w,space.y,WIDTH - (space.x - hw + space.w),space.h),
            color: match value {
                1 => DARKGRAY,
                5 => LIGHTGRAY,
                10 => WHITE,
                50 => RED,
                100 => ORANGE,
                2500 => YELLOW,
                10000 => GREEN,
                250000 => CYAN,
                1000000 => BLUE,
                25000000 => VIOLET,
                _                 => MAGENTA,
            },
            mark: false,
        }
    }
    pub fn draw(&self, x:f32, y:f32) {
        self.body1.draw(x, y, self.color);
        self.body2.draw(x, y, self.color);
    }
    pub fn change(&mut self, space: Rectangle, value: u64, layer: u64, hw: f32) { //hw = hud width
        self.value = value;
        self.layer = layer;
        self.body1 = Rectangle::new(hw,space.y,space.x-hw,space.h);
        self.body2 = Rectangle::new(space.x+space.w,space.y,WIDTH - (space.x - hw + space.w),space.h);
        self.color = match value {
            1 => DARKGRAY,
            5 => LIGHTGRAY,
            10 => WHITE,
            50 => RED,
            100 => ORANGE,
            2500 => YELLOW,
            10000 => GREEN,
            250000 => CYAN,
            1000000 => BLUE,
            25000000 => VIOLET,
            _                 => MAGENTA,
        };
        self.mark = false;
    }
}