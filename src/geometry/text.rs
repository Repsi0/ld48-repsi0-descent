use macroquad::text::draw_text;
//use macroquad::text::draw_text_ex;
//use macroquad::text::TextParams;
use macroquad::color::Color;
use crate::geometry::vertex::Vertex;
pub struct Text {
    pub pos: Vertex,
    pub txt: String,
}

impl Text {
    pub fn new(x:f32, y:f32, txt:String) -> Text {
        Text {
            pos:Vertex(x,y),
            txt
        }
    }
    //pub fn draw_ex(&mut self, x:f32, y:f32, text_params:TextParams) {
    //    draw_text_ex(&self.txt,self.pos.0+x,self.pos.1+y,text_params);
    //}
    pub fn draw(&self, x:f32, y:f32, font_size:f32, color:Color) {
        draw_text(&self.txt,self.pos.0+x,self.pos.1+y,font_size,color);
    }
}