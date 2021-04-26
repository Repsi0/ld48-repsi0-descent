use macroquad::color::WHITE;

use crate::geometry::text::Text;
use crate::geometry::rectangle::Rectangle;
use crate::geometry::sprite::Sprite;
use crate::game_object::bar::Bar;

use crate::WIDTH;
use crate::HEIGHT;
use crate::HUD_WIDTH_INITIAL;
use crate::GAME_WIDTH_INITIAL;
use crate::SHOP_RESUME_X;
use crate::SHOP_RESUME_Y;
use crate::SHOP_RESUME_W;
use crate::SHOP_RESUME_H;
use crate::SHOP_RESET_X;
use crate::SHOP_RESET_Y;
use crate::SHOP_RESET_W;
use crate::SHOP_RESET_H;
use crate::STATS_FONT_SIZE;

use crate::MAX_UPGRADES_GRAV;
use crate::MAX_UPGRADES_SHIELD;
use crate::MAX_UPGRADES_MISSILE;
use crate::MAX_UPGRADES_MOVE;
use crate::MAX_UPGRADES_WIN;
use crate::MAX_UPGRADES_RARITY;
use crate::MAX_UPGRADES_GAP;
use crate::MAX_UPGRADES_DEPTH_MULT;
use crate::MAX_UPGRADES_HOLE;

use crate::GRAVITY_COST_MULTIPLIER;
use crate::SHIELD_COST_MULTIPLIER;
use crate::MISSILE_COST_MULTIPLIER;
use crate::MOVE_COST_MULTIPLIER;
use crate::RARITY_COST_MULTIPLIER;
use crate::GAP_COST_MULTIPLIER;
use crate::DEPTH_MULT_COST_MULTIPLIER;
use crate::HOLE_COST_MULTIPLIER;

const SHOP_X_OFFSET: f32 = WIDTH-100.0;
const SHOP_BUTTON_SIZE: f32 = 128.0;
const SHOP_BUTTON_PADDING: f32 = 25.0;
const SHOP_BUTTON_DIST: f32 = SHOP_BUTTON_SIZE + SHOP_BUTTON_PADDING;
const SHOP_BUTTON_THICKNESS: f32 = 2.0;
const SHOP_BAR_OFFSET_X: f32 = -SHOP_BUTTON_SIZE / 2.0;
const SHOP_BAR_OFFSET_Y: f32 = SHOP_BUTTON_SIZE / 2.0 + 5.0;
const SHOP_BAR_HEIGHT: f32 = 10.0;
const SHOP_BAR_WIDTH_INTERNAL: f32 = 124.0;

pub enum Upgrade {
    Gravity,
    Shield,
    Missile,
    Move,
    Win,
    Rarity,
    Gap,
    DepthMult,
    Hole,
}

pub struct Shop {
    hud_sep_bar: Rectangle,

    grav: Rectangle,
    shield: Rectangle,
    missile: Rectangle,
    mov: Rectangle,
    win: Rectangle,
    rarity: Rectangle,
    gap: Rectangle,
    depth_mult: Rectangle,
    hole: Rectangle,

    grav_img: Sprite,
    shield_img: Sprite,
    missile_img: Sprite,
    mov_img: Sprite,
    win_img: Sprite,
    rarity_img: Sprite,
    gap_img: Sprite,
    depth_mult_img: Sprite,
    hole_img: Sprite,

    grav_bar: Bar,
    shield_bar: Bar,
    missile_bar: Bar,
    mov_bar: Bar,
    win_bar: Bar,
    rarity_bar: Bar,
    gap_bar: Bar,
    depth_mult_bar: Bar,
    hole_bar: Bar,

