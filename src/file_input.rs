use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use crate::texture::Texture;
use crate::dimensional::Vector;
use crate::colors::Colors;
use std::io;
use std::sync::Arc;

pub fn read_file(file_path:&str) ->Vec<(Vec<Vector<f32>>, Vec<Vector<f32>>,Vec<Vector<f32>>)> {
    let res=File::open(file_path).unwrap();
    let reader=BufReader::new(res);
    let mut points=Vec::new();
    let mut triangles=Vec::new();
    let mut texture_points=Vec::new();
    let mut normal_points=Vec::new();

    for line in reader.lines(){
        match line {
            Ok(line)=> {
                if line.starts_with("v ") || line.starts_with("vt ")|| line.starts_with("vn "){
                    let subline=&line[2..];
                    let mut point:[f32;3]=[0.0;3];
                    for (index,coord) in subline.split_whitespace().enumerate(){
                        point[index]=coord.parse::<f32>().unwrap();
                    }
                    if line.starts_with("v "){
                        points.push(point);
                    }else if line.starts_with("vt "){
                        texture_points.push(point);
                    }
                    else{
                        normal_points.push(point);
                    }
                }
                if line.starts_with("f "){
                    let mut triangle:[usize;3]=[0;3];
                    let mut text_triangle:[usize;3]=[0;3];
                    let mut norm_triangle:[usize;3]=[0;3];

                    let line=&line[2..];
                    for (index,coords) in line.split_whitespace().enumerate(){
                        let first_slash=coords.find('/').unwrap();
                        let last_slash=coords.rfind('/').unwrap();
                        let point=&coords[..first_slash];
                        triangle[index]=point.parse::<usize>().unwrap();
                        let tr_point=&coords[first_slash+1..last_slash];
                        text_triangle[index]=tr_point.parse::<usize>().unwrap();
                        norm_triangle[index]=tr_point.parse::<usize>().unwrap();
                    }
                    triangles.push((triangle,text_triangle,norm_triangle));
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
            real_coord.push(Vector::new(point[0], point[1], point[2]));
        }
        let mut real_text_coord=Vec::new();
        for text_val in triangle.1.iter(){
            let point = *texture_points.get(text_val - 1).unwrap();
            real_text_coord.push(Vector::new(point[0], point[1], point[2]))
        }
        let mut real_norm_coord=Vec::new();
        for text_val in triangle.2.iter(){
            let point = *texture_points.get(text_val - 1).unwrap();
            real_norm_coord.push(Vector::new(point[0], point[1], point[2]))
        }
        real_coords.push((real_coord,real_text_coord,real_norm_coord));
    }
    real_coords
}

pub fn read_texture_file(path:&str,color_format:Colors)->Result<Arc<Texture>,io::Error>{
    let texture=File::open(path)?;
    let mut reader=BufReader::new(texture);
    let texture=match color_format {
        Colors::RGBA=> imagefmt::read_from(&mut reader,imagefmt::ColFmt::RGBA).unwrap(),
        Colors::Gray=> imagefmt::read_from(&mut reader,imagefmt::ColFmt::Y).unwrap(),
    };
    let (height,width)=(texture.h,texture.w);
    let arr=texture.buf;
    Ok(Arc::new(Texture::new(height,width,arr,color_format)))
}