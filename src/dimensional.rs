use std::ops::Mul;
use std::ops::Add;
use num::traits::NumCast;
use std::ops::Sub;
use std::ops::Div;
use std::ops::AddAssign;
use std::ops::SubAssign;
use crate::matrix::Matrix;
use std::ops::Index;

#[derive(Debug,Copy,Clone)]
pub struct Vector<T>{
    pub x:T,
    pub y:T,
    pub z:T
}

impl<T> Vector<T> where T:NumCast+Copy{
    pub fn new(x:T,y:T,z:T)->Vector<T>{
        Vector{x,y,z}
    }
    pub fn vector_prod(self,vector:Vector<T>)->Vector<f32>{
        let (vec1,vec2)=(self.to_f32(), vector.to_f32());
        Vector::new(vec1.y*vec2.z-vec1.z*vec2.y,
                    vec1.z*vec2.x-vec1.x*vec2.z,
                    vec1.x*vec2.y-vec1.y*vec2.x)
    }
    pub fn normalize(&self)->Vector<f32>{
        let inv_length=1.0/self.length();
        let vec=self.to_f32();
        Vector::new(vec.x*inv_length, vec.y*inv_length,vec.z*inv_length)
    }
    pub fn scalar_prod(&self,vector:&Vector<T>)->f32{
        let (vec1,vec2)=(self.to_f32(), vector.to_f32());
        vec1.x*vec2.x + vec1.y *vec2.y + vec1.z * vec2.z
    }
    pub fn length(&self)->f32{
        let vec=self.to_f32();
        (vec.x*vec.x+vec.y*vec.y+vec.z*vec.z).sqrt()
    }
    pub fn to_f32(&self)->Vector<f32> {
        Vector::new(self.x.to_f32().unwrap(), self.y.to_f32().unwrap(),self.z.to_f32().unwrap() )
    }
    pub fn round(&self)->Vector<usize>{
        Vector::new(self.x.to_usize().unwrap(), self.y.to_usize().unwrap(), self.z.to_usize().unwrap())
    }
    pub fn to_plane(&self,height:usize,width:usize)->Vector<f32>{
        let vec=self.to_f32();
//        println!("before {:?}",vec);
        Vector::new((vec.x+1.)/2. * width as f32, (vec.y+1.)/2. * height as f32,(vec.z+1.)/2. * width as f32)
    }
    pub fn set(&mut self,index:usize,val:T){
        match index {
            0=>self.x=val,
            1=>self.y=val,
            _=>self.z=val
        }
    }
    pub fn to_matrix(&self)->Matrix{
        Matrix::from_vector(&self.to_f32())
    }
}

impl Mul<f32> for Vector<f32>{
    type Output = Vector<f32>;

    fn mul(mut self, rhs: f32) -> Self::Output {
        self.x*=rhs; self.y*=rhs; self.z*=rhs;
        self
    }
}

impl<T> Add for Vector<T> where T:AddAssign+NumCast+Copy{
    type Output = Vector<T>;

    fn add(mut self, rhs: Vector<T>) -> Self::Output {
        self.x+=rhs.x; self.y+=rhs.y; self.z+=rhs.z;
        self
    }
}

impl<T> Sub for Vector<T> where T:SubAssign+NumCast+Copy {
    type Output = Vector<T>;

    fn sub(mut self, rhs: Vector<T>) -> Self::Output {
        self.x-=rhs.x;self.y-=rhs.y;self.z-=rhs.z;
        self
    }
}

impl Div<f32> for Vector<f32>{
    type Output = Vector<f32>;

    fn div(mut self, rhs: f32) -> Self::Output {
        if rhs==0. {return Vector::new(0.,0.,0.)};
        self.x/=rhs; self.y/=rhs; self.z/=rhs;
        self
    }
}

impl<T> Index<usize> for Vector<T> where T:Copy{
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        match index{
            0=>&self.x,
            1=>&self.y,
            _=>&self.z
        }
    }
}