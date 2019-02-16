use simpleOpenGL::dimensional::Vector;
use simpleOpenGL::plane::TGAImage;
use simpleOpenGL::file_input::read_file;
use simpleOpenGL::file_input::read_texture_file;

const FILE_OUTPUT_PATH:&str="image.tga";
const FILE_INPUT_PATH:&str="african_head.obj";
const FILE_TEXTURE_PATH:&str="diff_text.tga";

const SIZE:usize=2000;

fn main() {
    let LIGHT_DIR=Vector::new(0.0,0.0,1.0);
    let mut tga_image=TGAImage::new(SIZE,SIZE);


    let triangles=read_file(FILE_INPUT_PATH);
    let texture=read_texture_file(FILE_TEXTURE_PATH);


    let mut trans_text_trngl;
    let mut trans_trngl;

    for triangle in &triangles{

        let vec0=triangle.0[0]-(triangle.0[1]);
        let vec1=triangle.0[0]-(triangle.0[2]);

        let triangle_normal=vec0.vector_prod(vec1)
            .normalize();
        let intensity=triangle_normal.scalar_prod(&LIGHT_DIR);

        trans_trngl=triangle.0
            .iter()
            .map(|element|element.to_plane(SIZE,SIZE))
            .collect::<Vec<Vector<isize>>>();

        trans_text_trngl= triangle.1
            .iter()
            .map(|element| Vector::new((element.x*texture.width as f32)as isize,
                                       (element.y*texture.height as f32) as isize,0))
            .collect::<Vec<Vector<isize>>>();

        if intensity>0.0{
            tga_image.fill_triangle(intensity,trans_trngl.as_mut_slice(),trans_text_trngl.as_mut_slice(),
            &texture);
        }

    }
    tga_image.flip_vertically();
    tga_image.write_tga_file(FILE_OUTPUT_PATH);
}