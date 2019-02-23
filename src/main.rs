use simpleOpenGL::dimensional::Vector;
use simpleOpenGL::plane::TGAImage;
use simpleOpenGL::file_input::read_file;
use simpleOpenGL::file_input::read_texture_file;
use simpleOpenGL::plane::TGAColor;
use simpleOpenGL::matrix::Matrix;

const FILE_OUTPUT_PATH:&str="image.tga";
const FILE_INPUT_PATH:&str="african_head.obj";
const FILE_TEXTURE_PATH:&str="diff_text.tga";
const FILE_NORMAL_PATH:&str="norm_map.tga";

const SIZE:usize=2000;

fn main() {
    let LIGHT_DIR=Vector::new(1.0,1.0,3.0).normalize();
    let mut tga_image=TGAImage::new(SIZE,SIZE);

    let triangles=read_file(FILE_INPUT_PATH);
    let texture=read_texture_file(FILE_TEXTURE_PATH);
    let normal_map=read_texture_file(FILE_NORMAL_PATH);


    let mut trans_text_trngl;
    let mut trans_trngl;
    let mut trans_norm_trngl;

    let mut projection=Matrix::ident(4);
    let eye=Vector::new(1.,1.,3.);
    let position=Vector::new(0.,0.,0.);
    let up=Vector::new(0.,1.,0.);
    projection[3][2]=-1./eye.z;

    let visible=Vector::new(0.,0.,1.);

    let mod_matrix=        Matrix::view_port(-2.,-2.,2.,2.)
        .multiply(&projection)
        .multiply(&look_at(eye,position,up))
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
        let intensity=triangle_normal.scalar_prod(&visible);

        trans_text_trngl= triangle.1
            .iter()
            .map(|element| Vector::new(element.x*texture.width as f32,
                                       element.y*texture.height as f32,0.))
            .collect::<Vec<Vector<f32>>>();
        trans_norm_trngl=triangle.2
            .iter()
            .map(|element| Vector::new(element.x*normal_map.width as f32,
                                       element.y*normal_map.height as f32,0.))
            .collect::<Vec<Vector<f32>>>();

        if intensity>0.0{
            tga_image.fill_triangle(&LIGHT_DIR,trans_trngl.as_mut_slice(),trans_text_trngl.as_mut_slice(),
            &texture,trans_norm_trngl.as_mut_slice(),&normal_map);
        }
    }
    tga_image.flip_vertically();
    tga_image.write_tga_file(FILE_OUTPUT_PATH);
}

fn look_at(eye:Vector<f32>,center:Vector<f32>,up:Vector<f32>)->Matrix{
    let z_vec=(eye-center).normalize();
    let x=up.vector_prod(z_vec);
    let y=z_vec.vector_prod(x);
    let (mut camera_view,mut trans_matrix)=(Matrix::ident(4),Matrix::ident(4));
    for i in 0..3{
        camera_view[0][i]=x[i];
        camera_view[1][i]=y[i];
        camera_view[2][i]=z_vec[i];
        trans_matrix[i][3]=-center[i];
    }
    camera_view.multiply(&trans_matrix)
}