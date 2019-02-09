use std::result::Result::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use simpleOpenGL::dimensional::DPoint;
use simpleOpenGL::dimensional::Vector;
use simpleOpenGL::plane::Point;
use simpleOpenGL::plane::TGAColor;
use simpleOpenGL::plane::TGAImage;


const FILE_OUTPUT_PATH:&str="image.tga";
const FILE_INPUT_PATH:&str="african_head.obj";
const LIGHT_DIR:Vector=Vector(0.0,0.0,1.0);
const SIZE:usize=10000;

fn main() {
    let mut tga_image=TGAImage::new(SIZE,SIZE);


    let triangles=read_file(FILE_INPUT_PATH);
    for triangle in &triangles{

        let vec0=triangle[0].to_vector(&triangle[1]);
        let vec1=triangle[0].to_vector(&triangle[2]);

        let triangle_normal=vec0.vector_prod(&vec1)
            .normalize();
        let intensity=triangle_normal.scalar_prod(&LIGHT_DIR);

        if intensity>0.0{
            let color=TGAColor::new((255.0*intensity) as u8,(255.0*intensity) as u8,(255.0*intensity) as u8,255);
            tga_image.fill_triangle(color,triangle
                .iter()
                .map(|element|element.to_point(SIZE,SIZE))
                .collect::<Vec<Point>>()
                .as_mut_slice());
        }

    }
//    let color=TGAColor::new(255,0,0,255);
//    for triangle in triangles{
//        tga_image.draw_line(&triangle[0].to_point(SIZE,SIZE),&triangle[1].to_point(SIZE,SIZE),&color);
//        tga_image.draw_line(&triangle[0].to_point(SIZE,SIZE),&triangle[2].to_point(SIZE,SIZE),&color);
//        tga_image.draw_line(&triangle[1].to_point(SIZE,SIZE),&triangle[2].to_point(SIZE,SIZE),&color);
//
//    }
    tga_image.flip_vertically();

//    tga_image.fill_triangle(color,[Point::new(500,50,0),Point::new(2500,50,0),Point::new(2000,2000,0)].as_mut());
//    tga_image.draw_line(&Point::new(500,50,0),&Point::new(2500,50,0),&TGAColor::new(0,255,0,255));
//    tga_image.draw_line(&Point::new(500,50,0),&Point::new(2000,2000,0),&TGAColor::new(0,255,0,255));
//    tga_image.draw_line(&Point::new(2000,2000,0),&Point::new(2500,50,0),&TGAColor::new(0,255,0,255));

    tga_image.write_tga_file(FILE_OUTPUT_PATH);
}

fn read_file(file_path:&str)->Vec<Vec<DPoint>>
    //->Result<Vec<String>,String>
{
    let res=File::open(file_path).unwrap();
    let reader=BufReader::new(res);
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
            Err(_e) =>()
        }
    }
    let mut response =Vec::new();
    for triangle in triangles {
        let mut real_coord = Vec::new();
        for val in triangle.iter() {
            let point = *points.get(val - 1).unwrap();
            real_coord.push(DPoint::new(point[0], point[1], point[2]));
        }
        response.push(real_coord);
    }
    response
}