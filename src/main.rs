use std::fs::{self, File};
use std::intrinsics::write_bytes;
use std::io::{IoSlice, Write};
use std::path::Path;
use std::path::PathBuf;
struct PPM {
    path: Box<Path>,
    width: usize,
    height: usize,
    image: Vec<u8>,
}

impl PPM {
    pub fn new(path: &Path, width: usize, height: usize) -> Self {
        let path = path.into();
        let image = vec![0; width * height * 3];
        Self {
            path,
            width,
            height,
            image,
        }
    }
    pub fn save(&self) -> Result<(), anyhow::Error> {
        let mut buf = File::create(self.path.as_ref())?;
        writeln!(
            buf,
            "P6
{} {}
255",
            self.width, self.height
        )?;

        buf.write_all(&self.image)?;
        Ok(())
    }

    /// x, y
    /// 0, 0 is top left corner
    /// increasing x means going right
    /// increasing y means going down
    pub fn set_pixel_unchecked(&mut self, point: &Point, colour: &Colour) {
        let Point(x, y) = *point;
        let x = x as usize;
        let y = y as usize;
        let index = x + y * self.width;
        let index = index * 3;
        let Colour(r, g, b) = *colour;

        self.image[index] = r;
        self.image[index + 1] = g;
        self.image[index + 2] = b;
    }

    pub fn draw_line(&mut self, start: &Point, end: &Point, colour: &Colour) {
        let Point(x0, y0) = *start;
        let Point(x1, y1) = *end;

        // Bresenham's Algorithm variables
        let mut x = x0;
        let mut y = y0;
        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            self.set_pixel_unchecked(&Point(x, y), colour);
            if x == x1 && y == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x += sx;
            }
            if e2 <= dx {
                err += dx;
                y += sy;
            }
        }
    }
}

struct Point(isize, isize);
struct Colour(u8, u8, u8);

fn main() -> Result<(), anyhow::Error> {
    let output_path = Path::new("output");
    if !output_path.exists() {
        fs::create_dir_all(output_path)?;
    }
    let mut f = PPM::new(&output_path.join("image.ppm"), 1920, 1080);
    for x in 0..1920 {
        for y in 0..1080 {
            let colour = Colour(0xFF, 0x00, 0x00);
            let point = Point(x, y);
            f.set_pixel_unchecked(&point, &colour);
        }
    }

    f.draw_line(&Point(69, 69), &Point(420, 420), &Colour(0x00, 0xFF, 0x00));
    f.save()?;
    Ok(())
}
