use ray_tracer::{format};
use std::io;


fn main() -> io::Result<()> {
    let content: [(u8, u8,u8); 6] = [
        (255, 0, 0),
        (0, 255, 0),
        (0, 0, 255),
        (255, 255, 0),
        (255, 255, 255),
        (0, 0, 0)
    ];
    format::Ppm::write("./fmt.ppm", 3, 2, &content)?;
    format::Ppm::test(256, 256)?;
    return Ok(());
}