    resume: Rectangle,
    resume_text: Text,
    reset: Rectangle,
    reset_text: Text,
}
impl Shop {
    pub fn new() -> Shop {
        let gw = GAME_WIDTH_INITIAL/2.0;
        let gh = HEIGHT / 2.0;
        let sb = SHOP_BUTTON_DIST;
        let left = gw-sb;
        let midx = gw;
        let right = gw+sb;
        let top = gh-sb;
        let midy = gh;
        let bot = gh+sb;

        Shop {
            hud_sep_bar: Rectangle::new(WIDTH-HUD_WIDTH_INITIAL-20.0, 0.0, 20.0, HEIGHT),
            
            grav: Rectangle::new_centered(left, top, SHOP_BUTTON_SIZE, SHOP_BUTTON_SIZE),
            shield: Rectangle::new_centered(midx, top, SHOP_BUTTON_SIZE, SHOP_BUTTON_SIZE),
            missile: Rectangle::new_centered(right, top, SHOP_BUTTON_SIZE, SHOP_BUTTON_SIZE),
            mov: Rectangle::new_centered(left, midy, SHOP_BUTTON_SIZE, SHOP_BUTTON_SIZE),
            win: Rectangle::new_centered(midx, midy, SHOP_BUTTON_SIZE, SHOP_BUTTON_SIZE),
            rarity: Rectangle::new_centered(right, midy, SHOP_BUTTON_SIZE, SHOP_BUTTON_SIZE),
            gap: Rectangle::new_centered(left, bot, SHOP_BUTTON_SIZE, SHOP_BUTTON_SIZE),
            depth_mult: Rectangle::new_centered(midx, bot, SHOP_BUTTON_SIZE, SHOP_BUTTON_SIZE),
            hole: Rectangle::new_centered(right, bot, SHOP_BUTTON_SIZE, SHOP_BUTTON_SIZE),

            grav_img: Sprite::new_centered(left, top, "res/gravity.png"),
            shield_img: Sprite::new_centered(midx, top, "res/shield.png"),
            missile_img: Sprite::new_centered(right, top, "res/missile.png"),
            mov_img: Sprite::new_centered(left, midy, "res/move.png"),
            win_img: Sprite::new_centered(midx, midy, "res/ld48.png"),
            rarity_img: Sprite::new_centered(right, midy, "res/rarity.png"),
            gap_img: Sprite::new_centered(left, bot, "res/gap.png"),
            depth_mult_img: Sprite::new_centered(midx, bot, "res/wage.png"),
            hole_img: Sprite::new_centered(right, bot, "res/hole.png"),
            
            grav_bar: Bar::new(left + SHOP_BAR_OFFSET_X, top + SHOP_BAR_OFFSET_Y, SHOP_BUTTON_SIZE, SHOP_BAR_HEIGHT, SHOP_BAR_WIDTH_INTERNAL, MAX_UPGRADES_GRAV),
            shield_bar: Bar::new(midx + SHOP_BAR_OFFSET_X, top + SHOP_BAR_OFFSET_Y, SHOP_BUTTON_SIZE, SHOP_BAR_HEIGHT, SHOP_BAR_WIDTH_INTERNAL, MAX_UPGRADES_SHIELD),
            missile_bar: Bar::new(right + SHOP_BAR_OFFSET_X, top + SHOP_BAR_OFFSET_Y, SHOP_BUTTON_SIZE, SHOP_BAR_HEIGHT, SHOP_BAR_WIDTH_INTERNAL, MAX_UPGRADES_MISSILE),
            mov_bar: Bar::new(left + SHOP_BAR_OFFSET_X, midy + SHOP_BAR_OFFSET_Y, SHOP_BUTTON_SIZE, SHOP_BAR_HEIGHT, SHOP_BAR_WIDTH_INTERNAL, MAX_UPGRADES_MOVE),
            win_bar: Bar::new(midx + SHOP_BAR_OFFSET_X, midy + SHOP_BAR_OFFSET_Y, SHOP_BUTTON_SIZE, SHOP_BAR_HEIGHT, SHOP_BAR_WIDTH_INTERNAL, MAX_UPGRADES_WIN),
            rarity_bar: Bar::new(right + SHOP_BAR_OFFSET_X, midy + SHOP_BAR_OFFSET_Y, SHOP_BUTTON_SIZE, SHOP_BAR_HEIGHT, SHOP_BAR_WIDTH_INTERNAL, MAX_UPGRADES_RARITY),
            gap_bar: Bar::new(left + SHOP_BAR_OFFSET_X, bot + SHOP_BAR_OFFSET_Y, SHOP_BUTTON_SIZE, SHOP_BAR_HEIGHT, SHOP_BAR_WIDTH_INTERNAL, MAX_UPGRADES_GAP),
            depth_mult_bar: Bar::new(midx + SHOP_BAR_OFFSET_X, bot + SHOP_BAR_OFFSET_Y, SHOP_BUTTON_SIZE, SHOP_BAR_HEIGHT, SHOP_BAR_WIDTH_INTERNAL, MAX_UPGRADES_DEPTH_MULT),
            hole_bar: Bar::new(right + SHOP_BAR_OFFSET_X, bot + SHOP_BAR_OFFSET_Y, SHOP_BUTTON_SIZE, SHOP_BAR_HEIGHT, SHOP_BAR_WIDTH_INTERNAL, MAX_UPGRADES_HOLE),

            resume: Rectangle::new(SHOP_X_OFFSET-SHOP_RESUME_X, SHOP_RESUME_Y, SHOP_RESUME_W, SHOP_RESUME_H),
            resume_text: Text::new(SHOP_X_OFFSET-SHOP_RESUME_X+15.0, SHOP_RESUME_Y+32.0, String::from("Resume")),
            reset: Rectangle::new(SHOP_X_OFFSET-SHOP_RESET_X, SHOP_RESET_Y, SHOP_RESET_W, SHOP_RESET_H),
            reset_text: Text::new(SHOP_X_OFFSET-SHOP_RESET_X+15.0, SHOP_RESET_Y+32.0, String::from("Reset")),
        }
    }
    pub fn draw(&mut self, x:f32, y:f32) {
        self.hud_sep_bar.draw(x,y,WHITE);

        self.grav.draw_outline(x,y,SHOP_BUTTON_THICKNESS,WHITE);
        self.shield.draw_outline(x,y,SHOP_BUTTON_THICKNESS,WHITE);
        self.missile.draw_outline(x,y,SHOP_BUTTON_THICKNESS,WHITE);
        self.mov.draw_outline(x,y,SHOP_BUTTON_THICKNESS,WHITE);
        self.win.draw_outline(x,y,SHOP_BUTTON_THICKNESS,WHITE);
        self.rarity.draw_outline(x,y,SHOP_BUTTON_THICKNESS,WHITE);
        self.gap.draw_outline(x,y,SHOP_BUTTON_THICKNESS,WHITE);
        self.depth_mult.draw_outline(x,y,SHOP_BUTTON_THICKNESS,WHITE);
        self.hole.draw_outline(x,y,SHOP_BUTTON_THICKNESS,WHITE);

        self.grav_img.draw(x,y,WHITE);
        self.shield_img.draw(x,y,WHITE);
        self.missile_img.draw(x,y,WHITE);
        self.mov_img.draw(x,y,WHITE);
        self.win_img.draw(x,y,WHITE);
        self.rarity_img.draw(x,y,WHITE);
        self.gap_img.draw(x,y,WHITE);
        self.depth_mult_img.draw(x,y,WHITE);
        self.hole_img.draw(x,y,WHITE);

        self.grav_bar.draw(x,y,WHITE);
        self.shield_bar.draw(x,y,WHITE);
        self.missile_bar.draw(x,y,WHITE);
        self.mov_bar.draw(x,y,WHITE);
        self.win_bar.draw(x,y,WHITE);
        self.rarity_bar.draw(x,y,WHITE);
        self.gap_bar.draw(x,y,WHITE);
        self.depth_mult_bar.draw(x,y,WHITE);
        self.hole_bar.draw(x,y,WHITE);

        self.resume.draw_outline(x,y,4.0,WHITE);
        self.resume_text.draw(x,y,STATS_FONT_SIZE * 1.5,WHITE);
        self.reset.draw_outline(x,y,4.0,WHITE);
        self.reset_text.draw(x,y,STATS_FONT_SIZE * 1.5, WHITE);
    }

