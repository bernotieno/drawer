use raster::{Color, Image};
use rand::Rng;

pub trait Drawable {
    fn draw(&self, image: &mut Image);
    fn color(&self) -> Color;
}

pub trait Displayable {
    fn display(&mut self, x: i32, y: i32, color: Color);
}

// Point implementation
pub struct Point {
    pub x: i32,
    pub y: i32,
}

impl Point {
    pub fn new(x: i32, y: i32) -> Self {
        Point { x, y }
    }

    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        Point {
            x: rng.gen_range(0..width),
            y: rng.gen_range(0..height),
        }
    }
}

impl Drawable for Point {
    fn draw(&self, image: &mut Image) {
        image.display(self.x, self.y, self.color());
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(0..255),
            g: rng.gen_range(0..255),
            b: rng.gen_range(0..255),
            a: 255,
        }
    }
}

// Line implementation
pub struct Line {
    start: Point,
    end: Point,
}

impl Line {
    pub fn new(start: &Point, end: &Point) -> Self {
        Line { 
            start: Point::new(start.x, start.y), 
            end: Point::new(end.x, end.y)
        }
    }

    pub fn random(width: i32, height: i32) -> Self {
        Line {
            start: Point::random(width, height),
            end: Point::random(width, height),
        }
    }
}

impl Drawable for Line {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        
        // Bresenham's line algorithm
        let mut x0 = self.start.x;
        let mut y0 = self.start.y;
        let x1 = self.end.x;
        let y1 = self.end.y;
        
        let dx = (x1 - x0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let dy = -(y1 - y0).abs();
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;
        
        loop {
            image.display(x0, y0, color.clone());
            if x0 == x1 && y0 == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy {
                if x0 == x1 { break; }
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                if y0 == y1 { break; }
                err += dx;
                y0 += sy;
            }
        }
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(0..255),
            g: rng.gen_range(0..255),
            b: rng.gen_range(0..255),
            a: 255,
        }
    }
}

// Pentagon implementation
pub struct Pentagon {
    center: Point,
    radius: i32,
}

impl Pentagon {
    pub fn new(center: &Point, radius: i32) -> Self {
        Pentagon { 
            center: Point::new(center.x, center.y), 
            radius 
        }
    }
    
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let center = Point::random(width, height);
        let max_radius = (width.min(height) / 10).max(10);
        Pentagon {
            center,
            radius: rng.gen_range(5..max_radius),
        }
    }
}

impl Drawable for Pentagon {
    fn draw(&self, image: &mut Image) {
        // Calculate five points on a circle to form a pentagon
        let points: Vec<Point> = (0..5)
            .map(|i| {
                let angle = 2.0 * std::f64::consts::PI * (i as f64) / 5.0;
                Point::new(
                    self.center.x + (self.radius as f64 * angle.cos()) as i32,
                    self.center.y + (self.radius as f64 * angle.sin()) as i32,
                )
            })
            .collect();
        
        // Draw the pentagon by connecting the points
        for i in 0..5 {
            let next = (i + 1) % 5;
            Line::new(&points[i], &points[next]).draw(image);
        }
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(0..255),
            g: rng.gen_range(0..255),
            b: rng.gen_range(0..255),
            a: 255,
        }
    }
}

// Cube implementation
pub struct Cube {
    front_top_left: Point,
    size: i32,
    depth_factor: f64, // For perspective effect (0.0-1.0)
}

impl Cube {
    pub fn new(front_top_left: &Point, size: i32) -> Self {
        Cube { 
            front_top_left: Point::new(front_top_left.x, front_top_left.y), 
            size,
            depth_factor: 0.5,
        }
    }
    
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let max_size = (width.min(height) / 8).max(10);
        let size = rng.gen_range(10..max_size);
        
        // Make sure the cube fits within the image
        let x = rng.gen_range(size..(width - size));
        let y = rng.gen_range(size..(height - size));
        
        Cube {
            front_top_left: Point::new(x, y),
            size,
            depth_factor: rng.gen_range(0.3..0.7),
        }
    }
}

