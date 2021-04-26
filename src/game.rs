use macroquad::color::WHITE;
use macroquad::input::MouseButton;
use macroquad::input::is_mouse_button_pressed;
use macroquad::input::mouse_position;
use macroquad::input::KeyCode;
use macroquad::input::is_key_pressed;

use rand::rngs::ThreadRng;
use rand::Rng;

use crate::game_object::player::Player;
use crate::game_object::layer::Layer;
use crate::game_object::hud::HUD;
use crate::game_object::hud::Stat;
use crate::game_object::shop::Shop;
use crate::game_object::shop::Upgrade;
use crate::game_object::menu::Menu;
use crate::game_object::projectile::Missile;
use crate::game_object::final_layer::FinalLayer;
use crate::sound_controller::SoundController;
use crate::geometry::text::Text;

use crate::geometry::rectangle::Rectangle;

use crate::HUDWIDTHRATIO_INITIAL;
use crate::HUD_WIDTH_INITIAL;
use crate::GAME_WIDTH_INITIAL;
use crate::SPACINGMINIMUM;
use crate::WIDTH;
use crate::HEIGHT;

use crate::GRAVITY_INIT_COST;
use crate::SHIELD_INIT_COST;
use crate::MISSILE_INIT_COST;
use crate::MOVE_INIT_COST;
use crate::WIN_INIT_COST;
use crate::RARITY_INIT_COST;
use crate::GAP_INIT_COST;
use crate::DEPTH_MULT_INIT_COST;
use crate::HOLE_INIT_COST;

use crate::GRAVITY_INITIAL;
use crate::GRAVITY_MULTIPLIER;
use crate::DEPTH_MULT_INITIAL;
use crate::DEPTH_MULT_MULTIPLIER;
use crate::GAP_INITIAL;
use crate::GAP_MULTIPLIER;
use crate::HOLE_INITIAL;
use crate::HOLE_MULTIPLIER;
use crate::SHIELD_INITIAL;
use crate::SHIELD_PER_UPGRADE;
use crate::MISSILE_INITIAL;
use crate::MISSILES_PER_UPGRADE;
use crate::MOVE_SPEED_INITIAL;
use crate::MOVE_SPEED_MULTIPLIER;
use crate::RARITIES;
use crate::INVINCIBLE;
use crate::THANKS_OFFSET_X;
use crate::LDNOTICE_OFFSET_X;

pub enum GameState {
    MENU,
    SHOP,
    GAME
}

pub struct Game {
    rng:ThreadRng,
    layers:Vec<Layer>,
    layer:u64,
    finallayer:FinalLayer,
    player:Player,
    hud:HUD,
    shop:Shop,
    menu:Menu,
    soundcontroller:SoundController,

    profit:f32, 
    depth:u64,
    record:u64,
    timer:u64,

    depth_mult:f32,
    gap:f32,
    hole:f32,
    missiles:Vec<Missile>,
    rarity:usize,

