extern crate sdl2;

use simpleOpenGL::dimensional::Vector;
use simpleOpenGL::file_input::read_texture_file;
use simpleOpenGL::obj::Scene;
use simpleOpenGL::obj::Object;
use simpleOpenGL::colors::Colors;

const FILE_OUTPUT_PATH:&str="image.tga";

const HEAD_OBJ_PATH:&str="objs/african_head.obj";
const HEAD_TEXTURE_PATH:&str="objs/diff_text.tga";
const HEAD_NORMAL_PATH:&str="objs/norm_map.tga";
const HEAD_SP_PATH:&str="objs/spec_map.tga";


const EYE_OBJ_PATH:&str="objs/eye.obj";
const EYE_TEXTURE_PATH:&str="objs/eye_diff.tga";
const EYE_NORMAL_PATH:&str="objs/eye_nm.tga";

const EYE_OUTER_OBJ_PATH:&str="eye_outer.obj";
const EYE_OUTER_TEXTURE_PATH:&str="eye_outer_diff.tga";
const EYE_OUTER_NORMAL_PATH:&str="eye_outer_nm.tga";

const FLOOR_OBJ_PATH:&str="floor.obj";
const FLOOR_TEXTURE_PATH:&str="floor_diff.tga";
const FLOOR_NORMAL_PATH:&str="floor_nm.tga";

const SIZE:usize=500;

fn get_scene() ->Scene{
    let head_texture=read_texture_file(HEAD_TEXTURE_PATH,Colors::RGBA).unwrap();
    let head_nm=read_texture_file(HEAD_NORMAL_PATH,Colors::RGBA).unwrap();
    let head_sp=read_texture_file(HEAD_SP_PATH,Colors::Gray).unwrap();

    let eye_texture=read_texture_file(EYE_TEXTURE_PATH,Colors::RGBA).unwrap();
    let eye_nm=read_texture_file(EYE_NORMAL_PATH,Colors::RGBA).unwrap();

    let LIGHT_DIR=Vector::new(1.0,1.0,-1.0).normalize();
    let mut scene = Scene::new(SIZE, SIZE, LIGHT_DIR);

    let position=Vector::new(0.,0.,0.);
        //.normalize();

    let head=Object::new(position)
        .set_text_map(head_texture)
        .set_norm_map(head_nm)
        .set_sp_map(head_sp)
        .build(HEAD_OBJ_PATH);
    let eyes =Object::new(position)
        .set_text_map(eye_texture)
        .set_norm_map(eye_nm)
        .build(EYE_OBJ_PATH);

    scene.add_obj(head);
    scene.add_obj(eyes);

    scene
}



use sdl2::pixels::PixelFormatEnum;
use sdl2::rect::Rect;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

pub fn main() -> Result<(), String> {
    let sdl_context = sdl2::init()?;
    let video_subsystem = sdl_context.video()?;

    let window = video_subsystem.window("rust-sdl2 demo: Video", SIZE as u32,SIZE as u32)
        .position_centered()
        .opengl()
        .build()
        .map_err(|e| e.to_string())?;

    let mut canvas = window.into_canvas().build().map_err(|e| e.to_string())?;
    let texture_creator = canvas.texture_creator();

    let mut texture = texture_creator.create_texture_streaming(PixelFormatEnum::RGBA32, SIZE as u32, SIZE as u32)
        .map_err(|e| e.to_string())?;

    let mut scene= get_scene();

    let mut buff=scene.draw().as_vec();

    texture.with_lock(None, move|buffer: &mut [u8] , _pitch: usize|
        buffer.swap_with_slice(buff[..buffer.len()].as_mut())
    )?;

    canvas.clear();
    canvas.copy(&texture, None, Some(Rect::new(0, 0, SIZE as u32, SIZE as u32)))?;
    canvas.present();

    let mut event_pump = sdl_context.event_pump()?;

    'running: loop {
        for event in event_pump.poll_iter() {
            match event {
                Event::Quit {..}
                | Event::KeyDown { keycode: Some(Keycode::Escape), .. } => {
                    break 'running
                },
                Event::KeyDown {..}=>{
                    match event {
                        Event::KeyDown {keycode: Some(Keycode::W),..}=>{
                            scene.objects[0].rotate_y(5.);
                            scene.objects[1].rotate_y(5.);
                        },
                        Event::KeyDown {keycode: Some(Keycode::S),..}=>{
                            scene.objects[0].rotate_y(-5.);
                            scene.objects[1].rotate_y(-5.);
                        },
                        Event::KeyDown {keycode: Some(Keycode::D),..}=>{
                            scene.objects[0].rotate_x(5.);
                            scene.objects[1].rotate_x(5.);
                        },
                        Event::KeyDown {keycode: Some(Keycode::A),..}=>{
                            scene.objects[0].rotate_x(-5.);
                            scene.objects[1].rotate_x(-5.);
                        },
                        _ => {}
                    }
                    let mut buff=scene.draw().as_vec();
                    texture.with_lock(None, move|buffer: &mut [u8] , _pitch: usize|
                        buffer.swap_with_slice(buff[..buffer.len()].as_mut())
                    )?;

                    canvas.copy(&texture, None, Some(Rect::new(0, 0, SIZE as u32, SIZE as u32)))?;
                    canvas.present();
                },
                _ => {}
            }
        }
    }

    Ok(())
}