impl Drawable for Cube {
    fn draw(&self, image: &mut Image) {
        // let color = self.color();
        
        // Calculate depth offset based on size and depth_factor
        let depth = (self.size as f64 * self.depth_factor) as i32;
        
        // Front face points
        let ftl = &self.front_top_left; // front top left
        let ftr = &Point::new(ftl.x + self.size, ftl.y); // front top right
        let fbl = &Point::new(ftl.x, ftl.y + self.size); // front bottom left
        let fbr = &Point::new(ftl.x + self.size, ftl.y + self.size); // front bottom right
        
        // Back face points
        let btl = &Point::new(ftl.x + depth, ftl.y - depth); // back top left
        let btr = &Point::new(ftr.x + depth, ftr.y - depth); // back top right
        let bbl = &Point::new(fbl.x + depth, fbl.y - depth); // back bottom left
        let bbr = &Point::new(fbr.x + depth, fbr.y - depth); // back bottom right
        
        // Draw front face
        Line::new(ftl, ftr).draw(image);
        Line::new(ftr, fbr).draw(image);
        Line::new(fbr, fbl).draw(image);
        Line::new(fbl, ftl).draw(image);
        
        // Draw back face
        Line::new(btl, btr).draw(image);
        Line::new(btr, bbr).draw(image);
        Line::new(bbr, bbl).draw(image);
        Line::new(bbl, btl).draw(image);
        
        // Connect front and back faces
        Line::new(ftl, btl).draw(image);
        Line::new(ftr, btr).draw(image);
        Line::new(fbl, bbl).draw(image);
        Line::new(fbr, bbr).draw(image);
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(0..255),
            g: rng.gen_range(0..255),
            b: rng.gen_range(0..255),
            a: 255,
        }
    }
}

// Triangle implementation
pub struct Triangle {
    a: Point,
    b: Point,
    c: Point,
}

impl Triangle {
    pub fn new(a: &Point, b: &Point, c: &Point) -> Self {
        Triangle { 
            a: Point::new(a.x, a.y), 
            b: Point::new(b.x, b.y), 
            c: Point::new(c.x, c.y)
        }
    }
}

impl Drawable for Triangle {
    fn draw(&self, image: &mut Image) {
        let a_point = &self.a;
        let b_point = &self.b;
        let c_point = &self.c;
        
        Line::new(a_point, b_point).draw(image);
        Line::new(b_point, c_point).draw(image);
        Line::new(c_point, a_point).draw(image);
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(0..255),
            g: rng.gen_range(0..255),
            b: rng.gen_range(0..255),
            a: 255,
        }
    }
}

// Rectangle implementation
pub struct Rectangle {
    top_left: Point,
    bottom_right: Point,
}

impl Rectangle {
    pub fn new(point1: &Point, point2: &Point) -> Self {
        Rectangle { 
            top_left: Point::new(point1.x, point1.y), 
            bottom_right: Point::new(point2.x, point2.y) 
        }
    }
}

impl Drawable for Rectangle {
    fn draw(&self, image: &mut Image) {
        let top_right = Point::new(self.bottom_right.x, self.top_left.y);
        let bottom_left = Point::new(self.top_left.x, self.bottom_right.y);
        
        Line::new(&self.top_left, &top_right).draw(image);
        Line::new(&top_right, &self.bottom_right).draw(image);
        Line::new(&self.bottom_right, &bottom_left).draw(image);
        Line::new(&bottom_left, &self.top_left).draw(image);
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(0..255),
            g: rng.gen_range(0..255),
            b: rng.gen_range(0..255),
            a: 255,
        }
    }
}

// Circle implementation
pub struct Circle {
    center: Point,
    radius: i32,
}

impl Circle {
    #[allow(dead_code)]
    pub fn new(center: &Point, radius: i32) -> Self {
        Circle { 
            center: Point::new(center.x, center.y), 
            radius 
        }
    }
    
    pub fn random(width: i32, height: i32) -> Self {
        let mut rng = rand::thread_rng();
        let center = Point::random(width, height);
        let max_radius = (width.min(height) / 10).max(10);
        Circle {
            center,
            radius: rng.gen_range(5..max_radius),
        }
    }
}

impl Drawable for Circle {
    fn draw(&self, image: &mut Image) {
        let color = self.color();
        let mut x = self.radius;
        let mut y = 0;
        let mut err = 0;

        while x >= y {
            image.display(self.center.x + x, self.center.y + y, color.clone());
            image.display(self.center.x + y, self.center.y + x, color.clone());
            image.display(self.center.x - y, self.center.y + x, color.clone());
            image.display(self.center.x - x, self.center.y + y, color.clone());
            image.display(self.center.x - x, self.center.y - y, color.clone());
            image.display(self.center.x - y, self.center.y - x, color.clone());
            image.display(self.center.x + y, self.center.y - x, color.clone());
            image.display(self.center.x + x, self.center.y - y, color.clone());

            y += 1;
            if err <= 0 {
                err += 2 * y + 1;
            }
            if err > 0 {
                x -= 1;
                err -= 2 * x + 1;
            }
        }
    }

    fn color(&self) -> Color {
        let mut rng = rand::thread_rng();
        Color {
            r: rng.gen_range(0..255),
            g: rng.gen_range(0..255),
            b: rng.gen_range(0..255),
            a: 255,
        }
    }
}