use imagefmt::ColFmt;
use imagefmt::Error;
use imagefmt::ColType;
use std::f32;
use crate::dimensional::Vector;
use crate::texture::Texture;
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
    pub fn add_intensity(&mut self,intensity:f32){
        self.red=(self.red as f32 * intensity) as u8;
        self.green=(self.green as f32 * intensity) as u8;
        self.blue=(self.blue as f32 * intensity) as u8;
    }
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

    pub fn set_pixel(&mut self, point: Vector<f32>, pixel: TGAColor) -> Result<(), String> {
        if let Err(e) = self.check_boundaries(&point.round()) { return Err(e) }
        //TODO lifetime reference
        self.set_pixel_unchecked(&point, pixel);
        Ok(())
    }

    fn set_pixel_unchecked(&mut self, vec:&Vector<f32>, pixel: TGAColor) {
        let index = vec.y as usize * self.width + vec.x as usize;
        if self.z_buff[index] < vec.z {
            self.pixels[index] = pixel;
            self.z_buff[index] = vec.z;
        }
    }

    fn as_vec(&self) -> Vec<u8> {
        TGAColor::from_arr_to_arr(self.pixels.as_slice())
    }

    pub fn write_tga_file(&self, path: &str) -> Result<(), Error> {
        imagefmt::write(path, self.width, self.height
                        , ColFmt::RGBA, self.as_vec().as_slice(), ColType::Auto)
    }

//    pub fn draw_line(&mut self, start: &Vector<usize>, end: &Vector<usize>, color: &TGAColor) -> Result<(), String> {
//        if let Err(e) = self.check_boundaries(start) { return Err(e) };
//        if let Err(e) = self.check_boundaries(end) { return Err(e) }
//
//        let (miny, maxy) = if start.y > end.y { (end.y, start.y) } else { (start.y, end.y) };
//        let (minx, maxx) = if start.x > end.x { (end.x, start.x) } else { (start.x, end.x) };
//
//        let x_is_greater = maxx - minx > maxy - miny;
//
//        let (min, max) = if x_is_greater { (minx, maxx) } else { (miny, maxy) };
//
//        for t in min..max {
//            let float: f32 = (t - min) as f32 / (max - min) as f32;
//            let x: usize = ((start.x as f32) * (1.0 - float) + (end.x as f32) * float) as usize;
//            let y: usize = ((start.y as f32) * (1.0 - float) + (end.y as f32) * float) as usize;
//            self.set_pixel_unchecked(x, y, f32::MAX, color.clone());
//        }
//        Ok(())
//    }

    fn check_boundaries(&self, point: &Vector<usize>) -> Result<(), String> {
        if self.height <= point.y || self.width <= point.x {
            return Err(String::from("out of image boundaries"))
        }
        Ok(())
    }

    pub fn fill_triangle(&mut self, intensity:f32, dim_coord: &mut [Vector<isize>],text_coord:&mut [Vector<isize>]
    ,texture:&Texture) {
        if dim_coord[0].y==dim_coord[1].y && dim_coord[0].y==dim_coord[2].y{return;}

        if dim_coord[0].y>dim_coord[1].y{dim_coord.swap(0,1); text_coord.swap(0,1); }
        if dim_coord[0].y>dim_coord[2].y{dim_coord.swap(0,2); text_coord.swap(0,2); }
        if dim_coord[1].y>dim_coord[2].y{dim_coord.swap(1,2); text_coord.swap(1,2); }

        let total_height=dim_coord[2].y-dim_coord[0].y;
        for i in 0..total_height{
            let second_half=i>dim_coord[1].y-dim_coord[0].y || dim_coord[1].y==dim_coord[0].y;
            let segment_height=if second_half{dim_coord[2].y-dim_coord[1].y}
                else {dim_coord[1].y-dim_coord[0].y};
            let alpha=i as f32/(total_height as f32);
            let beta=if second_half{(i -(dim_coord[1].y-dim_coord[0].y)) as f32/segment_height as f32}
                else { i as f32/segment_height as f32 };

            let mut A=dim_coord[0].to_f32()+(dim_coord[2]-dim_coord[0])*alpha;
            let mut B=if second_half{dim_coord[1].to_f32()+(dim_coord[2]-dim_coord[1])*beta}
                else{dim_coord[0].to_f32()+(dim_coord[1]-dim_coord[0])*beta};
            let mut uvA=text_coord[0].to_f32()+(text_coord[2]-text_coord[0])*alpha;
            let mut uvB=if second_half{text_coord[1].to_f32()+(text_coord[2]-text_coord[1])*beta}
                else { text_coord[0].to_f32()+(text_coord[1]-text_coord[0])*beta };
            if A.x>B.x {mem::swap(&mut A,&mut B); mem::swap(&mut uvA,&mut uvB)}

            for j in A.x as usize..B.x as usize{
                let phi=if A.x==B.x{1.}
                    else{(j-A.x as usize) as f32/(B.x-A.x)};
                let P=A+(B-A)*phi;
                let uvP=uvA+(uvB-uvA)*phi;
                let pixel=texture.get_pixel(uvP.x as usize,uvP.y as usize);
                //println!("{:?}",uvP);
                self.set_pixel_unchecked(&P,pixel);

            }
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