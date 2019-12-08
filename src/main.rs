use std::fs::read_to_string;

fn main() {
    let image = Image::new(25, 6, read_to_string("input.txt").unwrap());

    let final_image = image.layers.iter().fold(None, |acc, layer| match acc {
        None => Some(layer.clone()),
        Some(image) => {
            let mut new_image = Vec::new();
            for (index, pixel) in image.iter().enumerate() {
                if *pixel == 2 {
                    new_image.push(layer[index]);
                } else {
                    new_image.push(*pixel);
                }
            }

            Some(new_image)
        }
    });

    let mut col_count = 0;
    for pixel in final_image.unwrap() {
        if pixel == 0 {
            print!(" ");
        } else {
            print!("\u{2588}");
        }
        col_count += 1;
        if col_count == 25 {
            println!();
            col_count = 0;
        }
    }
}

struct Image {
    layers: Vec<Vec<u32>>,
}

impl Image {
    fn new(width: u32, height: u32, input: String) -> Self {
        let mut layers = Vec::new();
        let mut current_layer = Vec::new();
        let layer_size = (width * height) as usize;
        for chr in input.chars() {
            current_layer.push(chr.to_string().parse().unwrap());

            if current_layer.len() == layer_size {
                layers.push(current_layer);
                current_layer = Vec::new();
            }
        }

        Image { layers }
    }
}
