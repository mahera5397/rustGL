use crate::plane::TGAColor;

pub struct Texture{
    pub height:usize,
    pub width:usize,
    arr:Vec<u8>,
}

//unsafe impl Send for Texture{
//}

impl Texture{
    pub fn new(height:usize,width:usize,arr:Vec<u8>)->Texture{
        Texture{height,width,arr }
    }
    pub fn get_pixel(&self,x:usize,y:usize)->TGAColor{
        let index=y*self.width+x;
//        if index>self.width*self.height-1{ return  TGAColor::new(0,0,0,0)}
        let vec=self.arr.iter()
            .skip(index*4)
            .take(4)
            .collect::<Vec<&u8>>();
        TGAColor::new(*vec[0],*vec[1],*vec[2],*vec[3])
    }
}