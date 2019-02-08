use crate::plane::Point;

#[derive(Debug)]
pub struct Vector(pub f32,pub f32,pub f32);
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
    pub fn scalar_prod(&self,vector:&Vector)->f32{
        self.0*vector.0+self.1*vector.1+self.2*vector.2
    }
    pub fn length(&self)->f32{
        (self.0*self.0+self.1*self.1+self.2*self.2).sqrt()
    }
}

#[derive(Debug)]
pub struct DPoint {
    pub x:f32,
    pub y:f32,
    pub z:f32}
impl DPoint {
    pub fn new(x:f32,y:f32,z:f32)-> DPoint {
        DPoint {x,y,z}
    }
    pub fn to_point(&self,height:usize,width:usize)->Point{
        Point::new((((self.x +1.0)/2.0)*width as f32 )as usize,(((self.y +1.0)/2.0)*height as f32) as usize)
    }
    pub fn to_vector(&self, end_of_vector:&DPoint) ->Vector{
        Vector(end_of_vector.x-self.x,end_of_vector.y-self.y,end_of_vector.z-self.z)
    }
}