use crate::dimensional::Vector;

pub enum Colors{
    RGBA,
    Gray,
}

#[derive(Copy, Clone)]
pub struct TGAColor{
    red:u8,
    green:u8,
    blue:u8,
    alpha:u8
}

impl TGAColor{
    pub fn new(red:u8,green:u8,blue:u8,alpha:u8)->TGAColor{
        TGAColor{red,green,blue,alpha}
    }

    pub fn from_arr_to_arr(array:&[TGAColor])->Vec<u8>{
        let mut respond=Vec::new();
        for tga in array.iter(){
            for val in &tga.to_array(){
                respond.push(*val);
            }
        }
        respond
    }
    fn to_array(&self)->[u8;4]{
        [self.red,self.green,self.blue,self.alpha]
    }

    pub fn add_intensity(&mut self,intensity:f32) {
        if intensity < 0.0 {self.red=0; self.green=0; self.blue=0;}
        else {
            self.red = (self.red as f32 * intensity) as u8;
            self.green = (self.green as f32 * intensity) as u8;
            self.blue = (self.blue as f32 * intensity) as u8;
        }
    }
    pub fn to_vector(&self) ->Vector<f32>{
        Vector::new(self.red as f32/127.5-1.,self.green as f32/127.5-1.,(self.blue as f32-128.)/127.*(-1.))
    }
}