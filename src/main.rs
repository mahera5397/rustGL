use std::result::Result::*;
use std::fs::File;
use std::io::BufReader;
use std::io::BufRead;
use simpleOpenGL::dimensional::DPoint;
use simpleOpenGL::dimensional::Vector;
use simpleOpenGL::plane::Point;
use simpleOpenGL::plane::TGAColor;
use simpleOpenGL::plane::TGAImage;
use simpleOpenGL::file_input::read_file;
use simpleOpenGL::file_input::read_texture_file;
use simpleOpenGL::texture::Triangle;


const FILE_OUTPUT_PATH:&str="image.tga";
const FILE_INPUT_PATH:&str="african_head.obj";
const FILE_TEXTURE_PATH:&str="diff_text.tga";

const LIGHT_DIR:Vector=Vector(0.0,0.0,1.0);
const SIZE:usize=10000;

fn main() {
    let mut tga_image=TGAImage::new(SIZE,SIZE);


    let triangles=read_file(FILE_INPUT_PATH);
    let texture=read_texture_file(FILE_TEXTURE_PATH);


    let mut trans_text_trngl;
    let mut trans_trngl;

    for triangle in &triangles{

        let vec0=triangle.0[0].to_vector(&triangle.0[1]);
        let vec1=triangle.0[0].to_vector(&triangle.0[2]);

        let triangle_normal=vec0.vector_prod(&vec1)
            .normalize();
        let intensity=triangle_normal.scalar_prod(&LIGHT_DIR);

        trans_trngl=triangle.0
            .iter()
            .map(|element|element.to_plane_point(SIZE, SIZE))
            .collect::<Vec<Point>>();

        trans_text_trngl= triangle.1
            .iter()
            .map(|element|element.to_text_point(texture.height, texture.width))
            .collect::<Vec<Point>>();
        let text_triangle=Triangle::new(trans_text_trngl.as_mut_slice(),&texture);

//        println!("dim triangle {:?} text triangle {:?}",trans_trngl,trans_text_trngl);
//
//
        if intensity>0.0{
            tga_image.fill_triangle(intensity,trans_trngl.as_mut_slice(),&text_triangle);
        }

    }
//    for triangle in &triangles {
//        let trng=triangle.0
//            .iter()
//            .map(|element|element.to_plane_point(SIZE, SIZE))
//            .collect::<Vec<Point>>();
//
//        tga_image.draw_line(&trng[0], &trng[1], &TGAColor::new(0, 255, 0, 255));
//        tga_image.draw_line(&trng[1], &trng[2], &TGAColor::new(0, 255, 0, 255));
//        tga_image.draw_line(&trng[2], &trng[0], &TGAColor::new(0, 255, 0, 255));
//    }
    tga_image.flip_vertically();
    tga_image.write_tga_file(FILE_OUTPUT_PATH);
}