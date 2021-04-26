use macroquad::color::WHITE;

use crate::geometry::text::Text;
use crate::geometry::rectangle::Rectangle;

use crate::HEIGHT;
use crate::HUD_WIDTH_INITIAL;
use crate::DEPTH_X;
use crate::DEPTH_Y;
use crate::RECORD_X;
use crate::RECORD_Y;
use crate::MONEY_X;
use crate::MONEY_Y;
use crate::PROFIT_X;
use crate::PROFIT_Y;
use crate::TIMER_X;
use crate::TIMER_Y;
use crate::STATS_FONT_SIZE;

use crate::DEPTH_METER_X;
use crate::DEPTH_METER_Y_OFFSET;

pub enum Stat {
    Depth,
    Record,
    Money,
    Profit,
}
pub struct HUD {
    hud_sep_bar: Rectangle,
    depth_display: Text,
    record_display: Text,
    money_display: Text,
    profit_display: Text,
    timer_display: Text,
    depth_meter: Vec<Text>,
    depth_last: u64,
}
impl HUD {
    fn layer_text(layer: u64) -> String {
        let mut depthtxt = String::new();
        let unit = " m";
        depthtxt.push_str(&(layer+1).to_string());
        depthtxt.push_str(unit);
        return depthtxt;
    }
    pub fn new() -> HUD {
        let mut depth_meter = Vec::new();
        for i in 0..25 {
            depth_meter.push(Text::new(DEPTH_METER_X, DEPTH_METER_Y_OFFSET + 60.0 * (i+1) as f32, HUD::layer_text(i)));
        }
        HUD {
            hud_sep_bar: Rectangle::new(HUD_WIDTH_INITIAL, 0.0, 20.0, HEIGHT),
            depth_display: Text::new(DEPTH_X,DEPTH_Y,String::from("Depth: 0m")),
            record_display: Text::new(RECORD_X,RECORD_Y,String::from("(Record: 0m)")),
            money_display: Text::new(MONEY_X,MONEY_Y,String::from("  $0")),
            profit_display: Text::new(PROFIT_X,PROFIT_Y,String::from("+ $0")),
            timer_display: Text::new(TIMER_X,TIMER_Y,String::from("00:00:00")),
            depth_meter,
            depth_last: 25,
        }
    }
    pub fn reset(&mut self) {
        self.depth_meter = Vec::new();
        self.depth_last = 0;
        for i in 0..25 {
            self.depth_meter.push(Text::new(DEPTH_METER_X, DEPTH_METER_Y_OFFSET + 60.0 * (i+1) as f32, HUD::layer_text(i)));
        }
    }
    pub fn draw(&mut self, x:f32, y:f32, py:f32, bx:f32, draw_depth_meter:bool, draw_stats:bool, draw_timer:bool) {
        self.hud_sep_bar.draw(bx,y,WHITE);
        if draw_stats {
            self.depth_display.draw(x,y,STATS_FONT_SIZE,WHITE);
            self.record_display.draw(x,y,STATS_FONT_SIZE,WHITE);
            self.money_display.draw(x,y,STATS_FONT_SIZE,WHITE);
            self.profit_display.draw(x,y,STATS_FONT_SIZE,WHITE);
        }
        if draw_timer {
            self.timer_display.draw(x,y,STATS_FONT_SIZE,WHITE);
        }
        if draw_depth_meter {
            for d in self.depth_meter.iter() {
                d.draw(bx, y+py, 20.0, WHITE);
            }
        }
    }
    pub fn update(&mut self, py:f32, up:bool) {
        let depth = -py / 60.0;
        for dm in self.depth_meter.iter_mut() {
            let len = dm.txt.len();
            let dm_depth = match &dm.txt[0..len-2].parse::<f32>() {
                Ok(r) => *r,
                Err(e) => panic!("Error parsing: {} {}", dm.txt, e),
            };
            if !up && depth > dm_depth {
                dm.txt = HUD::layer_text(self.depth_last);
                dm.pos.1 = (60 * (self.depth_last + 1)) as f32 + DEPTH_METER_Y_OFFSET;
                self.depth_last += 1;
            } else if up && depth < dm_depth {
                dm.txt = HUD::layer_text(self.depth_last);
                dm.pos.1 = (60 * (self.depth_last + 1)) as f32 + DEPTH_METER_Y_OFFSET;
                self.depth_last -= 1;
            }
        }
    }
    pub fn change_stat(&mut self, stat: Stat, amnt: u64) {
        match stat {
            Stat::Depth => {
                self.depth_display.txt = String::from("Depth: ");
                self.depth_display.txt.push_str(&HUD::layer_text(amnt));
            },
            Stat::Money => {
                self.money_display.txt = String::from("$");
                self.money_display.txt.push_str(&amnt.to_string());
            },
            Stat::Record => {
                self.record_display.txt = String::from("(Record: ");
                self.record_display.txt.push_str(&HUD::layer_text(amnt));
                self.record_display.txt.push_str(")");
            },
            Stat::Profit => {
                self.profit_display.txt = String::from("+ $");
                self.profit_display.txt.push_str(&amnt.to_string());
            },
        }
    }
}