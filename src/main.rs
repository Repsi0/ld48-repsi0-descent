#![windows_subsystem = "windows"]
use macroquad::color::*;
use macroquad::window::clear_background;
use macroquad::window::next_frame;
use macroquad::prelude::Conf;

mod geometry;
mod game_object;
mod game;
mod sound_controller;

use crate::game::Game;

const CYAN: Color = Color::new(0.000, 0.500, 0.500, 1.000);

const WIDTH: f32 = 800.0;
const HEIGHT: f32 = 600.0;
const HUDWIDTHRATIO_INITIAL: f32 = 0.175;
const GAME_WIDTH_INITIAL: f32 = (1.0 - HUDWIDTHRATIO_INITIAL) * WIDTH;

const HUD_WIDTH_INITIAL: f32 = HUDWIDTHRATIO_INITIAL * WIDTH;
const DEPTH_X: f32 = 15.0;
const DEPTH_Y: f32 = 320.0;
const RECORD_X: f32 = DEPTH_X;
const RECORD_Y: f32 = 340.0;
const MONEY_X: f32 = DEPTH_X;
const MONEY_Y: f32 = 20.0;
const PROFIT_X: f32 = DEPTH_X;
const PROFIT_Y: f32 = 40.0;
const TIMER_X: f32 = DEPTH_X;
const TIMER_Y: f32 = HEIGHT - 15.0;
const STATS_FONT_SIZE: f32 = 18.0;
const SHOP_RESUME_X: f32 = DEPTH_X;
const SHOP_RESUME_Y: f32 = 100.0;
const SHOP_RESUME_W: f32 = 100.0;
const SHOP_RESUME_H: f32 = 50.0;
const SHOP_RESET_X: f32 = DEPTH_X;
const SHOP_RESET_Y: f32 = HEIGHT - 100.0;
const SHOP_RESET_W: f32 = 100.0;
const SHOP_RESET_H: f32 = 50.0;

const MAX_UPGRADES_GRAV: u32 = 8;
const MAX_UPGRADES_SHIELD: u32 = 20;
const MAX_UPGRADES_MISSILE: u32 = 20;
const MAX_UPGRADES_MOVE: u32 = 3;
const MAX_UPGRADES_WIN: u32 = 1;
const MAX_UPGRADES_RARITY: u32 = 9;
const MAX_UPGRADES_GAP: u32 = 4;
const MAX_UPGRADES_DEPTH_MULT: u32 = 16;
const MAX_UPGRADES_HOLE: u32 = 6;

const GRAVITY_INIT_COST: u64 = 10;
const SHIELD_INIT_COST: u64 = 20;
const MISSILE_INIT_COST: u64 = 15;
const MOVE_INIT_COST: u64 = 5;
const WIN_INIT_COST: u64 = 1000000000;
const RARITY_INIT_COST: u64 = 1;
const GAP_INIT_COST: u64 = 25;
const DEPTH_MULT_INIT_COST: u64 = 20;
const HOLE_INIT_COST: u64 = 15;

const GRAVITY_COST_MULTIPLIER: f32 = 1.3;
const SHIELD_COST_MULTIPLIER: f32 = 1.4; //
const MISSILE_COST_MULTIPLIER: f32 = 1.4; // 
const MOVE_COST_MULTIPLIER: f32 = 1.5; // 
const RARITY_COST_MULTIPLIER: f32 = 10.0; // 
const GAP_COST_MULTIPLIER: f32 = 1.5;
const DEPTH_MULT_COST_MULTIPLIER: f32 = 1.3;
const HOLE_COST_MULTIPLIER: f32 = 1.4;

const DEPTH_METER_X: f32 = 75.0;
const DEPTH_METER_Y_OFFSET: f32 = 15.0;

const PLAYERINITX: f32 = HUD_WIDTH_INITIAL + GAME_WIDTH_INITIAL * 0.5;
const PLAYERINITY: f32 = HEIGHT * 0.03;
const PLAYERSCALEX: f32 = 0.01 * WIDTH;
const PLAYERSCALEY: f32 = 0.01 * HEIGHT;
const PLAYER_MIN_X: f32 = -GAME_WIDTH_INITIAL / 2.0 + 20.0 + PLAYERSCALEX/2.0;
const PLAYER_MAX_X: f32 = GAME_WIDTH_INITIAL / 2.0 - PLAYERSCALEX;

const MISSILE_SPEED: f32 = 8.0;

const SPACINGMINIMUM: f32 = 0.1;

const LOGO_OFFSET_Y: f32 = -160.0;
const MENU_QUIT_OFFSET_Y: f32 = 128.0;

const GRAVITY_INITIAL: f32 = 0.02; //0.02 -> 0.11920928955
const GRAVITY_MULTIPLIER: f32 = 1.25;
const DEPTH_MULT_INITIAL: f32 = 0.25;
const DEPTH_MULT_MULTIPLIER: f32 = 2.0;
const GAP_INITIAL: f32 = 1.0;
const GAP_MULTIPLIER: f32 = 1.5;
const HOLE_INITIAL: f32 = 1.0;
const HOLE_MULTIPLIER: f32 = 1.25;
const SHIELD_INITIAL: u32 = 2;
const SHIELD_PER_UPGRADE: u32 = 2;
const MISSILE_INITIAL: u32 = 2;
const MISSILES_PER_UPGRADE: u32 = 2;
const MOVE_SPEED_INITIAL: f32 = 1.0;
const MOVE_SPEED_MULTIPLIER: f32 = 1.75;
const RARITIES: [u64;10] = [1,5,10,50,100,2500,10000,250000,1000000,25000000];

const THANKS_OFFSET_X: f32 = 103.0;
const LDNOTICE_OFFSET_X: f32 = 324.0;

const INVINCIBLE: bool = false;

const FALLING_SONG_PATH: &str = "res/falling.ogg";
const SHOP_SONG_PATH: &str = "res/shop.ogg";
const FINAL_SONG_PATH: &str = "res/final.ogg";
const MISSILE_SHOOT_SOUND_PATH: &str = "res/missile_shoot.ogg";
const SHIELD_LOSS_SOUND_PATH: &str = "res/shield_loss.ogg";
const LOSE_SOUND_PATH: &str = "res/lose_sound.ogg";

fn window_conf() -> Conf {
    Conf {
        window_title: String::from("DESCENT - LD48"),
        window_width: 800,
        window_height: 600,
        high_dpi: false,
        fullscreen: false,
        sample_count: 2,
        window_resizable: false,
        ..Default::default()
    }
}
#[macroquad::main(window_conf)]
async fn main() {
    let mut gamehandler = Game::new();
    let mut running = true;
    while running {
        clear_background(BLACK);
        running = gamehandler.update();
        gamehandler.draw();
        next_frame().await
    }
}
