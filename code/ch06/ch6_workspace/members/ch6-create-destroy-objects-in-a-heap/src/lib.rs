extern crate graphics;
extern crate piston_window;
extern crate rand;

use graphics::math::{ Vec2d, add, mul_scalar };

use rand::distributions::{Distribution,Uniform};

type RGBA = [f32; 4];
pub const WHITE: RGBA = [1.0; 4];
pub const GRAY: RGBA  = [0.7, 0.7, 0.7, 0.3];
pub const N_PARTICLES: usize = 500;

pub struct World {
    pub current_turn: usize,
    pub shapes: Vec<Box<Shape>>,
    pub height: u32,
    pub width: u32,
}

pub struct Shape {
    pub height: f64,
    pub width: f64,
    pub position: Vec2d<f64>,
    pub velocity: Vec2d<f64>,
    pub acceleration: Vec2d<f64>,
    pub color: RGBA,
}

impl Shape {
    pub fn new(x: f64, y: f64) -> Self {
        let mut rng = rand::thread_rng();
        let legal_range =Uniform::try_from( -5_f64..5_f64).unwrap();


        let x_speed = legal_range.sample(&mut rng);
        let y_speed = legal_range.sample(&mut rng);
        let x_accel = 0.1 * legal_range.sample(&mut rng);
        let y_accel = 0.1 * legal_range.sample(&mut rng);

        Shape {
            height: 10.0,
            width: 10.0,
            position: [x, y],
            velocity: [x_speed, y_speed],
            acceleration: [x_accel, y_accel],
            color: GRAY,
        }
    }

    pub fn update(&mut self) {
        self.velocity = add(self.velocity, self.acceleration); // <> There is no matrix/vector math operators within the language. `graphics::math` is providing this functionality for us.  
        self.position = add(self.position, self.velocity); 
        self.acceleration = mul_scalar(self.acceleration, 0.7); // <> Slow down the shape's movement
        self.color[3] *= 0.97; 
    }
}

impl World {
    pub fn new(width: u32, height: u32) -> World {
        World {
            current_turn: 0,
            shapes: Vec::<Box<Shape>>::new(),
            height: height,
            width: width,
        }
    }

    pub fn add_shapes(&mut self, n: usize) {
        let x = (self.width / 2) as f64;
        let y = (self.height / 2) as f64;

        for _ in 0..n {
            self.shapes.push(Box::new(Shape::new(x, y)));
        };
    }

    pub fn remove_shapes(&mut self, n: usize) {
        let n_shapes = self.shapes.len();

        let to_remove = if n > n_shapes {
            n_shapes
        } else {
            n
        };
        // let to_remove = cmp::min(n as usize, self.shapes.len());

        for _ in 0..to_remove {
            self.shapes.remove(0); // Remove the oldest particle. This is quite an inefficient operation, as all remaining particles are shifted to fill the now-empty slot. A smarter strategy would be to use `std::collections::VecDeque`, which supports removing from the front.
        }

        self.shapes.shrink_to_fit(); // Will help to force a re-allocation later when shapes are added.
    }

    pub fn calc_population_change(&self) -> isize {
        const N: f64 = N_PARTICLES as f64; // <> Shorter alias
        const MAX: f64 =  N*0.5;
        const MIN: f64 = -N*0.5;
        let x: f64 = self.current_turn as f64;

        //let n: f64 = N_PARTICLES;
        let n = 0.4*N*(0.1*x).sin() + 0.1*N*x.sin();
        n.max(MIN).min(MAX).round() as isize // limit range of growth/death then convert to `isize`
    }

    pub fn update(&mut self) {
        let n = self.calc_population_change();
        //let n = as usize; // <> Convert f64 to usize

        if n > 0 {
            self.add_shapes(n as usize);
        } else {
            self.remove_shapes(n.abs() as usize);
        }

        self.current_turn += 1;
    }
} 