    pub fn num_upgrades(&self, upgrade:Upgrade) -> u32 {
        match upgrade {
            Upgrade::Gravity => self.grav_bar.numenabled,
            Upgrade::Shield => self.shield_bar.numenabled,
            Upgrade::Missile => self.missile_bar.numenabled,
            Upgrade::Move => self.mov_bar.numenabled,
            Upgrade::Win => self.win_bar.numenabled,
            Upgrade::Rarity => self.rarity_bar.numenabled,
            Upgrade::Gap => self.gap_bar.numenabled,
            Upgrade::DepthMult => self.depth_mult_bar.numenabled,
            Upgrade::Hole => self.hole_bar.numenabled,
        }
    }

    fn cost_increase(upgrade:Upgrade, init: &[u64;9], money: u64) -> ([u64;9],u64,bool,bool,bool) {
        let mut ret: [u64;9] = [0;9];
        ret.clone_from_slice(init);
        match upgrade {
            Upgrade::Gravity => {
                ret[0] = (ret[0] as f32 * GRAVITY_COST_MULTIPLIER) as u64;
                return (ret,money-init[0],false,false,false);
            },
            Upgrade::Shield => {
                ret[1] = (ret[1] as f32 * SHIELD_COST_MULTIPLIER) as u64;
                return (ret,money-init[1],false,false,false);
            },
            Upgrade::Missile => {
                ret[2] = (ret[2] as f32 * MISSILE_COST_MULTIPLIER) as u64;
                return (ret,money-init[2],false,false,false);
            },
            Upgrade::Move => {
                ret[3] = (ret[3] as f32 * MOVE_COST_MULTIPLIER) as u64;
                return (ret,money-init[3],false,false,false);
            },
            Upgrade::Win => {
                return (ret,money-init[4],true,false,false);
            },
            Upgrade::Rarity => {
                ret[5] = (ret[5] as f32 * RARITY_COST_MULTIPLIER) as u64;
                return (ret,money-init[5],false,false,false);
            },
            Upgrade::Gap => {
                ret[6] = (ret[6] as f32 * GAP_COST_MULTIPLIER) as u64;
                return (ret,money-init[6],false,false,false);
            },
            Upgrade::DepthMult => {
                ret[7] = (ret[7] as f32 * DEPTH_MULT_COST_MULTIPLIER) as u64;
                return (ret,money-init[7],false,false,false);
            },
            Upgrade::Hole => {
                ret[8] = (ret[8] as f32 * HOLE_COST_MULTIPLIER) as u64;
                return (ret,money-init[8],false,false,false);
            },
        }
    }

