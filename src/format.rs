use std::fs::File;
use std::io;
use std::io::prelude::*;

/* Ppm: Unit-like struct upon which associated functions are implemented for the PPM (Portable
* Pixmap) file format */
pub struct Ppm{}

impl Ppm {
    /* output a test ppm file */
    pub fn test(width: u32, height: u32) -> io::Result<()> {
        File::options().append(true);
        let mut f = File::create("./test.ppm")?;
        f.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;
        for i in 0..height {
            for j in 0..width {
                let r: f32 = i as f32 / (width - 1) as f32;
                let g: f32 = j as f32 / (height - 1) as f32;
                let b: f32 = 0.0;

                const SCALE_FACTOR: u8 = 255;
                let ir = (r * SCALE_FACTOR as f32) as u32; 
                let ig = (g * SCALE_FACTOR as f32) as u32;
                let ib = (b * SCALE_FACTOR as f32) as u32;

                f.write_all(format!("{} {} {}\n", ir, ig, ib).as_bytes())?;
            }
        }
        Ok(())
    }

    pub fn write(path: &str, width: u32, height: u32, content: &[(u8, u8, u8)]) -> io::Result<()> {
        File::options().append(true);
        let mut f = File::create(path)?;
        f.write_all(format!("P3\n{} {}\n255\n", width, height).as_bytes())?;
        for i in 0..height {
            for j in 0..width {
                let (r,g,b) = content[(i*width + j) as usize];
                f.write_all(format!("{} {} {}\n", r,g,b).as_bytes())?;
            }
            f.write(b"\n")?;
        }
        Ok(())
    }
}
