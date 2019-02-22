use crate::dimensional::Vector;
use num::NumCast;
use std::ops::Index;
use std::ops::Div;
use std::ops::IndexMut;

pub struct Matrix{
    pub matrix:Vec<Vec<f32>>,
    rows:usize,
    col:usize,
}

impl Matrix{
    pub fn new(rows:usize,col:usize)->Matrix{
        let mut matrix =Vec::new();
        for _ in 0..rows{
            let mut vec=Vec::new();
            for _ in 0..col {
                vec.push( 0. );
            }
            matrix.push(vec);
        }
        Matrix{matrix, rows,col}
    }

    pub fn multiply(&self,other:&Matrix)->Matrix{
        assert_eq!(self.col,other.rows);
        let mut respond=Matrix::new(self.rows, other.col);
        for i in 0..self.rows {
            for j in 0..other.col{
                for k in 0..self.col{
                    respond[i][j]+=self[i][k]*other[k][j];
                }
            }
        }
        respond
    }

    pub fn transpose(&self)->Matrix{
        let mut response=Matrix::new(self.col,self.rows);
        for i in 0..self.rows {
            for j in 0..self.col{
                response[j][i]=self[i][j];
            }
        }
        response
    }

    pub fn inverse(&self)->Matrix{
        assert_eq!(self.col,self.rows);
        let mut result =Matrix::new(self.rows, self.col*2);
        for i in 0..self.rows{
            for j in 0..self.col{
                result[i][j]=self[i][j];
            }
        }
        for i in 0..self.rows{
            result[i][self.col+i]=1.;
        }
        for i in 0..self.rows-1{
            for j in 0..result.col{
                let borrowed=result.col;
                result[i][borrowed-1-j]/=result[i][i];
            }
            for j in i+1..self.rows{
                let coeff=result[j][i];
                for k in 0..result.col{
                    result[j][k]-=result[i][k]*coeff;
                }
            }
        }
        for i in 1..&result.col-&result.rows+1{
            let borrow=result.col;
            result[self.rows-1][borrow-i]/=result[self.rows-1][self.rows-1];
        }
        for i in 1..result.rows{
            for k in 0..i{
                let coeff=result[i-k-1][result.rows-i];
                for j in 0..result.col{
                    result[i-k-1][j]-=result[result.rows-i][j]*coeff;
                }
            }
        }
        let mut truncate =Matrix::new(self.rows, self.col);
        for i in 0..truncate.rows{
            for j in 0..truncate.col{
                truncate[i][j]=result[i][j+self.col];
            }
        }
        truncate
    }

    pub fn to_vector(&self)->Vector<f32>{
        Vector::new(self[0][0]/self[3][0], self[1][0]/self[3][0], self[2][0]/self[3][0])
    }

}
impl Matrix {
    pub fn from_vector(vector: &Vector<f32>) -> Matrix {
        let vec = vec![vec![vector.x], vec![vector.y], vec![vector.z], vec![1.]];
        Matrix { matrix: vec,col:1, rows:4 }
    }
    pub fn ident(size:usize)->Matrix{
        let mut respond =Vec::new();
        for i in 0..size{
            let mut vec=Vec::new();
            for j in 0..size {
                vec.push(if i==j{1.}else { 0. });
            }
            respond.push(vec);
        }
        Matrix{matrix:respond, rows:size,col:size}
    }
    pub fn zoom(zoom:f32)->Matrix{
        let mut matrix=Matrix::ident(4);
        matrix[0][0]=zoom;
        matrix[1][1]=zoom;
        matrix[2][2]=zoom;
        matrix
    }
    pub fn rotate_y(cos:f32, sin:f32) ->Matrix{
        let mut matrix=Matrix::ident(4);
        matrix[1][1]=cos;
        matrix[2][2]=cos;
        matrix[1][2]=-sin;
        matrix[2][1]=sin;
        matrix
    }
    pub fn rotate_x(cos:f32, sin:f32) ->Matrix{
        let mut matrix=Matrix::ident(4);
        matrix[0][0]=cos;
        matrix[2][2]=cos;
        matrix[0][2]=-sin;
        matrix[2][0]=sin;
        matrix
    }
    pub fn rotate_z(cos:f32,sin:f32)->Matrix{
        let mut matrix=Matrix::ident(4);
        matrix[1][1]=cos;
        matrix[0][0]=cos;
        matrix[0][1]=-sin;
        matrix[1][0]=sin;
        matrix
    }
    pub fn translation(vector:&Vector<f32>)->Matrix{
        let mut matrix=Matrix::ident(4);
        matrix[0][3]=vector.x;
        matrix[1][3]=vector.y;
        matrix[2][3]=vector.z;
        matrix
    }
    pub fn view_port(x:f32,y:f32,width:f32,height:f32)->Matrix{
        let mut matrix=Matrix::ident(4);
        matrix[0][3] = (x+width) as f32/2.;
        matrix[1][3] = (y+height) as f32/2.;
        //wtf is depth
        matrix[2][3] = 1.;

        matrix[0][0] = width as f32/2.;
        matrix[1][1] = height as f32/2.;
        matrix[2][2] = 1.;
        matrix
    }
}

impl Index<usize> for Matrix{
    type Output = Vec<f32>;

    fn index(&self, index: usize) -> &Self::Output {
       & self.matrix[index]
    }
}

impl IndexMut<usize> for Matrix{
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
            & mut self.matrix[index]
    }
}