    pub fn click(&mut self, x: f32, y: f32, upgrade_cost: &[u64;9], money: u64) -> ([u64;9],u64,bool,bool,bool) {
        if self.resume.collides_pt(x, y) {
            return (*upgrade_cost,money,false,true,false);
        } else if self.reset.collides_pt(x,y) {
            self.grav_bar.numenabled = 0;
            self.shield_bar.numenabled = 0;
            self.missile_bar.numenabled = 0;
            self.mov_bar.numenabled = 0;
            self.win_bar.numenabled = 0;
            self.rarity_bar.numenabled = 0;
            self.gap_bar.numenabled = 0;
            self.depth_mult_bar.numenabled = 0;
            self.hole_bar.numenabled = 0;
            return (*upgrade_cost,money,false,false,true);
        } else if self.grav.collides_pt(x,y) {
            if money >= upgrade_cost[0] && self.grav_bar.buy() {
                return Shop::cost_increase(Upgrade::Gravity, upgrade_cost, money);
            }
        } else if self.shield.collides_pt(x, y) {
            if money >= upgrade_cost[1] && self.shield_bar.buy() {
                return Shop::cost_increase(Upgrade::Shield, upgrade_cost, money);
            }
        } else if self.missile.collides_pt(x, y) {
            if money >= upgrade_cost[2] && self.missile_bar.buy() {
                return Shop::cost_increase(Upgrade::Missile, upgrade_cost, money);
            }
        } else if self.mov.collides_pt(x, y) {
            if money >= upgrade_cost[3] && self.mov_bar.buy() {
                return Shop::cost_increase(Upgrade::Move, upgrade_cost, money);
            }
        } else if self.win.collides_pt(x, y) {
            if money >= upgrade_cost[4] && self.win_bar.buy() {
                return Shop::cost_increase(Upgrade::Win, upgrade_cost, money);
            }
        } else if self.rarity.collides_pt(x, y) {
            if money >= upgrade_cost[5] && self.rarity_bar.buy() {
                return Shop::cost_increase(Upgrade::Rarity, upgrade_cost, money);
            }
        } else if self.gap.collides_pt(x, y) {
            if money >= upgrade_cost[6] && self.gap_bar.buy() {
                return Shop::cost_increase(Upgrade::Gap, upgrade_cost, money);
            }
        } else if self.depth_mult.collides_pt(x, y) {
            if money >= upgrade_cost[7] && self.depth_mult_bar.buy() {
                return Shop::cost_increase(Upgrade::DepthMult, upgrade_cost, money);
            }
        } else if self.hole.collides_pt(x, y) {
            if money >= upgrade_cost[8] && self.hole_bar.buy() {
                return Shop::cost_increase(Upgrade::Hole, upgrade_cost, money);
            }
        }
        return (*upgrade_cost,money,false,false,false);
    }
}