use crate::texture::Texture;
use crate::dimensional::Vector;
use crate::file_input;
use crate::matrix::Matrix;
use crate::plane::TGAImage;
use std::thread;
use std::sync::Arc;
use std::fmt::Debug;


const MIN_ON_THREAD:usize=200;
const NUM_OF_THREAD:usize=4;

#[derive(Clone)]
pub struct Poly{
    coords:Vec<Vector<f32>>,
    text_coords:Vec<Vector<f32>>,
    norm_coords:Vec<Vector<f32>>,
}
impl Poly {
    pub fn new(tulp:(Vec<Vector<f32>>, Vec<Vector<f32>>,Vec<Vector<f32>>))-> Poly {
        Poly{coords:tulp.0,text_coords:tulp.1,norm_coords:tulp.2}
    }
    fn draw_self(&mut self, image:& Arc<TGAImage>, light: &Vector<f32>,text_map:&Arc<Texture>, norm_map:&Arc<Texture>){
        image.fill_triangle(light,self.coords.as_mut_slice(),self.text_coords.as_mut_slice(),text_map
                            ,self.norm_coords.as_mut_slice(),norm_map);
    }
}
pub struct Object{
    polygons:Vec<Poly>,
    position:Vector<f32>,
    mod_matrix:Option<Matrix>,
    text_map:Arc<Texture>,
    norm_map:Arc<Texture>,
    pointer:usize,
}

impl Object{
    pub fn new(file_path:&str,text_map:Arc<Texture>, norm_map:Arc<Texture>,position:Vector<f32>)->Object{
        let triangles=file_input::read_file(file_path);
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

            polygons.push(Poly::new(triangle))
        }

        Object{polygons,mod_matrix:None,position,text_map,norm_map,pointer:0}
    }

    fn from_obj(&self,first:usize,last:usize)->Object{
        let polygons=self.polygons[first..last].to_vec();
        Object{polygons,  mod_matrix:self.mod_matrix.clone()
            ,position:self.position,    text_map:Arc::clone(&self.text_map)
            ,norm_map:Arc::clone(&self.norm_map),pointer:0}
    }

    fn draw_self(&mut self,image:Arc<TGAImage>,light: Vector<f32>,sight: Vector<f32>) {
        for poly in &mut self.polygons {
            let vec0 = poly.coords[0] - poly.coords[1];
            let vec1 = poly.coords[0] - poly.coords[2];

            let triangle_normal = vec0.vector_prod(vec1)
                .normalize();
            let intensity = triangle_normal.scalar_prod(&sight);
            if intensity > 0.0 {
                poly.draw_self(&image, &light, &self.text_map, &self.norm_map);
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

impl PortionIterator for Object{
    type Item = Object;

    fn next(& mut self, portion: usize) -> Option<Self::Item> {
        if self.pointer>=self.polygons.len(){return None}

        let last_index=if self.pointer+portion>self.polygons.len(){
            self.polygons.len()
        }else{
            self.pointer+portion
        };
        let first_index=self.pointer;
        self.pointer=last_index;
        Some(self.from_obj(first_index,last_index))
}

    fn rewind(&mut self) {
        self.pointer=0;
    }
}

pub trait PortionIterator{
    type Item;

    fn next(& mut self,portion:usize)->Option<Self::Item>;
    fn rewind(&mut self);
}

pub struct Scene{
    objects:Vec<Object>,
    image:Arc<TGAImage>,
    view_port:Matrix,
    projection:Matrix,
    light:Vector<f32>,
    eye:Vector<f32>,
    up:Vector<f32>,
    visible:Vector<f32>,
    total_triangles:usize,
}

impl Scene{
    pub fn new(height:usize,width:usize,light:Vector<f32>)->Scene{
        let eye=Vector::new(-1.,-1.,3.);
        let mut projection=Matrix::ident(4);
        projection[3][2]=-1./eye.z;
        let up=Vector::new(0.,1.,0.);
        let view_port=Matrix::view_port(-2.,-2.,2.,2.);
        let image=Arc::new(TGAImage::new(height,width));
        let visible=Vector::new(0.,0.,1.);
        Scene{objects:Vec::new(),image,light,eye,visible,projection,view_port,up,total_triangles:0}
    }

    pub fn add_obj(&mut self,obj:Object){
        self.total_triangles+=obj.polygons.len();
        self.objects.push(obj);
    }

    pub fn screen_basis(&mut self){
        for obj in &mut self.objects{
            let mut mod_matrix=self.view_port.multiply(&self.projection)
                .multiply(&look_at(self.eye,obj.position,self.up));
            if let Some(mat)=&obj.mod_matrix{
                mod_matrix=mod_matrix.multiply(mat);
            }

            for poly in &mut obj.polygons{
                for point in poly.coords.iter_mut(){
                    *point=mod_matrix.multiply(&point.to_matrix())
                        .to_vector()
                        .to_plane(self.image.height,self.image.width);
                }
            }
        }
    }

    pub fn draw(& mut self)->&TGAImage{
        //TODO do smth with it
        let FILE_OUTPUT_PATH:&str="image.tga";

        let portion=if self.total_triangles/MIN_ON_THREAD>=NUM_OF_THREAD{
            self.total_triangles/(NUM_OF_THREAD)
        }else {
            MIN_ON_THREAD
        };

        let mut jobs=Vec::new();
        let mut job=Vec::new();
        for obj in self.objects.as_mut_slice(){
            loop {
                let part_obj = obj.next(portion);
                if let None = part_obj { break }
                job.push(part_obj.unwrap());
                let in_job: usize = job.iter().map(|el| el.polygons.len()).sum();
                if in_job >= portion {
                    jobs.push(job);
                    job = Vec::new();
                }
            }
        }
        jobs.push(job);
        let mut handles =Vec::new();

        for job in jobs{
            let (light,visible)=(self.light.clone(),self.visible.clone());
            let image=self.image.clone();
            let handle=thread::spawn(move|| {
                let image=image;
                for mut obj in job {
                    obj.draw_self(image.clone(),light,visible);
                }
            });
            handles.push(handle);
        }
        for handle in handles{
            handle.join().unwrap();
        }
        //let image=*self.image.as_ref();
        //self.image.to_owned().flip_vertically();
        self.image.write_tga_file(FILE_OUTPUT_PATH);
        &self.image //.clone()
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