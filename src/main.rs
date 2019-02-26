use simpleOpenGL::dimensional::Vector;
use simpleOpenGL::file_input::read_texture_file;
use simpleOpenGL::plane::TGAColor;
use simpleOpenGL::obj::Scene;
use simpleOpenGL::obj::Object;

const FILE_OUTPUT_PATH:&str="image.tga";

const HEAD_OBJ_PATH:&str="african_head.obj";
const HEAD_TEXTURE_PATH:&str="diff_text.tga";
const HEAD_NORMAL_PATH:&str="norm_map.tga";

const EYE_OBJ_PATH:&str="eye.obj";
const EYE_TEXTURE_PATH:&str="eye_diff.tga";
const EYE_NORMAL_PATH:&str="eye_nm.tga";

const EYE_OUTER_OBJ_PATH:&str="eye_outer.obj";
const EYE_OUTER_TEXTURE_PATH:&str="eye_outer_diff.tga";
const EYE_OUTER_NORMAL_PATH:&str="eye_outer_nm.tga";

const FLOOR_OBJ_PATH:&str="floor.obj";
const FLOOR_TEXTURE_PATH:&str="floor_diff.tga";
const FLOOR_NORMAL_PATH:&str="floor_nm.tga";

const SIZE:usize=2000;

fn main(){
    let head_texture=read_texture_file(HEAD_TEXTURE_PATH);
    let head_nm=read_texture_file(HEAD_NORMAL_PATH);
    let eye_texture=read_texture_file(EYE_TEXTURE_PATH);
    let eye_nm=read_texture_file(EYE_NORMAL_PATH);
    let eye_outer_texture=read_texture_file(EYE_OUTER_TEXTURE_PATH);
    let eye_outer_nm=read_texture_file(EYE_OUTER_NORMAL_PATH);
    let floor_texture=read_texture_file(FLOOR_TEXTURE_PATH);
    let floor_nm=read_texture_file(FLOOR_NORMAL_PATH);

    let LIGHT_DIR=Vector::new(0.0,0.0,-1.0).normalize();
    let mut scene = Scene::new(SIZE, SIZE, LIGHT_DIR);

    let position=Vector::new(0.,0.,0.);
        //.normalize();

    let mut head=Object::new(HEAD_OBJ_PATH,&head_texture,&head_nm,position);
    let mut eyes =Object::new(EYE_OBJ_PATH,&eye_texture,&eye_nm,position);
    let mut eyes_outer =Object::new(EYE_OUTER_OBJ_PATH,&eye_outer_texture,&eye_outer_nm,position);
    let floor =Object::new(FLOOR_OBJ_PATH,&floor_texture,&floor_nm,position);

    head.rotate_x(20.)
        .rotate_y(20.)
        .rotate_z(20.);
    eyes.rotate_x(20.)
        .rotate_y(20.)
        .rotate_z(20.);
    eyes_outer.rotate_x(20.)
        .rotate_y(20.)
        .rotate_z(20.);

    scene.add_obj(head);
    scene.add_obj(eyes);
    //scene.add_obj(eyes_outer);
    //scene.add_obj(floor);

    scene.screen_basis();
    scene.draw();
}