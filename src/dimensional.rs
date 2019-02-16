use crate::plane::TGAImage;
use std::ops::Mul;
use std::ops::Add;
use std::ops::Neg;
use core::mem;
use num::traits::NumCast;

#[derive(Debug,Copy,Clone)]
pub struct Vector<T>{
    pub x:T,
    pub y:T,
    pub z:T
}

impl<T> Vector<T> where T:Mul+Add+Neg+Copy+NumCast{
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
    fn to_f32(self)->Vector<f32> {
        Vector::new(num::cast(self.x).unwrap(), num::cast(self.y).unwrap(),
                    num::cast(self.z).unwrap() )
    }
}

impl Mul<f32> for Vector<f32>{
    type Output = Vector<f32>;

    fn mul(self, rhs: f32) -> Self::Output {
        Vector::new(self.x*rhs,self.y*rhs,self.z*rhs)
    }
}

impl<T> Add for Vector<T> where T:Mul+Add<Output=T>+Neg+Copy+NumCast{
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
       Vector::new(self.x+rhs.x,self.y+rhs.y,self.z+rhs.z)
    }
}