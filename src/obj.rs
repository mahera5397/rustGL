use crate::texture::Texture;
use crate::dimensional::Vector;
use crate::file_input;
use std::collections::hash_map::Entry::Vacant;
use crate::matrix::Matrix;
use crate::plane::TGAImage;

pub struct Poly<'a>{
    coords:Vec<Vector<f32>>,
    text_coords:Vec<Vector<f32>>,
    norm_coords:Vec<Vector<f32>>,
    text_map:& 'a Texture,
    norm_map:& 'a Texture,
}

impl <'a>Poly <'a>{
    pub fn new(tulp:(Vec<Vector<f32>>, Vec<Vector<f32>>,Vec<Vector<f32>>),text_map:& 'a Texture, norm_map:& 'a Texture)-> Poly<'a> {
        Poly{coords:tulp.0,text_coords:tulp.1,norm_coords:tulp.2,text_map,norm_map}
    }
    fn draw_self(&mut self, image:&mut TGAImage, light:& Vector<f32>){
        image.fill_triangle(light,self.coords.as_mut_slice(),self.text_coords.as_mut_slice(),self.text_map
                            ,self.norm_coords.as_mut_slice(),self.norm_map);
    }
}

pub struct Object<'a>{
    polygons:Vec<Poly<'a>>,
    position:Vector<f32>,
    mod_matrix:Option<Matrix>,
}

impl <'a> Object<'a>{
    pub fn new(file_path:&str,text_map:& 'a Texture, norm_map:& 'a Texture,position:Vector<f32>)->Object<'a>{
        let mut triangles=file_input::read_file(file_path);
        let mut polygons=Vec::new();
        for mut triangle in triangles{
            triangle.1=triangle.1
                   .iter()
                   .map(|element| Vector::new(element.x*text_map.width as f32,
                                              element.y*text_map.height as f32,0.))
                   .collect::<Vec<Vector<f32>>>();
            triangle.2=triangle.2.iter()
                .map(|element| Vector::new(element.x*norm_map.width as f32,
                                              element.y*norm_map.height as f32,0.))
                   .collect::<Vec<Vector<f32>>>();

            polygons.push(Poly::new(triangle,text_map,norm_map))
        }

        Object{polygons,mod_matrix:None,position}
    }
    fn draw_self(&mut self,image:&mut TGAImage,light:& Vector<f32>,sight:& Vector<f32>){
        for poly in &mut self.polygons{
            let vec0=poly.coords[0]-poly.coords[1];
            let vec1=poly.coords[0]-poly.coords[2];

            let triangle_normal=vec0.vector_prod(vec1)
                .normalize();
            let intensity=triangle_normal.scalar_prod(sight);
            if intensity>0.0 {
                poly.draw_self(image, light);
            }
        }
    }
    pub fn rotate_x(&mut self,angle:f32)->&mut Self{
        //degree ffs
        let angle=angle/57.4;
        let (cos,sin)=(angle.cos(),angle.sin());
        self.mod_matrix=match &self.mod_matrix {
            None=>Some(Matrix::rotate_x(cos, sin)),
            Some(mat)=>Some(mat.multiply(&Matrix::rotate_x(cos, sin)))
        };
        self
    }
    pub fn rotate_y(&mut self,angle:f32)->&mut Self{
        let angle=angle/57.4;
        let (cos,sin)=(angle.cos(),angle.sin());
        self.mod_matrix=match &self.mod_matrix {
            None=>Some(Matrix::rotate_x(cos, sin)),
            Some(mat)=>Some(mat.multiply(&Matrix::rotate_y(cos, sin)))
        };
        self
    }
    pub fn rotate_z(&mut self,angle:f32)->&mut Self{
        let angle=angle/57.4;
        let (cos,sin)=(angle.cos(),angle.sin());
        self.mod_matrix=match &self.mod_matrix {
            None=>Some(Matrix::rotate_x(cos, sin)),
            Some(mat)=>Some(mat.multiply(&Matrix::rotate_z(cos, sin)))
        };
        self
    }

//    pub fn apply(&mut self){
//        if !self.is_applied{
//            let mod_matrix=self.mod_matrix.clone();
//            for  poly in self.polygons.iter_mut() {
//                poly.coords=
//                    poly.coords.iter()
//                    .map(|element|
//                        mod_matrix
//                            .multiply(&element.to_matrix())
//                            .to_vector())
//                    .collect::<Vec<Vector<f32>>>();
//                self.is_applied = true;
//            }
//        }
//    }
}

pub struct Scene<'a>{
    objects:Vec<Object<'a>>,
    image:TGAImage,
    light:Vector<f32>,
    view_port:Matrix,
    projection:Matrix,
    eye:Vector<f32>,
    up:Vector<f32>,
}

impl <'a> Scene<'a>{
    pub fn new(height:usize,width:usize,light:Vector<f32>)->Scene<'a>{
        let eye=Vector::new(-1.,-1.,3.);
        let mut projection=Matrix::ident(4);
        projection[3][2]=-1./eye.z;
        let up=Vector::new(0.,1.,0.);
        let view_port=Matrix::view_port(-2.,-2.,2.,2.);
        Scene{objects:Vec::new(),image:TGAImage::new(height,width),light,eye,projection,view_port,up}
    }

    pub fn add_obj(&mut self,obj:Object<'a>){
        self.objects.push(obj)
    }

    pub fn screen_basis(&mut self){
        for obj in &mut self.objects{
            let mut mod_matrix=self.view_port.multiply(&self.projection)
                .multiply(&look_at(self.eye,obj.position,self.up));
            if let Some(mat)=&obj.mod_matrix{
                mod_matrix=mod_matrix.multiply(mat);
            }
                //.multiply(&obj.mod_matrix);

            for poly in &mut obj.polygons{
                for point in poly.coords.iter_mut(){
                    *point=mod_matrix.multiply(&point.to_matrix())
                        .to_vector()
                        .to_plane(self.image.height,self.image.width);
                }
            }
        }
    }

    pub fn draw(&mut self)->TGAImage{
        //TODO do smth with it
        let FILE_OUTPUT_PATH:&str="image.tga";
        let visible=Vector::new(0.,0.,1.);
        for obj in &mut self.objects{
            obj.draw_self(&mut self.image,&self.light,&visible);
        }
        self.image.flip_vertically();
        self.image.write_tga_file(FILE_OUTPUT_PATH);

        self.image.clone()
    }
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