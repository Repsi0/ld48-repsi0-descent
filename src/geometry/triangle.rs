use std::f32::consts::PI;
use macroquad::shapes::draw_line;
use macroquad::color::Color;
pub struct Triangle;

impl Triangle {
    pub fn draw(&self, x:f32, y:f32, thickness:f32,color:Color,angle_deg:f32, scale:f32) { //x and y offsets
        let angle_rad_a = (angle_deg      ) * PI / 180.0;
        let angle_rad_b = (angle_deg+120.0) * PI / 180.0;
        let angle_rad_c = (angle_deg+240.0) * PI / 180.0;
        let sin_a = scale * angle_rad_a.sin();
        let cos_a = scale * angle_rad_a.cos();
        let sin_b = scale * angle_rad_b.sin();
        let cos_b = scale * angle_rad_b.cos();
        let sin_c = scale * angle_rad_c.sin();
        let cos_c = scale * angle_rad_c.cos();
        draw_line(cos_a+x,sin_a+y,cos_b+x,sin_b+y,thickness,color);
        draw_line(cos_b+x,sin_b+y,cos_c+x,sin_c+y,thickness,color);
        draw_line(cos_c+x,sin_c+y,cos_a+x,sin_a+y,thickness,color);
    }
}