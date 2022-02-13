use crate::{util::Random, models::Image};

use super::Mutator;

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

#[derive(Default)]
pub struct TriangleMutator {
    random: Random,
}

impl TriangleMutator {
    fn get_random_point(&mut self, image: &Image) -> Point {
        let x = self.random.get_random(0isize, image.width() as isize - 1);
        let y = self.random.get_random(0isize, image.height() as isize - 1);

        Point { x, y }
    }

    fn get_triangle_vertices(&mut self, image: &Image) -> (Point, Point, Point) {
        let mut vertices = Vec::new();

        while vertices.len() != 3 {
            let point = self.get_random_point(image);

            if !vertices.contains(&point) {
                vertices.push(point);
            }
        }

        // The points must be sorted vertically. Required by the algorithm.
        vertices.sort_by(|p1, p2| p1.y.cmp(&p2.y));

        (vertices[0], vertices[1], vertices[2])
    }
}

impl Mutator for TriangleMutator {
    fn mutate_rgb(&mut self, image: &mut Image) {
        let (p0, p1, p2) = self.get_triangle_vertices(image);

        let r = self.random.get_random(0u8, 255);
        let g = self.random.get_random(0u8, 255);
        let b = self.random.get_random(0u8, 255);

        let image_width = image.width() as isize;
        let image_height = image.height() as isize;

        let dx_far = (p2.x - p0.x) as f64 / (p2.y - p0.y + 1) as f64;
        let dx_upper = (p1.x - p0.x) as f64 / (p1.y - p0.y + 1) as f64;
        let dx_low = (p2.x - p1.x) as f64 / (p2.y - p1.y +1) as f64;

        let mut xf = p0.x as f64;
        let mut xt = p0.x as f64 + dx_upper;

        let mut y = p0.y as isize;
        while y <= if p2.y > image_height - 1 {image_height - 1} else {p2.y} {
            if y >= 0 {
                let mut x = if xf > 0f64 {xf as isize} else {0};
                while x <= if xt < image_width as f64 {xt as isize} else {image_width - 1} {
                    let idx = (y * image_width + x) as usize;
                    image[idx].r(r);
                    image[idx].g(g);
                    image[idx].b(b);

                    x += 1;
                }

                let mut x = if xf < image_width as f64 {xf as isize} else {image_width - 1};
                while x >= if xt > 0f64 {xt as isize} else {0} {
                    let idx = (y * image_width + x) as usize;
                    image[idx].r(r);
                    image[idx].g(g);
                    image[idx].b(b);

                    x -= 1;
                }
            }
            xf += dx_far;

            if y < p1.y {
                xt += dx_upper;
            } else {
                xt += dx_low;
            }

            y += 1;
        }
    }

    fn mutate_grayscale(&mut self, image: &mut Image) {
        let (p0, p1, p2) = self.get_triangle_vertices(image);

        let grayscale = self.random.get_random(0u8, 255);

        let image_width = image.width() as isize;
        let image_height = image.height() as isize;

        let dx_far = (p2.x - p0.x) as f64 / (p2.y - p0.y + 1) as f64;
        let dx_upper = (p1.x - p0.x) as f64 / (p1.y - p0.y + 1) as f64;
        let dx_low = (p2.x - p1.x) as f64 / (p2.y - p1.y +1) as f64;

        let mut xf = p0.x as f64;
        let mut xt = p0.x as f64 + dx_upper;

        let mut y = p0.y as isize;
        while y <= if p2.y > image_height - 1 {image_height - 1} else {p2.y} {
            if y >= 0 {
                let mut x = if xf > 0f64 {xf as isize} else {0};
                while x <= if xt < image_width as f64 {xt as isize} else {image_width - 1} {
                    let idx = (y * image_width + x) as usize;
                    image[idx].set_grayscale(grayscale);

                    x += 1;
                }

                let mut x = if xf < image_width as f64 {xf as isize} else {image_width - 1};
                while x >= if xt > 0f64 {xt as isize} else {0} {
                    let idx = (y * image_width + x) as usize;
                    image[idx].set_grayscale(grayscale);

                    x -= 1;
                }
            }
            xf += dx_far;

            if y < p1.y {
                xt += dx_upper;
            } else {
                xt += dx_low;
            }

            y += 1;
        }
    }
}
