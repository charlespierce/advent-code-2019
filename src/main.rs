use std::fs::read_to_string;

fn main() {
    let image = Image::new(25, 6, read_to_string("input.txt").unwrap());

    let mut min_zero_count = 151;
    let mut target_one_count = 0;
    let mut target_two_count = 0;
    for layer in image.layers {
        let mut zero_count = 0;
        let mut one_count = 0;
        let mut two_count = 0;
        for val in layer {
            match val {
                0 => {
                    zero_count += 1;
                }
                1 => {
                    one_count += 1;
                }
                2 => {
                    two_count += 1;
                }
                _ => {}
            }
        }

        if zero_count < min_zero_count {
            min_zero_count = zero_count;
            target_one_count = one_count;
            target_two_count = two_count;
        }
    }

    println!("Count of Ones: {}", target_one_count);
    println!("Count of Twos: {}", target_two_count);
    println!("Product: {}", target_one_count * target_two_count);
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
