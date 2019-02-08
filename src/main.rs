extern crate imagefmt;
use std::result::Result::*;
use imagefmt::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use simpleOpenGL::Point;
use simpleOpenGL::TGAColor;
use simpleOpenGL::TGAImage;



const FILE_PATH:&str="image.tga";
const LIGHT_DIR:Vector=Vector(0.0,0.0,1.0);
fn main() {
    let WHITE:TGAColor=TGAColor::new(255,255,255,255);
    let RED:TGAColor=TGAColor::new(255,0,0,255);
    let GREEN:TGAColor=TGAColor::new(0,255,0,255);


    let mut tga_image=TGAImage::new(10000,10000);


    let (points,triangles)=read_file();
    for triangle in triangles{
        let mut triangle_real_coord:[(f32,f32,f32);3]=[(0.0,0.0,0.0);3];
        for (index,point) in triangle.iter().enumerate(){
            let point:[f32;3]=*points.get(*point-1).unwrap();
            triangle_real_coord[index]=((point[0]+1.0)/2.0,(point[1]+1.0)/2.0,point[2]);
        }
        let mut final_coord=Vec::new();
        for  (index,real_coorod) in triangle_real_coord.iter().enumerate(){
            final_coord.push(D_point::new(real_coorod.0,
            real_coorod.1,real_coorod.2));
        }
        let vec0=final_coord[0].to_vector(&final_coord[1]);
        let vec1=final_coord[0].to_vector(&final_coord[2]);

        let triangle_normal=vec0.vector_prod(&vec1)
            .normalize();
        println!("normal {:?}",triangle_normal);
        let intensity=triangle_normal.scalar_prod(&LIGHT_DIR);
        println!("intensity {}",intensity);
        if intensity>0.0{
            let color=TGAColor::new((255.0*intensity) as u8,(255.0*intensity) as u8,(255.0*intensity) as u8,255);
            tga_image.fill_triangle(&color,final_coord
                .iter()
                .map(|element|element.to_point(10000,10000))
                .collect::<Vec<Point>>()
                .as_slice());
        }


//        tga_image.draw_line(&final_coord[0], &final_coord[1], &RED);
//        tga_image.draw_line(&final_coord[1], &final_coord[2], &RED);
//        tga_image.draw_line(&final_coord[2], &final_coord[0], &RED);

    }

//    let t0=[Point::new(10,70),Point::new(50,160),Point::new(70,80)];
//    let t1=[Point::new(180,50),Point::new(150,1),Point::new(70,180)];
//    let t2=[Point::new(180,150),Point::new(120,160),Point::new(130,180)];
//
//    tga_image.fill_triangle(&GREEN,&t0);
//    tga_image.fill_triangle(&GREEN,&t1);
//    tga_image.fill_triangle(&GREEN,&t2);
//
//
//        tga_image.draw_line(&t0[0], &t0[1], &RED);
//        tga_image.draw_line(&t0[1], &t0[2], &RED);
//        tga_image.draw_line(&t0[2], &t0[0], &RED);
//
//    tga_image.draw_line(&t1[0], &t1[1], &RED);
//    tga_image.draw_line(&t1[1], &t1[2], &RED);
//    tga_image.draw_line(&t1[2], &t1[0], &RED);
//
//    tga_image.draw_line(&t2[0], &t2[1], &RED);
//    tga_image.draw_line(&t2[1], &t2[2], &RED);
//    tga_image.draw_line(&t2[2], &t2[0], &RED);

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

struct Triangle((usize,usize),(usize,usize),(usize,usize));
#[derive(Debug)]
struct Vector(f32,f32,f32);
impl Vector{
    fn vector_prod(&self,vector:&Vector)->Vector{
        Vector(self.1*vector.2-self.2*vector.1,
               self.2*vector.0-self.0*vector.2,
               self.0*vector.1-self.1*vector.0)
    }
    fn normalize(mut self)->Vector{
        let inv_length=1.0/self.length();
        self.0=self.0*inv_length;
        self.1=self.1*inv_length;
        self.2=self.2*inv_length;
        self
    }
    fn scalar_prod(&self,vector:&Vector)->f32{
        self.0*vector.0+self.1*vector.1+self.2*vector.2
    }
    fn length(&self)->f32{
        (self.0*self.0+self.1*self.1+self.2*self.2).sqrt()
    }
}

struct D_point{
    x:f32,
    y:f32,
    z:f32}
impl D_point{
    fn new(x:f32,y:f32,z:f32)->D_point{
        D_point{x,y,z}
    }
    fn to_point(&self,height:usize,width:usize)->Point{
        Point::new((self.x*width as f32 )as usize,(self.y*height as f32) as usize)
    }
    fn to_vector(&self,end_of_vector:&D_point)->Vector{
        Vector(end_of_vector.x-self.x,end_of_vector.y-self.y,end_of_vector.z-self.z)
    }
}