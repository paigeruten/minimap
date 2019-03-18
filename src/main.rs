use std::io::prelude::*;
use syntect::easy::HighlightFile;
use syntect::highlighting::ThemeSet;
use syntect::parsing::SyntaxSet;

const MARGIN: u32 = 2;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let path = std::env::args().nth(1)
        .expect("Usage: minimap <path>");

    let ss = SyntaxSet::load_defaults_nonewlines();
    let ts = ThemeSet::load_defaults();
    let mut highlighter = HighlightFile::new(&path, &ss, &ts.themes["base16-mocha.dark"]).unwrap();

    let mut line_regions = Vec::new();
    for line in highlighter.reader.lines() {
        let line = line.unwrap();
        let regions = highlighter.highlight_lines.highlight(&line, &ss)
            .iter()
            .map(|&(style, region)| (style, String::from(region)))
            .collect::<Vec<_>>();
        line_regions.push(regions);
    }

    let width = line_regions.iter()
        .map(|regions| {
            regions.iter()
                .map(|(_, line)| line.len())
                .sum::<usize>()
        })
        .max()
        .unwrap() as u32 + MARGIN*2;
    let height = line_regions.len() as u32 * 3 + MARGIN*2;

    let mut imgbuf = image::ImageBuffer::new(width, height);

    for pixel in imgbuf.pixels_mut() {
        *pixel = image::Rgb([255, 255, 255]);
    }

    for (y, regions) in line_regions.iter().enumerate() {
        let y = y as u32;
        let mut x = 0u32;
        for (style, region) in regions.iter() {
            for ch in region.chars() {
                if !ch.is_whitespace() {
                    let color = image::Rgb([style.foreground.r, style.foreground.g, style.foreground.b]);
                    imgbuf.put_pixel(x + MARGIN, y * 3 + MARGIN, color);
                    imgbuf.put_pixel(x + MARGIN, y * 3 + 1 + MARGIN, color);
                }
                x += 1;
            }
        }
    }

    imgbuf.save("minimap.png").unwrap();
    println!("Minimap written to minimap.png");

    Ok(())
}
