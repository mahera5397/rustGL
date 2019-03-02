use imagefmt::ColFmt;
use imagefmt::Error;
use imagefmt::ColType;
use std::f32;
use crate::dimensional::Vector;
use crate::texture::Texture;
use num::NumCast;
use std::sync::Arc;
use std::sync::Mutex;
use crate::colors::TGAColor;


pub struct TGAImage{
    pub height:usize,
    pub width:usize,
    pixels:Mutex<Vec<TGAColor>>,
    z_buff:Mutex<Vec<f32>>
}


impl TGAImage {
    pub fn new(height: usize, width: usize) -> TGAImage {
        let pixels =Mutex::new( vec![TGAColor::new(255, 255, 255, 255); height * width]);
        let z_buff = Mutex::new(vec![f32::MIN; height * width]);
        TGAImage { height, width, pixels, z_buff }
    }

    pub fn set_pixel(&self, point: Vector<f32>, pixel: TGAColor) -> Result<(), String> {
        if let Err(e) = self.check_boundaries(&point) { return Err(e) }
        //TODO lifetime reference
        self.set_pixel_unchecked(&point, pixel);
        Ok(())
    }

    fn set_pixel_unchecked(&self, vec:&Vector<f32>, pixel: TGAColor) {
        let index = vec.y as usize * self.width + vec.x as usize;
        let mut z_buff=self.z_buff.lock().unwrap();
        let mut pixels=self.pixels.lock().unwrap();
        if z_buff[index] < vec.z {
            pixels[index] = pixel;
            z_buff[index] = vec.z;
        }
    }

    fn set_pixels(&self,tulp:Vec<(Vector<f32>, TGAColor)>){
        let mut z_buff=self.z_buff.lock().unwrap();
        let mut pixels=self.pixels.lock().unwrap();
        for (point,pixel) in tulp{
            let index = point.y as usize * self.width + point.x as usize;
            if index>pixels.len(){continue}
            if z_buff[index] < point.z {
                pixels[index] = pixel;
                z_buff[index] = point.z;
            }
        }
    }

    fn as_vec(&self) -> Vec<u8> {
        TGAColor::from_arr_to_arr(self.pixels.lock().unwrap().as_slice())
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

    pub fn fill_triangle(&self, light:&Vector<f32>, coords: &mut [Vector<f32>],text_coords:&mut [Vector<f32>]
    ,texture:&Option<Arc<Texture>>,norm_coords:&mut [Vector<f32>],norm_map:&Option<Arc<Texture>>,sp_map:&Option<Arc<Texture>>) {

        if coords[0].y==coords[1].y && coords[0].y==coords[2].y{return;}

        if coords[0].y>coords[1].y{coords.swap(0,1); text_coords.swap(0,1); norm_coords.swap(0,1);}
        if coords[0].y>coords[2].y{coords.swap(0,2); text_coords.swap(0,2); norm_coords.swap(0,2);}
        if coords[1].y>coords[2].y{coords.swap(1,2); text_coords.swap(1,2); norm_coords.swap(1,2);}

        let mut pixels=Vec::new();

        let (mut uvA,mut uvB)=(text_coords[0],text_coords[0]);
        let (mut A,mut B)=(coords[0],coords[0]);
        let (mut unA,mut unB)=(norm_coords[0],norm_coords[0]);

        let tg1_text=(text_coords[2]-text_coords[0]) / (coords[2].y-coords[0].y);
        let mut tg2_text=(text_coords[1]-text_coords[0])/ (coords[1].y-coords[0].y);
        let tg1_norm=(norm_coords[2]-norm_coords[0]) / (coords[2].y-coords[0].y);
        let mut tg2_norm=(norm_coords[1]-norm_coords[0])/ (coords[1].y-coords[0].y);
        let tg_last=(coords[2]-coords[0])/(coords[2].y-coords[0].y);
        let mut tg_middle=(coords[1]-coords[0])/(coords[1].y-coords[0].y);

        for dy in coords[0].y as usize..coords[2].y as usize{
            if coords[1].y as usize==dy{
                uvB=text_coords[1];
                tg2_text=(text_coords[2]-text_coords[1])/ (coords[2].y-coords[1].y);
                unB=norm_coords[1];
                tg2_norm=(norm_coords[2]-norm_coords[1])/ (coords[2].y-coords[1].y);
                B=coords[1];
                tg_middle=(coords[2]-coords[1])/(coords[2].y-coords[1].y);
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

                let mut pixel= match texture {
                    Some(val)=>val.get_pixel(uvP.x as usize,uvP.y as usize),
                    None=> TGAColor::new(0,0,0,255),
                };
                let mut intensity=0.;
                if let Some(val)=norm_map{
                    let norm_pixel=val.get_pixel(unP.x as usize,unP.y as usize).to_vector().normalize();
                    intensity+=norm_pixel.scalar_prod(light);
                };
                if let Some(val)=sp_map{
                    intensity+=val.get_pixel_grey(unP.x as usize,unP.y as usize)*0.6;
                };
//                let intensity=norm_pixel.scalar_prod(light)*1.5;
                pixel.add_intensity(intensity);
                pixels.push((P,pixel));
            }
            unA=unA+tg1_norm;
            unB=unB+tg2_norm;
            uvA=uvA+tg1_text;
            uvB=uvB+tg2_text;
            A=A+tg_last;
            B=B+tg_middle;
        }
        self.set_pixels(pixels);
    }

    pub fn flip_vertically(&self) {
        let mut top_half = Vec::new();
        let mut pixels=self.pixels.lock().unwrap();
        for y in 0..self.height {
            top_half.append(&mut pixels[self.width * (self.height - y - 1)..self.width * (self.height - y)].to_vec())
        }
        pixels.swap_with_slice(&mut top_half);
    }
}