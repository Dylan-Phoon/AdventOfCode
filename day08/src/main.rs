use::std::fs;

struct Image {
	width: usize,
	height: usize
}

fn main() {
	let image_str  = fs::read_to_string("input.txt").expect("could not read input.txt");
	let mut image = image_str.split("").map(|str_num| str_num.parse::<u8>().unwrap_or(0)).collect::<Vec<u8>>();
	image.remove(0);
	image.pop();
	let res = part_two(&image, Image{ width: 25, height: 6 });
	println!("{:?}",res );
}

/// verbose solution so its easy to solve the second part
fn part_one(image: &Vec<u8>, image_info: Image) -> u32 {
	//loop through image array
	//Push each item into a row vector
	//If the index % width then we have a row
	//Take this row and push it into the current layer array
	//If the layer array length % height == 0 then push it into the image array
	let mut curr_row: Vec<u8> = Vec::new();
	let mut curr_layer: Vec<Vec<u8>> = Vec::new();
	let mut image_arr: Vec<Vec<Vec<u8>>> = Vec::new();
	for i in 0..image.len() {

		if curr_layer.len() % image_info.height == 0 && curr_layer.len() > 0 { //We have an image
			image_arr.push(curr_layer.clone());
			println!("{:?}", curr_layer);
			curr_layer.clear();
		}

		if i % image_info.width == 0 && i > 0 { //We have a row
			curr_layer.push(curr_row.clone());
			curr_row.clear();
		}
		curr_row.push(image[i as usize]);
	}

	//println!("{:?}", image_arr);

	//Loop through the image and count all of the zeroes and ones
	let mut layer_with_least_zeroes: Vec<Vec<u8>> = Vec::new(); // [[]] //layer[rows[]]
	let mut least_zeroes = 0;
	for layer in image_arr {
		let mut curr_zeroes = 0;
		for row in &layer {
			for byte in row {
				if byte == &0 { curr_zeroes += 1; }
			}
		}

		if curr_zeroes < least_zeroes || least_zeroes == 0 {
			least_zeroes = curr_zeroes;
			layer_with_least_zeroes = layer;
		}
	}

	let mut number_of_ones = 0;
	let mut number_of_twos = 0;
	for row in layer_with_least_zeroes {
		for byte in row {
			if byte == 1 {
				number_of_ones += 1;
			} else if byte == 2 {
				number_of_twos += 1;
			}
		}
	}

	return number_of_ones * number_of_twos;

}

const BLACK: u8 = 0;
const WHITE: u8 = 1;
const TRNS: u8 = 2;

fn part_two(image: &Vec<u8>, image_info: Image) -> u32 {
	/// Take the given image and transform it into below;
	/// image[layers[rows[pixels]]]
	let mut curr_row: Vec<u8> = Vec::new();
	let mut curr_layer: Vec<Vec<u8>> = Vec::new();
	let mut image_arr: Vec<Vec<Vec<u8>>> = Vec::new();
	for i in 0..=image.len() {
		if i % image_info.width == 0 && i > 0 { //We have a row
			curr_layer.push(curr_row.clone());
			curr_row.clear();
		}


		if curr_layer.len() % image_info.height == 0 && curr_layer.len() > 0 { //We have a layer
			image_arr.push(curr_layer.clone());
			curr_layer.clear();
		}

		if i < image.len() { //We have a pixel
			curr_row.push(image[i as usize]);
		}
	}

	//Loop through the image and decode it
	//image[rows[bytes]]
	//create an empty image of all TRNS type as the first layer
	let mut decoded_image: Vec<Vec<u8>> = vec![vec![TRNS; image_info.width]; image_info.height];
	for layer in image_arr {
		for row_i in 0..layer.len() as usize {
			for byte_i in 0..layer[row_i].len() as usize {
				let curr_pixel = layer[row_i][byte_i];
				let mut decoded_image_pixel = decoded_image[row_i][byte_i];
				if decoded_image_pixel == TRNS {
					decoded_image[row_i][byte_i] = curr_pixel;
				}
			}
		}
	}

	for row in decoded_image {
		for byte in row {
			if byte == WHITE {
				print!("{:?}", byte);
			} else {
				print!(" ")
			}
		}
		print!("\n");
	}
	return 0;
}