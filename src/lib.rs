use imagefmt::ColFmt;
use imagefmt::Error;
use imagefmt::ColType;
use std::cmp::Ordering;

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

pub fn line_length(start:& Point,end:& Point)->f64{
    ((start.x as f64 -end.x as f64)*
        (start.x as f64 -end.x as f64)+(start.y as f64 -end.y as f64 )*
        (start.y as f64 -end.y as f64 ) ).sqrt()
}

pub struct TGAImage{
    pub height:usize,
    pub width:usize,
    pixels:Vec<TGAColor>
}


impl TGAImage{
    pub fn new(height:usize, width:usize)->TGAImage{
        let mut pixels=Vec::new();
        for _ in 0..height*width{
            pixels.push(TGAColor::new(255,255,255,255));
        }
        TGAImage{height,width, pixels}
    }

    pub fn set_pixel(&mut self,point:& Point,pixel:& TGAColor)->Result<(),String>{
        if let Err(e)=self.check_boundaries(point){ return Err(e) }
        //TODO lifetime reference
        self.pixels[point.y*self.width+point.x]=*pixel;
        Ok(())
    }

    fn set_pixel_unchecked(&mut self,x:usize,y:usize,pixel:& TGAColor){
        self.pixels[y*self.width+x]=*pixel;
    }
    fn as_vec(&self)->Vec<u8>{
        TGAColor::from_arr_to_arr(self.pixels.as_slice())
    }

    pub fn write_tga_file(&self,path:&str)->Result<(),Error>{
        imagefmt::write(path,self.width,self.height
                        ,ColFmt::RGBA,self.as_vec().as_slice(),ColType::Auto)
    }

    pub fn draw_line(&mut self,start:& Point,end:& Point,color:& TGAColor)->Result<(),String>{
        if let Err(e)=self.check_boundaries(start){ return Err(e) };
        if let Err(e)=self.check_boundaries(end){ return Err(e) }

        let (miny, maxy) = if start.y > end.y { (end.y, start.y) } else { (start.y, end.y) };
        let (minx, maxx) = if start.x > end.x { (end.x, start.x) } else { (start.x, end.x) };


        let x_is_greater = maxx - minx > maxy - miny;

        let (min, max) = if x_is_greater { (minx, maxx) } else { (miny, maxy) };

        for t in min..max {
            let float: f32 = (t - min) as f32 / (max - min) as f32;
            let x: usize = ((start.x as f32) * (1.0 - float) + (end.x as f32) * float) as usize;
            let y: usize = ((start.y as f32) * (1.0 - float) + (end.y as f32) * float) as usize;
            self.set_pixel_unchecked(x, y, color);
        }
        Ok(())
    }

    fn check_boundaries(&self,point:& Point)->Result<(),String>{
        if self.height<=point.y ||self.width<=point.x{
            return Err(String::from("out of image boundaries"))
        }
        Ok(())
    }

    pub fn fill_triangle(&mut self, color: &TGAColor, final_coord: &[Point]) {
        let mut first_dot = None;
        let mut last_dot = None;
        for point in final_coord{
            if first_dot == None && last_dot==None {
                first_dot = Some(point);
                last_dot = Some(point);
                continue
            }
            if first_dot.unwrap().y>point.y{first_dot=Some(point)}
            if last_dot.unwrap().y<point.y{last_dot=Some(point)}
        }
        let first_dot=first_dot.unwrap();
        let last_dot=last_dot.unwrap();
        let mut middle_dot = None;
        for dot in final_coord.iter() {
            if dot != first_dot && dot != last_dot { middle_dot = Some(dot) }
        };
        //if no middle dot then triangle cant exist
        if middle_dot==None{return;}
        let middle_dot=middle_dot.unwrap();

        let mut y_axis_middle=Point::new(first_dot.x,middle_dot.y);
        let y_axis_last=Point::new(first_dot.x,last_dot.y);

        let mut tg_migle = line_length(&y_axis_middle, middle_dot) / line_length(first_dot, &y_axis_middle);
        if tg_migle.is_infinite(){tg_migle=0.0}
        let mut tg_last = line_length(&y_axis_last, last_dot) / line_length(first_dot, &y_axis_last);
        if tg_last.is_infinite(){tg_last=0.0}
        if middle_dot.x<first_dot.x{tg_migle=tg_migle*(-1.0)}
        if last_dot.x<first_dot.x{tg_last=tg_last*(-1.0)}

        let (mut on_mid_border,mut on_last_border)=
            (first_dot.x as f64-tg_migle,first_dot.x as f64-tg_last);
        for dy in first_dot.y..middle_dot.y+1 {
            on_mid_border = on_mid_border + tg_migle;
            on_last_border = on_last_border + tg_last;

            let (start, end) = if on_mid_border > on_last_border { (on_last_border as usize, on_mid_border as usize) } else { (on_mid_border as usize, on_last_border as usize) };
            for x_coord in start..end+1 {
                self.set_pixel_unchecked(x_coord, dy, &color);
            }
        }
        y_axis_middle=Point::new(last_dot.x,middle_dot.y);
        tg_migle = line_length(&y_axis_middle, middle_dot) / line_length(last_dot, &y_axis_middle);
        if tg_migle.is_infinite(){tg_migle=0.0}
        if middle_dot.x>last_dot.x{tg_migle=tg_migle*(-1.0)}
        for dy in middle_dot.y+1..last_dot.y{
            on_mid_border = on_mid_border + tg_migle;
            on_last_border = on_last_border + tg_last;
            let (start, end) = if on_mid_border > on_last_border { (on_last_border as usize, on_mid_border as usize) } else { (on_mid_border  as usize, on_last_border as usize) };
            for x_coord in start..end+1{
                self.set_pixel_unchecked(x_coord, dy, &color);
            }
        }


    }

    fn both_right(left:&usize,right:&usize,apex_x:&usize)->(usize,usize){
        let (left,right)=if left>right{(right,left)}
        else { (left,right) };
        let length=right-left;
        let start_point=apex_x+left;
        (start_point,length)
    }

    fn both_left(left:&usize,right:&usize,apex_x:&usize)->(usize,usize){
        let (left,right)=if left>right{(right,left)}
        else { (left,right) };
        let length=right-left;
        let start_point=apex_x-right;
        (start_point,length)
    }

    fn one_left_one_right(left:&usize,right:&usize,apex_x:&usize)->(usize,usize){
        let length=left+right;
        let start_point=apex_x-left;
        (start_point,length)
    }
}


#[derive(PartialEq,Debug,Clone)]
pub struct Point{
    pub x:usize,
    pub y:usize,
}
impl Point{
    pub fn new(x:usize,y:usize)->Point{
        Point{x,y}
    }
}