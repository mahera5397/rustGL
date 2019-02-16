use std::ops::Mul;
use std::ops::Add;
use num::traits::NumCast;
use std::ops::Sub;

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
    pub fn to_f32(self)->Vector<f32> {
        Vector::new(self.x.to_f32().unwrap(), self.y.to_f32().unwrap(),self.z.to_f32().unwrap() )
    }
    pub fn round(self)->Vector<usize>{
        Vector::new(self.x.to_usize().unwrap(), self.y.to_usize().unwrap(), self.z.to_usize().unwrap())
    }
    pub fn to_plane(self,height:usize,width:usize)->Vector<isize>{
        let vec=self.to_f32();
        Vector::new(((vec.x+1.)/2. * width as f32) as isize, ((vec.y+1.)/2. * height as f32) as isize
        ,((vec.z+1.)/2. * width as f32) as isize)
    }
}

impl<T,K> Mul<K> for Vector<T> where T:NumCast+Copy,K:NumCast+Copy{
    type Output = Vector<f32>;

    fn mul(self, rhs: K) -> Self::Output {
        let vec=self.to_f32();
        Vector::new(vec.x*rhs.to_f32().unwrap(),vec.y*rhs.to_f32().unwrap(),vec.z*rhs.to_f32().unwrap())
    }
}

impl<T> Add for Vector<T> where T:Add<Output=T>+NumCast+Copy{
    type Output = Vector<T>;

    fn add(self, rhs: Vector<T>) -> Self::Output {
       Vector::new(self.x+rhs.x,self.y+rhs.y,self.z+rhs.z)
    }
}

impl<T> Sub for Vector<T> where T:Sub<Output=T>+NumCast+Copy {
    type Output = Vector<T>;

    fn sub(self, rhs: Vector<T>) -> Self::Output {
        Vector::new(self.x-rhs.x,self.y-rhs.y,self.z-rhs.z)
    }
}