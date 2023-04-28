use crate::{
    models::{Image, Pixel},
    util::Random,
};

use super::Mutator;

fn get_dx(vertices: &[Point; 3]) -> (f64, f64, f64) {
    (
        (vertices[2].x - vertices[0].x) as f64 / (vertices[2].y - vertices[0].y + 1) as f64,
        (vertices[1].x - vertices[0].x) as f64 / (vertices[1].y - vertices[0].y + 1) as f64,
        (vertices[2].x - vertices[1].x) as f64 / (vertices[2].y - vertices[1].y + 1) as f64,
    )
}

#[derive(Debug, PartialEq, Clone, Copy)]
struct Point {
    x: isize,
    y: isize,
}

/// This mutator mutates the given specimen by generating a random triangle with random background color.
#[derive(Debug, Default)]
pub struct TriangleMutator;

impl TriangleMutator {
    fn get_random_point(&self, random: &mut Random, image: &Image) -> Point {
        let x = random.get_random(0isize, image.width() as isize - 1);
        let y = random.get_random(0isize, image.height() as isize - 1);

        Point { x, y }
    }

    fn get_triangle_vertices(&self, random: &mut Random, image: &Image) -> [Point; 3] {
        let mut vertices = Vec::new();

        while vertices.len() != 3 {
            let point = self.get_random_point(random, image);

            if !vertices.contains(&point) {
                vertices.push(point);
            }
        }

        // The points must be sorted vertically. Required by the algorithm.
        vertices.sort_by(|p1, p2| p1.y.cmp(&p2.y));

        [vertices[0], vertices[1], vertices[2]]
    }

    fn draw_triangle<F>(&self, random: &mut Random, image: &mut Image, pixel_mutator: F)
    where
        F: Fn(&mut Pixel),
    {
        let vertices = self.get_triangle_vertices(random, image);

        let (dx_far, dx_upper, dx_low) = get_dx(&vertices);

        let image_width = image.width() as isize;
        let image_height = image.height() as isize;

        let mut xf = vertices[0].x as f64;
        let mut xt = vertices[0].x as f64 + dx_upper;

        let mut y = vertices[0].y;
        while y
            <= if vertices[2].y > image_height - 1 {
                image_height - 1
            } else {
                vertices[2].y
            }
        {
            if y >= 0 {
                let mut x = if xf > 0f64 { xf as isize } else { 0 };
                while x
                    <= if xt < image_width as f64 {
                        xt as isize
                    } else {
                        image_width - 1
                    }
                {
                    let idx = (y * image_width + x) as usize;
                    pixel_mutator(&mut image[idx]);

                    x += 1;
                }

                let mut x = if xf < image_width as f64 {
                    xf as isize
                } else {
                    image_width - 1
                };
                while x >= if xt > 0f64 { xt as isize } else { 0 } {
                    let idx = (y * image_width + x) as usize;
                    pixel_mutator(&mut image[idx]);

                    x -= 1;
                }
            }
            xf += dx_far;

            if y < vertices[1].y {
                xt += dx_upper;
            } else {
                xt += dx_low;
            }

            y += 1;
        }
    }
}

impl Mutator for TriangleMutator {
    fn mutate_rgb(&self, image: &mut Image) {
        let mut random = Random::default();

        let r = random.get_random(0u8, 255);
        let g = random.get_random(0u8, 255);
        let b = random.get_random(0u8, 255);

        let rgb_pixel_mutator = move |p: &mut Pixel| {
            p.r(r);
            p.g(g);
            p.b(b);
        };

        self.draw_triangle(&mut random, image, rgb_pixel_mutator);
    }

    fn mutate_grayscale(&self, image: &mut Image) {
        let mut random = Random::default();

        let grayscale = random.get_random(0u8, 255);

        let grayscale_pixel_mutator = move |p: &mut Pixel| p.set_grayscale(grayscale);

        self.draw_triangle(&mut random, image, grayscale_pixel_mutator);
    }
}
