use imagefmt::ColFmt;
use imagefmt::Error;
use imagefmt::ColType;
use std::f32;
use crate::dimensional::Vector;
use core::mem;

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
    fn to_array(&self)->[u8;4]{
        [self.red,self.green,self.blue,self.alpha]
    }
    fn from_arr_to_arr(array:&[TGAColor])->Vec<u8>{
        let mut respond=Vec::new();
        for tga in array.iter(){
            for val in &tga.to_array(){
                respond.push(*val);
            }
        }
        respond
    }
}

pub fn line_length(start:& Point,end:& Point)->f32{
    ((start.x as f32 -end.x as f32)*
        (start.x as f32 -end.x as f32)+(start.y as f32 -end.y as f32 )*
        (start.y as f32 -end.y as f32 ) ).sqrt()
}

pub struct TGAImage{
    pub height:usize,
    pub width:usize,
    pixels:Vec<TGAColor>,
    z_buff:Vec<f32>
}


impl TGAImage {
    pub fn new(height: usize, width: usize) -> TGAImage {
        let pixels = vec![TGAColor::new(255, 255, 255, 255); height * width];
        let z_buff = vec![f32::MIN; height * width];
        TGAImage { height, width, pixels, z_buff }
    }

    pub fn set_pixel(&mut self, point: &Point, pixel: TGAColor) -> Result<(), String> {
        if let Err(e) = self.check_boundaries(point) { return Err(e) }
        //TODO lifetime reference
        self.set_pixel_unchecked(point.x, point.y, point.z as f32, pixel);
        Ok(())
    }

    fn set_pixel_unchecked(&mut self, x: usize, y: usize, z: f32, pixel: TGAColor) {
        let index = y * self.width + x;
        if self.z_buff[index] < z {
            if self.z_buff[index]>f32::MIN{
                if x>250 && y<250{

                    println!("x {}, y {}",x,y);
                    println!("z {} , z in buffer {}",z,self.z_buff[index]);
                }
            }
            self.pixels[index] = pixel;
            self.z_buff[index] = z;
        }
        else if self.z_buff[index]>f32::MIN{
            if x>250 && y<250 && self.z_buff[index]-z<10.0{

                println!("bounced x {}, y {}",x,y);
                println!("bounced z {} , z in buffer {}",z,self.z_buff[index]);
            }
        }
    }

    fn as_vec(&self) -> Vec<u8> {
        TGAColor::from_arr_to_arr(self.pixels.as_slice())
    }

    pub fn write_tga_file(&self, path: &str) -> Result<(), Error> {
        imagefmt::write(path, self.width, self.height
                        , ColFmt::RGBA, self.as_vec().as_slice(), ColType::Auto)
    }

    pub fn draw_line(&mut self, start: &Point, end: &Point, color: &TGAColor) -> Result<(), String> {
        if let Err(e) = self.check_boundaries(start) { return Err(e) };
        if let Err(e) = self.check_boundaries(end) { return Err(e) }

        let (miny, maxy) = if start.y > end.y { (end.y, start.y) } else { (start.y, end.y) };
        let (minx, maxx) = if start.x > end.x { (end.x, start.x) } else { (start.x, end.x) };


        let x_is_greater = maxx - minx > maxy - miny;

        let (min, max) = if x_is_greater { (minx, maxx) } else { (miny, maxy) };

        for t in min..max {
            let float: f32 = (t - min) as f32 / (max - min) as f32;
            let x: usize = ((start.x as f32) * (1.0 - float) + (end.x as f32) * float) as usize;
            let y: usize = ((start.y as f32) * (1.0 - float) + (end.y as f32) * float) as usize;
            self.set_pixel_unchecked(x, y, f32::MAX, color.clone());
        }
        Ok(())
    }

    fn check_boundaries(&self, point: &Point) -> Result<(), String> {
        if self.height <= point.y || self.width <= point.x {
            return Err(String::from("out of image boundaries"))
        }
        Ok(())
    }

    pub fn fill_triangle(&mut self, color: TGAColor, final_coord: &mut [Point]) {
        final_coord.sort_by(|a, b| a.y.partial_cmp(&b.y).unwrap());
        let first_dot = &final_coord[0];
        let last_dot = &final_coord[2];
        let middle_dot = &final_coord[1];

        let tg_last = first_dot.to_vector(last_dot);
        let mut tg_middle = first_dot.to_vector(middle_dot);

        let (mut tg_middle_x, mut tg_last_x) = (tg_middle.k_of_axis(0, 1), tg_last.k_of_axis(0, 1));
        let (mut tg_middle_z, mut tg_last_z) = (tg_middle.k_of_axis(2, 1), tg_last.k_of_axis(2, 1));


        let (mut on_last_border_dx, mut on_last_border_dz) = (first_dot.x as f32, first_dot.z as f32);
        let (mut on_mid_border_dx, mut on_mid_border_dz) = (first_dot.x as f32, first_dot.z as f32);

        for dy in first_dot.y..last_dot.y {
            if dy == middle_dot.y {
                on_mid_border_dx = middle_dot.x as f32;
                on_mid_border_dz = middle_dot.z as f32;

                tg_middle = middle_dot.to_vector(last_dot);
                tg_middle_x = tg_middle.k_of_axis(0, 1);
                tg_middle_z = tg_middle.k_of_axis(2, 1);
            }

            let (start, end,mut dz_start,mut tg_x_z) =
                if on_mid_border_dx > on_last_border_dx {
                    (on_last_border_dx as usize, on_mid_border_dx as usize
                     ,on_last_border_dz, on_mid_border_dz-on_last_border_dz) }
                else {
                    (on_mid_border_dx as usize, on_last_border_dx as usize
                     ,on_mid_border_dz,on_last_border_dz-on_mid_border_dz) };

            if start != end {
                tg_x_z=tg_x_z/(end-start) as f32;
                for x_coord in start..end {
                    self.set_pixel_unchecked(x_coord, dy, dz_start, color);
                    dz_start += tg_x_z;
                    if tg_x_z==0.0{
                        println!("tg 0");
                    }
                }
            }

            on_mid_border_dx += tg_middle_x;
            on_last_border_dx += tg_last_x;

            on_mid_border_dz += tg_middle_z;
            on_last_border_dz += tg_last_z;
        }
    }

    pub fn flip_vertically(&mut self) {
        let mut top_half = Vec::new();
        for y in 0..self.height {
            top_half.append(&mut self.pixels[self.width * (self.height - y - 1)..self.width * (self.height - y)].to_vec())
        }
        self.pixels.swap_with_slice(&mut top_half);
    }
}


#[derive(PartialEq,Debug,Clone)]
pub struct Point{
    pub x:usize,
    pub y:usize,
    pub z:usize
}
impl Point{
    pub fn new(x:usize,y:usize,z:usize)->Point{
        Point{x,y,z}
    }
    pub fn to_vector(&self,end_of_vector:&Point) ->Vector{
        Vector(end_of_vector.x as f32-self.x as f32,
               end_of_vector.y as f32-self.y as f32,
               end_of_vector.z as f32-self.z as f32)
    }
}