    gs: GameState,
    upgrade_cost: [u64;9],
    game_to_shop_transition: bool,
    shop_to_menu_transition: bool,
    shop_to_game_transition: bool,
    menu_to_game_transition: bool,
    pre_cutscene: bool,
    cutscene: bool,
    hud_smoosh_transition: bool,
    post_smoosh: bool,
    liftoff: bool,
    ldvisible: bool,
    player_angle_deg: f32,
    transition_x: f32,
    transition_x_2: f32,
    transition_y: f32,
    transition_y_2: f32,
    transition_vel_y_2: f32,
    thanksforplaying: Text,
    ldnotice: Text,
}
impl Game {
    fn gen_space(rng: &mut ThreadRng, layer: u64, gap_mult: f32, hole_mult: f32) -> Rectangle {
        Rectangle {
            x: HUD_WIDTH_INITIAL + rng.gen::<f32>() * 0.5 * GAME_WIDTH_INITIAL + 0.2 * GAME_WIDTH_INITIAL,
            y: (layer+1) as f32 * 80.0 * gap_mult,
            w: GAME_WIDTH_INITIAL * (SPACINGMINIMUM + rng.gen::<f32>() * 0.1) * hole_mult,
            h: 20.0,
        }
    }
    pub fn new() -> Game {
        let mut rng = rand::thread_rng();

        let player: Player = Player::new(GRAVITY_INITIAL,SHIELD_INITIAL,MISSILE_INITIAL,MOVE_SPEED_INITIAL,INVINCIBLE);
        let hud: HUD = HUD::new();
        let shop: Shop = Shop::new();
        let profit: f32 = 0.0;
        let depth: u64 = 0;
        let record: u64 = 0;
        let timer: u64 = 0;

        let gs = GameState::MENU;

        let mut layers: Vec<Layer> = vec!();
        let mut layer: u64 = 0;
        for i in 0..25 {
            layers.push(Layer::new(Game::gen_space(&mut rng,i,GAP_INITIAL,HOLE_INITIAL),RARITIES[0],i,HUD_WIDTH_INITIAL));
            layer += 1;
        }
        Game {
            rng,layers,layer,player,hud,shop,profit,depth,record,timer,gs,
            upgrade_cost: [ GRAVITY_INIT_COST,
                            SHIELD_INIT_COST,
                            MISSILE_INIT_COST,
                            MOVE_INIT_COST,
                            WIN_INIT_COST,
                            RARITY_INIT_COST,
                            GAP_INIT_COST,
                            DEPTH_MULT_INIT_COST,
                            HOLE_INIT_COST        ],
            game_to_shop_transition: false,
            shop_to_menu_transition: false,
            shop_to_game_transition: false,
            menu_to_game_transition: false,
            pre_cutscene: false,
            cutscene: false,
            hud_smoosh_transition: false,
            post_smoosh: false,
            liftoff: false,
            ldvisible: false,
            player_angle_deg: 0.0,
            transition_x: 0.0,
            transition_x_2: 0.0,
            transition_y: 0.0,
            transition_y_2: 0.0,
            transition_vel_y_2: 0.0,
            menu: Menu::new(),
            depth_mult: DEPTH_MULT_INITIAL,
            gap: GAP_INITIAL,
            hole: HOLE_INITIAL,
            missiles: Vec::new(),
            rarity: 0,
            finallayer: FinalLayer::new(),
            soundcontroller: SoundController::new(),
            thanksforplaying: Text::new(WIDTH/2.0 - THANKS_OFFSET_X, 3.0*HEIGHT/4.0, String::from("Thanks for playing!")),
            ldnotice: Text::new(WIDTH/2.0 - LDNOTICE_OFFSET_X, 4.0*HEIGHT/5.0, String::from("Made by Repsi0 in 48 hours for Ludum Dare 48")),
        }
    }
    pub fn draw(&mut self) {
        if !(self.game_to_shop_transition || self.shop_to_menu_transition || self.shop_to_game_transition || self.menu_to_game_transition || self.hud_smoosh_transition || self.cutscene) {
            match self.gs {
                GameState::MENU => {
                    self.menu.draw(0.0, 0.0);
                },
                GameState::GAME => {
                    for m in self.missiles.iter_mut() {
                        m.draw(0.0,self.player.y,WHITE);
                    }
                    self.player.draw(0.0, 0.0, WHITE, 0.0);

                    for l in self.layers.iter_mut() {
                        l.draw(0.0, self.player.y);
                    }
                    self.finallayer.draw(0.0,self.player.y);
                    
                    self.hud.draw(0.0,0.0,self.player.y,0.0, true, true, true);
                },
                GameState::SHOP => {
                    self.shop.draw(0.0,0.0);
                    self.hud.draw(WIDTH - HUD_WIDTH_INITIAL,0.0,9999.0,WIDTH, true, true, true);
                },
            };
        } else {
            self.handle_transitions();
        }
    }
    fn update_stats(&mut self) {
        let realdepth: f32 = -self.player.y / 60.0;
        self.depth = realdepth.floor() as u64;
        self.hud.change_stat(Stat::Depth, self.depth);
        if self.depth > self.record {
            self.record = self.depth;
            self.hud.change_stat(Stat::Record, self.record);
        }
        self.hud.change_stat(Stat::Profit, (self.profit + realdepth * self.depth_mult) as u64);
    }
    pub fn update(&mut self) -> bool {
        if !(self.game_to_shop_transition || self.shop_to_menu_transition || self.shop_to_game_transition || self.menu_to_game_transition || self.cutscene || self.hud_smoosh_transition) {
            match self.gs {
                GameState::MENU => {},
                GameState::GAME => {
                    self.player.update();
                    self.hud.update(self.player.y,false);
                    for m in self.missiles.iter_mut() {
                        m.update();
                    }
                    self.update_stats();
                    if self.depth >= 9500 && self.depth < 9999 {
                        self.player.grav = -2.25;
                    } else if self.depth == 9999 {
                        self.player.grav = 0.0;
                        self.player.vel_y = 0.0;
                        self.pre_cutscene = self.player.invincible;
                    }
                    for l in self.layers.iter_mut() {
                        let num = self.player.check_collision(&l.body1) + self.player.check_collision(&l.body2);
                        if  num > 0 && num < 2 {
                            self.soundcontroller.play_lose();
                            self.player.money += self.profit + (-self.player.y / 60.0) * self.depth_mult;
                            self.profit = 0.0;
                            self.hud.change_stat(Stat::Profit, 0);
                            self.hud.change_stat(Stat::Money, self.player.money as u64);
                            self.game_to_shop_transition = true;
                            self.soundcontroller.stop_falling();
                            self.soundcontroller.play_shop();
                            self.layer = 25;
                            continue;
                        } else if num != 0 {
                            if num == 3 && !self.player.invincible {
                                self.soundcontroller.play_shield();
                            }
                            self.profit += l.value as f32;
                            if self.depth < 9500 {
                                l.change(Game::gen_space(&mut self.rng,self.layer,self.gap,self.hole), RARITIES[self.rarity], self.layer,HUD_WIDTH_INITIAL);
                            } else {
                                l.change(Game::gen_space(&mut self.rng,0,self.gap,self.hole), RARITIES[self.rarity], 99999, 0.0);
                            }
                            self.layer += 1;
                        }
                        let mut missiles_to_remove: Vec<usize> = Vec::new();
                        let mut missile_index = 0;
                        for m in self.missiles.iter_mut() {
                            if m.check_collision(&l.body1) || m.check_collision(&l.body2) {
                                self.profit += l.value as f32;
                                if self.depth < 9500 {
                                    l.change(Game::gen_space(&mut self.rng,self.layer,self.gap,self.hole), RARITIES[self.rarity], self.layer,HUD_WIDTH_INITIAL);
                                } else {
                                    l.change(Game::gen_space(&mut self.rng,self.layer,self.gap,self.hole), RARITIES[self.rarity], 99999, 0.0);
                                }
                                self.layer += 1;
                                missiles_to_remove.push(missile_index);
                                self.soundcontroller.play_missile();
                            }
                            missile_index += 1;
                        }
                        for m in missiles_to_remove.iter() {
                            self.missiles.remove(*m);
                        }
                    }
                    let mut missiles_to_remove: Vec<usize> = Vec::new();
                    let mut missile_index = 0;
                    for m in self.missiles.iter_mut() {
                        if self.finallayer.check_collision(&m.bounds) {
                            missiles_to_remove.push(missile_index);
                            if self.pre_cutscene {
                                self.pre_cutscene = false;
                                self.cutscene = true;
                                self.finallayer.draw = false;
                            }
                        }
                        missile_index += 1;
                    }
                    for m in missiles_to_remove.iter() {
                        self.missiles.remove(*m);
                    }
                    if is_key_pressed(KeyCode::Escape) {
                        self.player.money += self.profit + (-self.player.y / 60.0) * self.depth_mult;
                        self.profit = 0.0;
                        self.hud.change_stat(Stat::Profit, 0);
                        self.hud.change_stat(Stat::Money, self.player.money as u64);
                        self.game_to_shop_transition = true;
                        self.soundcontroller.stop_falling();
                        self.soundcontroller.play_shop();
                        self.layer = 25;
                    } else if is_key_pressed(KeyCode::Z) {
                        if self.player.missileleft > 0 {
                            self.missiles.push(Missile::new(self.player.x + HUD_WIDTH_INITIAL + GAME_WIDTH_INITIAL / 2.0,16.0-self.player.y, self.player.vel_y, self.player.grav));
                            self.player.missileleft -= 1;
                        }
                    }
                },
                GameState::SHOP => {
                    if is_key_pressed(KeyCode::Escape) {
                        self.shop_to_menu_transition = true;
                        self.soundcontroller.stop_shop();
                        self.layer = 25;
                    }
                }
            }
            if is_mouse_button_pressed(MouseButton::Left) {
                match self.gs {
                    GameState::MENU => {
                        let pos = mouse_position();
                        let res = self.menu.click(pos.0, pos.1);
                        if res.0 {
                            self.hud.reset();
                            self.player.grav = GRAVITY_INITIAL * GRAVITY_MULTIPLIER.powi(self.shop.num_upgrades(Upgrade::Gravity) as i32);
                            self.player.shield = SHIELD_INITIAL + SHIELD_PER_UPGRADE * self.shop.num_upgrades(Upgrade::Shield);
                            self.player.missile = MISSILE_INITIAL + MISSILES_PER_UPGRADE * self.shop.num_upgrades(Upgrade::Missile);
                            self.player.move_speed = MOVE_SPEED_INITIAL * MOVE_SPEED_MULTIPLIER.powi(self.shop.num_upgrades(Upgrade::Move) as i32);
                            self.player.invincible = INVINCIBLE || self.shop.num_upgrades(Upgrade::Win) == 1;
                            self.player.reset();
                            self.depth_mult = DEPTH_MULT_INITIAL * DEPTH_MULT_MULTIPLIER.powi(self.shop.num_upgrades(Upgrade::DepthMult) as i32);
                            self.gap = GAP_INITIAL * GAP_MULTIPLIER.powi(self.shop.num_upgrades(Upgrade::Gap) as i32);
                            self.hole = HOLE_INITIAL * HOLE_MULTIPLIER.powi(self.shop.num_upgrades(Upgrade::Hole) as i32);
                            self.rarity = self.shop.num_upgrades(Upgrade::Rarity) as usize;
                            self.missiles = Vec::new();
                            let mut layers: Vec<Layer> = vec!();
                            for i in 0..25 {
                                layers.push(Layer::new(Game::gen_space(&mut self.rng,i,self.gap,self.hole),RARITIES[self.rarity],i,HUD_WIDTH_INITIAL));
                            }
                            self.layers = layers;
                            self.menu_to_game_transition = true;
                            self.soundcontroller.play_falling();
                            self.layer = 25;
                        } else if res.1 {
                            return false;
                        }
                    },
                    GameState::GAME => {},
                    GameState::SHOP => {
                        let pos = mouse_position();
                        let res = self.shop.click(pos.0, pos.1, &self.upgrade_cost, self.player.money as u64);
                        self.player.money = res.1 as f32;
                        self.hud.change_stat(Stat::Money, self.player.money as u64);
                        self.upgrade_cost = res.0;
                        if res.4 {
                            self.upgrade_cost = [ GRAVITY_INIT_COST,
                            SHIELD_INIT_COST,
                            MISSILE_INIT_COST,
                            MOVE_INIT_COST,
                            WIN_INIT_COST,
                            RARITY_INIT_COST,
                            GAP_INIT_COST,
                            DEPTH_MULT_INIT_COST,
                            HOLE_INIT_COST ];
                        }
                        if res.3 {
                            self.shop_to_game_transition = true;
                            self.soundcontroller.stop_shop();
                            self.soundcontroller.play_falling();
                            self.layer = 25;
                            self.player.grav = GRAVITY_INITIAL * GRAVITY_MULTIPLIER.powi(self.shop.num_upgrades(Upgrade::Gravity) as i32);
                            self.player.shield = SHIELD_INITIAL + SHIELD_PER_UPGRADE * self.shop.num_upgrades(Upgrade::Shield);
                            self.player.missile = MISSILE_INITIAL + MISSILES_PER_UPGRADE * self.shop.num_upgrades(Upgrade::Missile);
                            self.player.move_speed = MOVE_SPEED_INITIAL * MOVE_SPEED_MULTIPLIER.powi(self.shop.num_upgrades(Upgrade::Move) as i32);
                            self.player.invincible = INVINCIBLE || self.shop.num_upgrades(Upgrade::Win) == 1;
                            self.depth_mult = DEPTH_MULT_INITIAL * DEPTH_MULT_MULTIPLIER.powi(self.shop.num_upgrades(Upgrade::DepthMult) as i32);
                            self.gap = GAP_INITIAL * GAP_MULTIPLIER.powi(self.shop.num_upgrades(Upgrade::Gap) as i32);
                            self.hole = HOLE_INITIAL * HOLE_MULTIPLIER.powi(self.shop.num_upgrades(Upgrade::Hole) as i32);
                            self.rarity = self.shop.num_upgrades(Upgrade::Rarity) as usize;
                            self.player.reset();
                            self.hud.reset();
                            self.missiles = Vec::new();
                            self.transition_x = WIDTH;
                            let mut layers: Vec<Layer> = vec!();
                            for i in 0..25 {
                                layers.push(Layer::new(Game::gen_space(&mut self.rng,i,self.gap,self.hole),RARITIES[self.rarity],i,HUD_WIDTH_INITIAL));
                            }
                            self.layers = layers;
                        }
                    }
                }
            }
        }
        self.timer += 1;
        return true;
    }
    fn reset_transitions(&mut self) {
        self.transition_x = 0.0;
        self.transition_y = 0.0;
        self.game_to_shop_transition = false;
        self.shop_to_menu_transition = false;
        self.shop_to_game_transition = false;
        self.menu_to_game_transition = false;
    }
    fn handle_transitions(&mut self) {
        if self.game_to_shop_transition {
            self.player.draw(self.transition_x, self.transition_y, WHITE, 0.0);
            for l in self.layers.iter_mut() {
                l.draw(self.transition_x, self.player.y + self.transition_y);
            }
            
            self.hud.draw(self.transition_x - HUDWIDTHRATIO_INITIAL * self.transition_x, self.transition_y, self.player.y, self.transition_x, true, true, true);
            self.shop.draw(-WIDTH + self.transition_x, self.transition_y);
            
            self.transition_x += 20.0;
            if self.transition_x >= WIDTH {
                self.gs = GameState::SHOP;
                self.reset_transitions();
            }
        } else if self.shop_to_game_transition {
            self.player.draw(self.transition_x, self.transition_y, WHITE, 0.0);
            for l in self.layers.iter_mut() {
                l.draw(self.transition_x, self.player.y + self.transition_y);
            }
            
            self.hud.draw(self.transition_x - HUD_WIDTH_INITIAL + HUDWIDTHRATIO_INITIAL * (WIDTH-self.transition_x),self.transition_y,self.player.y,self.transition_x, true, true, true);
            self.shop.draw(-WIDTH + self.transition_x, self.transition_y);
            
            self.transition_x -= 20.0;
            if self.transition_x <= 0.0 {
                self.gs = GameState::GAME;
                self.reset_transitions();
            }
        } else if self.shop_to_menu_transition {
            self.hud.draw(WIDTH - HUD_WIDTH_INITIAL + self.transition_x, self.transition_y, self.player.y, WIDTH - HUD_WIDTH_INITIAL + self.transition_x, false, true, true);
            self.shop.draw(self.transition_x, self.transition_y);

            self.transition_y += 20.0;
            if self.transition_y >= HEIGHT {
                self.gs = GameState::MENU;
                self.reset_transitions();
            }
        } else if self.menu_to_game_transition {
            self.hud.draw(WIDTH - self.transition_x, self.transition_y, self.player.y, WIDTH - self.transition_x, true, true, true);
            self.player.draw(WIDTH - self.transition_x, self.transition_y, WHITE, 0.0);
            for l in self.layers.iter_mut() {
                l.draw(WIDTH - self.transition_x, self.player.y + self.transition_y);
            }
            self.menu.draw(-self.transition_x, self.transition_y);
            
            self.transition_x += 20.0;
            if self.transition_x >= WIDTH {
                self.gs = GameState::GAME;
                self.reset_transitions();
            }
        } else if self.cutscene && !self.hud_smoosh_transition {
            self.hud.draw(self.transition_x, self.transition_y, self.player.y, self.transition_x, true, true, false);
            self.player.draw(self.transition_x, self.transition_y, WHITE, self.player_angle_deg);
            if self.player_angle_deg == 0.0 {
                self.soundcontroller.stop_falling();
                self.soundcontroller.play_final();
            }
            self.player_angle_deg += 1.0;
            if self.player_angle_deg == 180.0 {
                self.reset_transitions();
                self.hud_smoosh_transition = true;
            }
        } else if self.cutscene && !self.post_smoosh {
            self.hud.draw(self.transition_x, self.transition_y, self.player.y, self.transition_x, true, false, false);
            self.player.draw((1.0-HUDWIDTHRATIO_INITIAL) * self.transition_x, self.transition_y, WHITE, 180.0);
            self.transition_x -= 1.0;
            if self.transition_x == -(HUD_WIDTH_INITIAL - 80.0) {
                self.post_smoosh = true;
                self.transition_vel_y_2 = 1.0;
            }
        } else if self.cutscene && !self.liftoff {
            self.hud.draw(self.transition_x + self.transition_x_2, self.transition_y, self.player.y, self.transition_x + self.transition_x_2, true, false, false);
            self.player.draw((1.0-HUDWIDTHRATIO_INITIAL) * self.transition_x, self.transition_y + self.transition_y_2, WHITE, 180.0);
            self.transition_vel_y_2 += 0.2;
            let xdone = self.transition_x_2 + self.transition_x <= -128.0;
            let ydone = self.transition_y_2 >= HEIGHT / 2.0;
            if !xdone {
                self.transition_x_2 -= 1.0;
            }
            if !ydone {
                self.transition_y_2 += self.transition_vel_y_2;
            } else if xdone && ydone {
                self.liftoff = true;
                self.player.grav = -0.02;
            }
        } else if self.cutscene && !self.ldvisible {
            self.player.draw((1.0-HUDWIDTHRATIO_INITIAL) * self.transition_x, self.transition_y + self.transition_y_2, WHITE, 180.0);
            self.player.update();
            self.thanksforplaying.draw(self.transition_x, self.transition_y, 40.0, WHITE);
            self.ldnotice.draw(self.transition_x, self.transition_y, 40.0, WHITE);
            self.transition_y -= 5.0;
            if self.transition_y <= -HEIGHT * 11.0 / 40.0 {
                self.ldvisible = true;
            }
        } else if self.cutscene {
            self.player.draw((1.0-HUDWIDTHRATIO_INITIAL) * self.transition_x, self.transition_y + self.transition_y_2, WHITE, 180.0);
            self.player.update();
            self.thanksforplaying.draw(self.transition_x, self.transition_y, 40.0, WHITE);
            self.ldnotice.draw(self.transition_x, self.transition_y, 40.0, WHITE);
            if self.transition_y + self.transition_y_2 > -64.0 {
                self.transition_y_2 -= 5.0;
            }
        }
    }
}