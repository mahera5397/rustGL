use imagefmt::ColFmt;
use imagefmt::Error;
use imagefmt::ColType;

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
        if final_coord.len()!=3 {return;}
        if final_coord[0].x==final_coord[1].x&&final_coord[1].x==final_coord[2].x
        || final_coord[0].y==final_coord[1].y&&final_coord[1].y==final_coord[2].y{ return; }

        println!("filling triangle {:?}",&final_coord);
        let mut two_min_y = false;
        let mut y_apex = None;
        let mut bottom_point = None;
        for dot in final_coord.iter() {
            if y_apex == None {
                y_apex = Some(dot);
                bottom_point = Some(dot);
                continue
            }
            if dot.y <= y_apex.unwrap().y {
                if dot.y == y_apex.unwrap().y { two_min_y = true; } else {
                    y_apex = Some(dot);
                    two_min_y = false;
                }
            }
            if dot.y > bottom_point.unwrap().y {
                bottom_point = Some(dot);
            }
        }
        //flip points if two apexes
        let (apex_point,bottom_dot)=if two_min_y{
            (bottom_point.unwrap(),y_apex.unwrap().y)
        }
        else {
            (y_apex.unwrap(),bottom_point.unwrap().y)
        };
        let mut left_dot = None;
        for dot in final_coord.iter() {
            if dot == apex_point { continue }
            if left_dot == None {
                left_dot = Some(dot);
                continue
            }
            if dot.x < left_dot.unwrap().x {
                left_dot = Some(dot);
            }
        }
        let left_dot = left_dot.unwrap();
        let mut right_dot = None;
        for dot in final_coord.iter() {
            if dot != apex_point && dot != left_dot { right_dot = Some(dot) }
        };
        //TODO
        if right_dot==None{return;}
        let right_dot = right_dot.unwrap();
        let (left_dot,right_dot)=if two_min_y{
            (right_dot,left_dot)
        }else {
            (left_dot,right_dot)
        };
        let bottom_dot_right = Point::new(apex_point.x, right_dot.y);
        let mut tg_right = line_length(&bottom_dot_right, right_dot) / line_length(apex_point, &bottom_dot_right);
        let bottom_dot_left = Point::new(apex_point.x, left_dot.y);
        let mut tg_left = line_length(&bottom_dot_left, left_dot) / line_length(apex_point, &bottom_dot_left);
        if tg_right.is_infinite(){tg_right=0.0}
        if tg_left.is_infinite(){tg_left=0.0}
        let mut counter = 0;
        for dot in final_coord.iter() {
            if dot.x < apex_point.x { counter += 1; }
        }
        let clousure;
        match counter {
            0 => clousure = Some(TGAImage::both_right as fn(&usize, &usize, &usize) -> (usize, usize)),
            1 => clousure = Some(TGAImage::one_left_one_right as fn(&usize, &usize, &usize) -> (usize, usize)),
            _ => {
                clousure = Some(TGAImage::both_left as fn(&usize, &usize, &usize) -> (usize, usize));
                let swap = tg_right;
                tg_right = tg_left;
                tg_left = swap;
            }
        }

        let mut midle_point=None;
        for coord in final_coord.iter(){
            if coord!=apex_point && coord!=bottom_point.unwrap(){
                midle_point=Some(coord.clone());
                break
            }
        };
        let midle_point=midle_point.unwrap();

        //flip direction if two apexes
        let (dy_min,dy_max)=if two_min_y{
            (0,apex_point.y + 1 - bottom_dot  )
        }else {
            (0 ,midle_point.y + 1 - apex_point.y)
        };
        for dy in dy_min..dy_max {
            let right = (dy as f64 * tg_right) as usize;
            let left = (dy as f64 * tg_left) as usize;
            let (left,right)=if two_min_y{
                (right,left)
            }else {
                (left,right)
            };
            let (start, length) = clousure.unwrap()(&left, &right, &apex_point.x);
            let y_coord=if two_min_y{
                apex_point.y-dy
            }else{
                dy + apex_point.y
            };
            for x_coord in start..start + length {
                self.set_pixel_unchecked(x_coord, y_coord, &color);
            }
        }
        if !two_min_y {
            let right = (dy_max as f64 * tg_right) as usize;
            let left = (dy_max as f64 * tg_left) as usize;
            let (start,length)=clousure.unwrap()(&left,&right,&apex_point.x);
            let new_point =match counter {
                0=>Point::new(start+length,midle_point.y),
                1=>{
                    let point=Point::new(start+length,midle_point.y);
                    if  point==midle_point{ Point::new(start,midle_point.y)}
                    else{point} },
                _=>Point::new(start,midle_point.y)
            };
            let mut new_point=Point::new(start+length,midle_point.y);
            if  new_point==midle_point{
                new_point=Point::new(start,midle_point.y);
            }
            let bottom_point=bottom_point.unwrap().clone();
            let final_coord=[midle_point,bottom_point,new_point];
            println!("before recursive {:?}",&final_coord);
            self.fill_triangle(color,&final_coord);
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