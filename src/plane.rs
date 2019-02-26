use imagefmt::ColFmt;
use imagefmt::Error;
use imagefmt::ColType;
use std::f32;
use crate::dimensional::Vector;
use crate::texture::Texture;
use num::NumCast;
use std::ops::Sub;

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
    pub fn add_intensity(&mut self,intensity:f32) {
        if intensity < 0.0 {self.red=0; self.green=0; self.blue=0;}
        else {
            self.red = (self.red as f32 * intensity) as u8;
            self.green = (self.green as f32 * intensity) as u8;
            self.blue = (self.blue as f32 * intensity) as u8;
        }
    }
    pub fn to_vector(&self)->Vector<f32>{
        Vector::new(self.red as f32/127.5-1.,self.green as f32/127.5-1.,(self.blue as f32-128.)/127.*(-1.))
    }
}

#[derive(Clone)]
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
        if let Err(e) = self.check_boundaries(&point) { return Err(e) }
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

    pub fn draw_line(&mut self, start: &Vector<usize>, end: &Vector<usize>, color: &TGAColor) -> Result<(), String> {
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
            self.set_pixel_unchecked(&Vector::new(x as f32,y as f32,f32::MAX), color.clone());
        }
        Ok(())
    }

    fn check_boundaries<T:NumCast>(&self, point: &Vector<T>) -> Result<(), String>  {
        let (x,y) =(point.x.to_usize(),point.y.to_usize());
        let x =match x {
            Some(x)=>x,
            None=> return Err(String::from("out of image boundaries"))
        };
        let y=match y {
            Some(x)=>x,
            None=> return Err(String::from("out of image boundaries"))
        };
        if self.height <= y || self.width <= x {
            return Err(String::from("out of image boundaries"))
        }
        Ok(())
    }

    pub fn fill_triangle(&mut self, light:&Vector<f32>, dim_coord: &mut [Vector<f32>],text_coord:&mut [Vector<f32>]
    ,texture:&Texture,norm_coord:&mut [Vector<f32>],norm_map:&Texture) {
//        println!("drawing triangle {:?}",dim_coord);

        if dim_coord[0].y==dim_coord[1].y && dim_coord[0].y==dim_coord[2].y{return;}

        if dim_coord[0].y>dim_coord[1].y{dim_coord.swap(0,1); text_coord.swap(0,1); norm_coord.swap(0,1);}
        if dim_coord[0].y>dim_coord[2].y{dim_coord.swap(0,2); text_coord.swap(0,2); norm_coord.swap(0,2);}
        if dim_coord[1].y>dim_coord[2].y{dim_coord.swap(1,2); text_coord.swap(1,2); norm_coord.swap(1,2);}

        let (mut uvA,mut uvB)=(text_coord[0],text_coord[0]);
        let (mut A,mut B)=(dim_coord[0],dim_coord[0]);
        let (mut unA,mut unB)=(norm_coord[0],norm_coord[0]);

        let tg1_text=(text_coord[2]-text_coord[0]) / (dim_coord[2].y-dim_coord[0].y);
        let mut tg2_text=(text_coord[1]-text_coord[0])/ (dim_coord[1].y-dim_coord[0].y);
        let tg1_norm=(norm_coord[2]-norm_coord[0]) / (dim_coord[2].y-dim_coord[0].y);
        let mut tg2_norm=(norm_coord[1]-norm_coord[0])/ (dim_coord[1].y-dim_coord[0].y);
        let tg_last=(dim_coord[2]-dim_coord[0])/(dim_coord[2].y-dim_coord[0].y);
        let mut tg_middle=(dim_coord[1]-dim_coord[0])/(dim_coord[1].y-dim_coord[0].y);

        for dy in dim_coord[0].y as usize..dim_coord[2].y as usize{
            if dim_coord[1].y as usize==dy{
                uvB=text_coord[1];
                tg2_text=(text_coord[2]-text_coord[1])/ (dim_coord[2].y-dim_coord[1].y);
                unB=norm_coord[1];
                tg2_norm=(norm_coord[2]-norm_coord[1])/ (dim_coord[2].y-dim_coord[1].y);
                B=dim_coord[1];
                tg_middle=(dim_coord[2]-dim_coord[1])/(dim_coord[2].y-dim_coord[1].y);
            }

            let (start,end)=if A.x<B.x{(A,B)}
                else{ (B,A)};

            for dx in start.x as usize..end.x as usize{
                let mut phi=(dx as f32-A.x)/(B.x-A.x);
                if phi>1.{phi=1.};
                if phi<0.{phi=0.};
                let P=A+(B-A)*phi;
                let uvP=uvA+(uvB-uvA)*phi;
                let unP=unA+(unB-unA)*phi;
//                println!("x {}, y{}",uvP.x as usize,uvP.y);
                let mut pixel=texture.get_pixel(uvP.x as usize,uvP.y as usize);
                let norm_pixel=norm_map.get_pixel(unP.x as usize,unP.y as usize).to_vector().normalize();
                let intensity=norm_pixel.scalar_prod(light)*1.5;
//                println!("intensity {}",intensity);
                pixel.add_intensity(intensity);
                self.set_pixel(P,pixel);
            }
            unA=unA+tg1_norm;
            unB=unB+tg2_norm;
            uvA=uvA+tg1_text;
            uvB=uvB+tg2_text;
            A=A+tg_last;
            B=B+tg_middle;
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