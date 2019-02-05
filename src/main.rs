extern crate imagefmt;
use imagefmt::{ColFmt, ColType};
use std::result::Result::*;
use imagefmt::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;

#[derive(Copy, Clone)]
struct TGAColor{
    red:u8,
    green:u8,
    blue:u8,
    alpha:u8
}
impl TGAColor{
    fn new(red:u8,green:u8,blue:u8,alpha:u8)->TGAColor{
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

struct TGAImage{
    height:usize,
    width:usize,
    pixels:Vec<TGAColor>
}

impl TGAImage{
    fn new(height:usize, width:usize)->TGAImage{
        let mut pixels=Vec::new();
        for _ in 0..height*width{
            pixels.push(TGAColor::new(255,255,255,255));
        }
        TGAImage{height,width, pixels}
    }
    fn set_pixel(&mut self,x:usize,y:usize,pixel:& TGAColor)->Result<(),String>{
        if let Err(e)=self.check_boundaries(x,y){
            return Err(e).unwrap()
        }
        self.pixels[y*self.width+x]=*pixel;
        Ok(())
    }
    fn set_pixel_unchecked(&mut self,x:usize,y:usize,pixel:& TGAColor){
        self.pixels[y*self.width+x]=*pixel;
    }
    fn as_vec(&self)->Vec<u8>{
        TGAColor::from_arr_to_arr(self.pixels.as_slice())
    }
    fn write_tga_file(&self,path:&str)->Result<(),Error>{
        imagefmt::write(path,self.width,self.height
                        ,ColFmt::RGBA,self.as_vec().as_slice(),ColType::Auto)
    }
    fn draw_line(&mut self,mut x0:usize,mut y0:usize,mut x1:usize,mut y1:usize,color:& TGAColor)->Result<(),String>{
        if let Err(e)=self.check_boundaries(x0,y0){
            return Err(e)
        }
        if let Err(e)=self.check_boundaries(x1,y1){
            return Err(e)
        }

        let (miny,maxy)=if y0>y1 {(y1,y0)} else { (y0,y1) };
        let (minx,maxx)=if x0>x1{(x1,x0)} else { (x0,x1) };
        let x_is_greater=maxx-minx>maxy-miny;

        let (min,max)=if x_is_greater{(minx,maxx)}else { (miny,maxy) };

        for t in min..max{
            let float:f32=(t-min)as f32/(max-min) as f32;
            let x:usize=((x0 as f32)*(1.0-float)+(x1 as f32)*float) as usize;
            let y:usize=((y0 as f32)*(1.0-float)+(y1 as f32)*float) as usize;
            self.set_pixel_unchecked(x,y,color);
        }
        Ok(())
    }
    fn check_boundaries(&self,x:usize,y:usize)->Result<(),String>{
        if self.height<=y ||self.width<=x{
            return Err(String::from("out of image boundaries"))
        }
        Ok(())
    }
}


const FILE_PATH:&str="image.tga";

fn main() {
let WHITE:TGAColor=TGAColor::new(255,255,255,255);
let RED:TGAColor=TGAColor::new(255,0,0,255);

    let mut tga_image=TGAImage::new(10000,10000);
    //tga_image.set_pixel(52,41,&RED);
    //tga_image.draw_line(0,0,50,50,TGAColor::new(0,0,0,255));
    //tga_image.draw_line(13, 20, 80, 40, &RED);
    //tga_image.draw_line(20, 13, 40, 80, &RED);


    let (points,triangles)=read_file();
    for triangle in triangles{
        let mut triangle_real_coord:[(f32,f32);3]=[(0.0,0.0);3];
        for (index,point) in triangle.iter().enumerate(){
            let point:[f32;3]=*points.get(*point-1).unwrap();
            triangle_real_coord[index]=((point[0]+1.0)/2.0,(point[1]+1.0)/2.0);
        }
        let mut final_coord:[(f32,f32);3]=[(0.0,0.0);3];
        for  (index,real_coorod) in triangle_real_coord.iter().enumerate(){
            final_coord[index]=(real_coorod.0*tga_image.width as f32,real_coorod.1*tga_image.height as f32);
        }
        //println!("draw line for {}, {}", final_coord[0].0,final_coord[0].1);
        tga_image.draw_line(final_coord[0].0 as usize, final_coord[0].1 as usize
                            , final_coord[1].0 as usize, final_coord[1].1 as usize, &RED);
        tga_image.draw_line(final_coord[1].0 as usize, final_coord[1].1 as usize
                            , final_coord[2].0 as usize, final_coord[2].1 as usize, &RED);
        tga_image.draw_line(final_coord[2].0 as usize, final_coord[2].1 as usize
                            , final_coord[0].0 as usize, final_coord[0].1 as usize, &RED);
    }
    tga_image.write_tga_file(FILE_PATH);
}

fn read_file()->(Vec<[f32;3]>,Vec<[usize;3]>)
    //->Result<Vec<String>,String>
{
    let res=File::open("african_head.obj").unwrap();
    let mut reader=BufReader::new(res);
    let mut points=Vec::new();
    let mut triangles=Vec::new();

    for line in reader.lines(){
        match line {
            Ok(line)=> {
                if line.starts_with("v "){
                    let line=&line[2..];
                    let mut point:[f32;3]=[0.0;3];
                    for (index,coord) in line.split_whitespace().enumerate(){
                        point[index]=coord.parse::<f32>().unwrap();
                    }
                    points.push(point);
                }
                if line.starts_with("f "){
                    let mut triangle:[usize;3]=[0;3];
                    let line=&line[2..];
                    for (index,coords) in line.split_whitespace().enumerate(){
                        let index_of_slash=coords.find('/').unwrap();
                        let point=&coords[..index_of_slash];
                        triangle[index]=point.parse::<usize>().unwrap();
                    }
                    triangles.push(triangle);                }
            },
            Err(e) =>()
        }
    }
    (points,triangles)
}
