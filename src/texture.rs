use crate::plane::TGAColor;
use crate::plane::Point;
use crate::dimensional::Vector;
use core::mem;

pub struct Texture{
    pub height:usize,
    pub width:usize,
    arr:Vec<u8>,
}

impl Texture{
    pub fn new(height:usize,width:usize,arr:Vec<u8>)->Texture{
        Texture{height,width,arr }
    }
    pub fn get_pixel(&self,x:usize,y:usize)->TGAColor{
        let index=y*self.width+x;
        let vec=self.arr.iter()
            .skip(index*4)
            .take(4)
            .collect::<Vec<&u8>>();
        TGAColor::new(*vec[0],*vec[1],*vec[2],*vec[3])
    }
}

pub struct Triangle<'a>{
    points:&'a [Point],
    texture:&'a Texture,
    vectors:Vec<Vector>,
    height:usize,
    width:usize,
    min_x:usize,
    min_y:usize,
}
impl <'a> Triangle<'a>{
    pub fn new(points:&'a mut [Point],texture:&'a Texture)-> Triangle<'a>{
        points.sort_by(|a,b|a.x.partial_cmp(&b.x).unwrap());
        let (min_x,width)=(points[0].x,points[2].x-points[0].x);

        points.sort_by(|a,b|a.y.partial_cmp(&b.y).unwrap());
        let (min_y,height)=(points[0].y,points[2].y-points[0].y);
        let tg01=points[0].to_vector(&points[1]);
        let tg02=points[0].to_vector(&points[2]);
        let tg12=points[1].to_vector(&points[2]);
        let vectors=vec![tg02,tg01,tg12];

        Triangle{points,texture,vectors,height,width,min_y,min_x}
    }

    pub fn get_color(&self,mut kx:f32,ky:f32)->TGAColor{
        let y=self.min_y+(ky*self.height as f32) as usize;
        let mut x_on_last=(y-self.min_y) as f32 * self.vectors[0].k_of_axis(0,1)+self.points[0].x as f32;
        let mut x_on_middle= if y>self.points[1].y{
            (y-self.points[1].y) as f32 * self.vectors[2].k_of_axis(0,1)+self.points[1].x as f32
        }else {
            (y-self.min_y) as f32 * self.vectors[1].k_of_axis(0,1)+self.points[0].x as f32
        };
        if x_on_last<x_on_middle{ mem::swap(&mut x_on_middle,&mut x_on_last); }
        let x_length=x_on_last-x_on_middle;
        let x_offset=x_length*kx;
        let final_x=(x_on_middle+x_offset) as usize;

        self.texture.get_pixel(final_x,y)
    }
}