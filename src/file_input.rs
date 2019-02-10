use std::fs::File;
use crate::dimensional::DPoint;
use std::io::BufReader;
use std::io::BufRead;
use crate::texture::Texture;

pub fn read_file(file_path:&str) ->Vec<(Vec<DPoint>, Vec<DPoint>)> {
    let res=File::open(file_path).unwrap();
    let reader=BufReader::new(res);
    let mut points=Vec::new();
    let mut triangles=Vec::new();
    let mut texture_points=Vec::new();

    for line in reader.lines(){
        match line {
            Ok(line)=> {
                if line.starts_with("v ") || line.starts_with("vt "){
                    let subline=&line[2..];
                    let mut point:[f32;3]=[0.0;3];
                    for (index,coord) in subline.split_whitespace().enumerate(){
                        point[index]=coord.parse::<f32>().unwrap();
                    }
                    if line.starts_with("v "){
                        points.push(point);
                    }else {
                        texture_points.push(point);
                    }
                }
                if line.starts_with("f "){
                    let mut triangle:[usize;3]=[0;3];
                    let mut text_triangle:[usize;3]=[0;3];

                    let line=&line[2..];
                    for (index,coords) in line.split_whitespace().enumerate(){
                        let first_slash=coords.find('/').unwrap();
                        let last_splash=coords.rfind('/').unwrap();
                        let point=&coords[..first_slash];
                        triangle[index]=point.parse::<usize>().unwrap();
                        let tr_point=&coords[first_slash+1..last_splash];
                        text_triangle[index]=tr_point.parse::<usize>().unwrap();
                    }
                    triangles.push((triangle,text_triangle));
                }
            },
            Err(_e) =>()
        }
    }
    let mut real_coords =Vec::new();
    for triangle in triangles {
        let mut real_coord = Vec::new();
        for val in triangle.0.iter() {
            let point = *points.get(val - 1).unwrap();
            real_coord.push(DPoint::new(point[0], point[1], point[2]));
        }
        let mut real_text_coord=Vec::new();
        for text_val in triangle.1.iter(){
            let point = *texture_points.get(text_val - 1).unwrap();
            real_text_coord.push(DPoint::new(point[0], point[1], point[2]))
        }

        real_coords.push((real_coord,real_text_coord));
    }

    real_coords
}

pub fn read_texture_file(path:&str)->Texture{
    let texture=File::open(path).unwrap();
    let mut reader=BufReader::new(texture);

    let texture=imagefmt::read_from(&mut reader,imagefmt::ColFmt::RGBA).unwrap();
    let (height,width)=(texture.h,texture.w);
    let arr=texture.buf;
    Texture::new(height,width,arr)
}