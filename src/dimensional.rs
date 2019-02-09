use crate::plane::Point;
use crate::plane::TGAImage;

#[derive(Debug)]
pub struct Vector(pub f64,pub f64,pub f64);
impl Vector{
    pub fn vector_prod(&self,vector:&Vector)->Vector{
        Vector(self.1*vector.2-self.2*vector.1,
               self.2*vector.0-self.0*vector.2,
               self.0*vector.1-self.1*vector.0)
    }
    pub fn normalize(mut self)->Vector{
        let inv_length=1.0/self.length();
        self.0=self.0*inv_length;
        self.1=self.1*inv_length;
        self.2=self.2*inv_length;
        self
    }
    pub fn scalar_prod(&self,vector:&Vector)->f64{
        self.0*vector.0+self.1*vector.1+self.2*vector.2
    }
    pub fn length(&self)->f64{
        (self.0*self.0+self.1*self.1+self.2*self.2).sqrt()
    }
}

#[derive(Debug)]
pub struct DPoint {
    pub x:f64,
    pub y:f64,
    pub z:f64}
impl DPoint {
    pub fn new(x:f64,y:f64,z:f64)-> DPoint {
        DPoint {x,y,z}
    }
    pub fn to_point(&self,height:usize,width:usize)->Point{
        Point::new((((self.x +1.0)/2.0)*width as f64 )as usize,(((self.y +1.0)/2.0)*height as f64) as usize,
                   (((self.z +1.0)/2.0)*height as f64) as usize)
    }
    pub fn to_vector(&self, end_of_vector:&DPoint) ->Vector{
        Vector(end_of_vector.x-self.x,end_of_vector.y-self.y,end_of_vector.z-self.z)
    }
}