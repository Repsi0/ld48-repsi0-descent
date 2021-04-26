use macroquad::color::WHITE;

use crate::geometry::rectangle::Rectangle;
use crate::geometry::sprite::Sprite;

use crate::HEIGHT;
use crate::WIDTH;
use crate::LOGO_OFFSET_Y;
use crate::MENU_QUIT_OFFSET_Y;

pub struct Menu {
    logo_img: Sprite,
    play: Rectangle,
    play_img: Sprite,
    quit: Rectangle,
    quit_img: Sprite,
}
impl Menu {
    pub fn new() -> Menu {
        let w = WIDTH/2.0;
        let h = HEIGHT / 2.0;

        Menu {
            logo_img: Sprite::new_centered(w, h + LOGO_OFFSET_Y, "res/logo.png"),
            play: Rectangle::new_centered(w, h, 188.0, 56.0),
            play_img: Sprite::new_centered(w, h, "res/play.png"),
            quit: Rectangle::new_centered(w, h + MENU_QUIT_OFFSET_Y, 188.0, 56.0),
            quit_img: Sprite::new_centered(w, h + MENU_QUIT_OFFSET_Y, "res/quit.png"),
        }
    }
    pub fn draw(&mut self, x:f32, y:f32) {
        self.logo_img.draw(x,y,WHITE);
        self.play.draw_outline(x, y, 2.0, WHITE);
        self.play_img.draw(x,y,WHITE);
        self.quit.draw_outline(x, y, 2.0, WHITE);
        self.quit_img.draw(x,y,WHITE);
    }

    pub fn click(&mut self, x: f32, y: f32) -> (bool, bool) {
        if self.play.collides_pt(x, y) {
            return (true, false);
        } else if self.quit.collides_pt(x, y) {
            return (false, true);
        }
        return (false, false);
    }
}