mod geometrical_shapes;

use geometrical_shapes as gs;
use gs::{Displayable, Drawable};
use raster::{Color, Image};

fn main() {
    let mut image = Image::blank(1000, 1000);

    gs::Line::random(image.width, image.height).draw(&mut image);

    gs::Point::random(image.width, image.height).draw(&mut image);

    // Create points with longer lifetimes
    let point1 = gs::Point::new(150, 150);
    let point2 = gs::Point::new(50, 50);
    let rectangle = gs::Rectangle::new(&point1, &point2);
    rectangle.draw(&mut image);

    // Create points with longer lifetimes
    let point_a = gs::Point::new(500, 500);
    let point_b = gs::Point::new(250, 700);
    let point_c = gs::Point::new(700, 800);
    let triangle = gs::Triangle::new(&point_a, &point_b, &point_c);
    triangle.draw(&mut image);

    for _ in 1..50 {
        gs::Circle::random(image.width, image.height).draw(&mut image);
    }

    // Add pentagon
    let center_point = gs::Point::new(300, 300);
    let pentagon = gs::Pentagon::new(&center_point, 100);
    pentagon.draw(&mut image);

    // Add a random pentagon
    gs::Pentagon::random(image.width, image.height).draw(&mut image);

    // Add cube
    let cube_point = gs::Point::new(700, 200);
    let cube = gs::Cube::new(&cube_point, 120);
    cube.draw(&mut image);

    // Add a random cube
    gs::Cube::random(image.width, image.height).draw(&mut image);

    raster::save(&image, "image.png").unwrap();
}

impl Displayable for Image {
    fn display(&mut self, x: i32, y: i32, color: Color) {
        if x >= 0 && x < self.width && y >= 0 && y < self.height {
            self.set_pixel(x, y, color).unwrap();
        }
    }
}