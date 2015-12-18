use std::ops::{ Index, IndexMut };

#[allow(dead_code)]
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

#[derive(Copy, Clone, PartialEq)]
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

struct RasterState {
    step: Vec3,
    d: Vec3,
    major_axis: usize,
}

struct LineRasterizer {
    from: Vec3,
    to: Vec3,

    state: Option<RasterState>
}

impl LineRasterizer {

    fn new(from: Vec3, to: Vec3) -> LineRasterizer {
        LineRasterizer {
            from: from,
            to: to,
            state: None
        }
    }

    fn next_point(&mut self) -> Option<Vec3> {
        match self.state {
            None => {
                let mut state = RasterState {
                    step: Vec3 { x: 0, y: 0, z: 0 },
                    d: Vec3 { x: 0, y: 0, z: 0 },
                    major_axis: 0
                };   

                let mut max = 0;
                for i in 0..3 {
                    let d = self.to[i] - self.from[i];
                    state.step[i] = if d > 0 { 1 } else { -1 };
                    
                    let d = d.abs();
                    if d > max {
                        max = d;
                        state.major_axis = i as usize;
                    };
                }

                self.state = Some(state);
                Some(self.from)
            },
            Some(ref mut state) => {
                if self.from == self.to {
                    None
                } else {
                    let from = self.from; let to = self.to;
                    let calc_residual_steps = |axis| { (to[axis] - from[axis]).abs() };
                    
                    self.from[state.major_axis] += state.step[state.major_axis];                    
                    let rs_base = calc_residual_steps(state.major_axis);
                    for i in 0..3 {
                        let rs = calc_residual_steps(i);
                        
                        if rs > 0 && i != state.major_axis {
                            state.d[i] += rs;
                            if state.d[i] >= rs_base {
                                state.d[i] -= rs_base;
                                self.from[i] += state.step[i];
                            }
                        }
                    }                    

                    Some(self.from)
                }
            },
        }
    }
}

impl Iterator for LineRasterizer
{
    type Item = Vec3;

    fn next(&mut self) -> Option<Self::Item> {
        self.next_point()
    }
}

#[allow(dead_code)]
fn test_code(canvas: &mut Canvas) {
    let a = Vec3 { x: 0, y:0, z:0 };
    let b = Vec3 { x: 50, y: 55, z: -20 };

    let rasterizer = LineRasterizer::new(a, b);
    for point in rasterizer {
        let color = std::u32::MAX;
        canvas.set_pixel(point.x as usize, point.y as usize, color);
    }
}

#[allow(dead_code)]
fn test_code_generic<T: Canvas>(canvas: &mut T) {
    let a = Vec3 { x: 0, y:0, z:0 };
    let b = Vec3 { x: 50, y: 55, z: -20 };

    let rasterizer = LineRasterizer::new(a, b);
    for point in rasterizer {
        let color = std::u32::MAX;
        canvas.set_pixel(point.x as usize, point.y as usize, color);
    }
}

#[allow(dead_code)]
fn test_code_boxed(canvas: &mut Box<Canvas>) {
    let a = Vec3 { x: 0, y:0, z:0 };
    let b = Vec3 { x: 50, y: 55, z: -20 };

    let rasterizer = LineRasterizer::new(a, b);
    for point in rasterizer {
        let color = std::u32::MAX;
        canvas.set_pixel(point.x as usize, point.y as usize, color);
    }
}

fn main() {
    let mut canvas = Pixmap::new(300, 300, 0);

    let a = Vec3 { x: 0, y:0, z:0 };
    let b = Vec3 { x: 10, y: 5, z: -4 };

    let rasterizer = LineRasterizer::new(a, b);
    for point in rasterizer {
        let color = std::u32::MAX;
        canvas.set_pixel(point.x as usize, point.y as usize, color);
        println!("Rust: point: x: {}, y: {}, z: {}, color: #{:X}", point.x, point.y, point.z, color);
    }
    
     for _ in 0..1000000 {
         test_code(&mut canvas)
     }
    
//     for _ in 0..1000000 {
//         test_code_generic(&mut canvas)
//     }

//    let mut boxed_canvas: Box<Canvas> = Box::new(canvas);
//    for _ in 0..1000000 {
//        test_code_boxed(&mut boxed_canvas)
//    }
}
