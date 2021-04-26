use macroquad::color::*;

use crate::HUD_WIDTH_INITIAL;
use crate::GAME_WIDTH_INITIAL;
use crate::geometry::rectangle::ColoredRectangle;
use crate::geometry::rectangle::Rectangle;

use crate::CYAN;

pub struct FinalLayer {
    pub body: Vec<ColoredRectangle>,
    pub draw: bool,
}
impl FinalLayer {
    pub fn new() -> FinalLayer {
        let mut body: Vec<ColoredRectangle> = Vec::new();
        let colors: [Color;7] = [RED,ORANGE,YELLOW,GREEN,CYAN,BLUE,VIOLET];
        for i in 0 .. 7 {
            body.push(ColoredRectangle::new(HUD_WIDTH_INITIAL + 20.0 + i as f32 * GAME_WIDTH_INITIAL / 7.0, 10000.0*60.0, (GAME_WIDTH_INITIAL - 20.0) / 7.0, 20.0, colors[i]));
        }
        FinalLayer {
            body,
            draw:true,
        }
    }
    pub fn draw(&self, x:f32, y:f32) {
        if self.draw {
            for cr in self.body.iter() {
                cr.draw(x,y);
            }
        }
    }
    pub fn check_collision(&self, rect: &Rectangle) -> bool {
        for r in self.body.iter() {
            if r.check_collision(rect) {
                return true;
            }
        }
        return false;
    }
}