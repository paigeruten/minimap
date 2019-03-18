const MARGIN: u32 = 2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1)
        .expect("Usage: minimap <path>");

    let code = std::fs::read_to_string(&path)?;

    let lines = code.lines()
        .map(|line| line.chars().collect::<Vec<char>>())
        .collect::<Vec<Vec<char>>>();

    let width = lines.iter().map(|line| line.len()).max().unwrap() as u32 + MARGIN*2;
    let height = lines.len() as u32 * 3 + MARGIN*2;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for pixel in imgbuf.pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    for (y, line) in lines.iter().enumerate() {
        let y = y as u32;
        for (x, ch) in line.iter().enumerate() {
            let x = x as u32;
            if !ch.is_whitespace() {
                imgbuf.put_pixel(x + MARGIN, y * 3 + MARGIN, image::Rgb([0, 0, 0]));
                imgbuf.put_pixel(x + MARGIN, y * 3 + 1 + MARGIN, image::Rgb([0, 0, 0]));
            }
        }
    }

    imgbuf.save("minimap.png").unwrap();
    println!("Minimap written to minimap.png");

    Ok(())
}
