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

fn main() {
    let WHITE:TGAColor=TGAColor::new(255,255,255,255);
    let RED:TGAColor=TGAColor::new(255,0,0,255);
    let GREEN:TGAColor=TGAColor::new(0,255,0,255);


    let mut tga_image=TGAImage::new(2000,2000);


    let (points,triangles)=read_file();
    for triangle in triangles{
        let mut triangle_real_coord:[(f32,f32);3]=[(0.0,0.0);3];
        for (index,point) in triangle.iter().enumerate(){
            let point:[f32;3]=*points.get(*point-1).unwrap();
            triangle_real_coord[index]=((point[0]+1.0)/2.0,(point[1]+1.0)/2.0);
        }
        let mut final_coord=Vec::new();
        for  (index,real_coorod) in triangle_real_coord.iter().enumerate(){
            final_coord.push(Point::new((real_coorod.0*tga_image.width as f32)as usize,(real_coorod.1*tga_image.height as f32)as usize));
        }
        tga_image.fill_triangle(&GREEN,final_coord.as_slice());

        tga_image.draw_line(&final_coord[0], &final_coord[1], &RED);
        tga_image.draw_line(&final_coord[1], &final_coord[2], &RED);
        tga_image.draw_line(&final_coord[2], &final_coord[0], &RED);

    }
//    let final_coord=[Point::new(50,50),Point::new(1900,150),Point::new(1000,1900)];
//    tga_image.fill_triangle(&GREEN,&final_coord);
//
//        tga_image.draw_line(&final_coord[0], &final_coord[1], &RED);
//        tga_image.draw_line(&final_coord[1], &final_coord[2], &RED);
//        tga_image.draw_line(&final_coord[2], &final_coord[0], &RED);

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
