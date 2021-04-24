use macroquad::prelude::*;

#[macroquad::main("BasicShapes")]
struct Layer {
    value: u64,
    layer: u64,
    color: Color,
    rainbow: bool,
}
impl Layer {
    fn new(ly:u64, value:u64) -> Layer {
        Layer {
            layer: ly,
            value: val,
            color: match val {
                     0 ..=     24 => DARKGRAY,
                    25 ..=     50 => LIGHTGRAY,
                   100 ..=    499 => WHITE,
                   500 ..=    999 => RED,
                  1000 ..=   2499 => ORANGE,
                  2500 ..=   9999 => YELLOW,
                 10000 ..=  24999 => GREEN,
                 25000 ..=  99999 => BLUE,
                100000 ..= 249999 => INDIGO,
                _                 => VIOLET,
            },
            rainbow: (value >= 1000000),
        }
    }
}

fn draw_hud() {
    draw_rectangle(60.0, 0.0, 20.0, screen_height(), WHITE);
}
fn draw_timer(frame: usize) {
    let timestr = String::from("-");
}

async fn main() {
    //maybe add speedrun counter?
    let frame: usize = 0;
    loop {
        clear_background(RED);
        
        draw_rectangle(60.0, 60.0, screen_width() * 0.4, 20.0, WHITE);
        draw_rectangle(screen_width()*0.8, 60, screen_width()*0.2, 20.0, WHITE);
        draw_text("-1 m", 20.0, 30.0, 20.0, LIGHTGRAY);
        draw_hud();
        
        frame += 1;
        draw_timer();
        
        next_frame().await
    }
}
