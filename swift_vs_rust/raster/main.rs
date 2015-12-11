use std::ops::{ Index, IndexMut };

struct GenericPixmap<T> {
    w: usize,
    h: usize,
    
    data: Vec<T> 
}

impl<T> GenericPixmap<T> 
    where T: Copy + Clone 
{
    fn new(w: usize, h: usize, fill_value: T) -> GenericPixmap<T> {
        GenericPixmap {
            w: w,
            h: h,
            data: vec![fill_value; w*h]
        }
    }
}

impl<T> Index<usize> for  GenericPixmap<T> 
    where T: Copy + Clone 
{
    type Output = [T];

    fn index<'a>(&'a self, i: usize) -> &'a Self::Output {
        let from = i*self.w;
        let to = from+self.w;
        &self.data[from..to]
    }
}
impl<T> IndexMut<usize> for GenericPixmap<T> 
    where T: Copy + Clone 
{
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
        let from = i*self.w;
        let to = from+self.w;
        &mut self.data[from..to]
    }    
}


type Pixmap = GenericPixmap<u32>;

trait Canvas
{
    fn set_pixel(&mut self, x: usize, y:usize, color:u32);
}

impl Canvas for Pixmap
{
    fn set_pixel(&mut self, x: usize, y:usize, color:u32)
    {
        self[x][y] = color;
    }
}

struct Vec3 {
    x: i32,
    y: i32,
    z: i32
}

impl Index<usize> for Vec3
{
    type Output = i32;

    fn index<'a>(&'a self, i: usize) -> &'a Self::Output {
        match i {
            0   => &self.x,
            1   => &self.y,
            2   => &self.z,
            _   => panic!("Wrong index"),
        }
    }
}
impl IndexMut<usize> for Vec3
{
    fn index_mut<'a>(&'a mut self, i: usize) -> &'a mut Self::Output {
        match i {
            0   => &mut self.x,
            1   => &mut self.y,
            2   => &mut self.z,
            _   => panic!("Wrong index"),
        }
    }    
}


fn main() {
    let canvas = Pixmap::new(300, 300, 0);
}
