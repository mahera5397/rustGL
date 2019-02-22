use simpleOpenGL::dimensional::Vector;
use simpleOpenGL::plane::TGAImage;
use simpleOpenGL::file_input::read_file;
use simpleOpenGL::file_input::read_texture_file;
use simpleOpenGL::plane::TGAColor;
use simpleOpenGL::matrix::Matrix;

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
    let mut camera=Matrix::ident(4);
    camera[3][2]=-1./2.;

    let mod_matrix=        Matrix::view_port(-2.,-2.,2.,2.)
        .multiply(&camera)
        .multiply(&Matrix::rotate_y(0.86, 0.5))
        .multiply(&Matrix::rotate_x(0.86, 0.5))
        .multiply(&Matrix::rotate_z(0.86, 0.5));

    for triangle in &triangles{

        trans_trngl=triangle.0
            .iter()
            .map(|element|
               mod_matrix
                   .multiply(&element.to_matrix())
                   .to_vector()
                   .to_plane(SIZE,SIZE))
            .collect::<Vec<Vector<f32>>>();

        let vec0=trans_trngl[0]-trans_trngl[1];
        let vec1=trans_trngl[0]-trans_trngl[2];

        let triangle_normal=vec0.vector_prod(vec1)
            .normalize();
        let intensity=triangle_normal.scalar_prod(&LIGHT_DIR);

        trans_text_trngl= triangle.1
            .iter()
            .map(|element| Vector::new(element.x*texture.width as f32,
                                       element.y*texture.height as f32,0.))
            .collect::<Vec<Vector<f32>>>();

        if intensity>0.0{
            tga_image.fill_triangle(intensity,trans_trngl.as_mut_slice(),trans_text_trngl.as_mut_slice(),
            &texture);
        }
    }
    tga_image.flip_vertically();
    tga_image.write_tga_file(FILE_OUTPUT_PATH);
}