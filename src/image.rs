use crate::pixel::Pixel;
use std::path::Path;
use std::io::prelude::*;
use std::io::{BufReader, BufWriter};
use std::fs::File;

// extern crate test;


/// Image structure
#[derive(Debug)]
pub struct Image {
    pixels: Vec<Pixel>,
    height: usize,
    width: usize,
    nb_colors: usize,
    ppm_type: String
}


impl Image {

    /// Instantiate an Image with initial values
	pub fn new_empty() -> Self{
		Image{pixels: Vec::new(), width: 0, height: 0, nb_colors: 255, ppm_type: "p3".to_string()}
	}

    /// type setter
	pub fn set_type(&mut self, ppm_type: String) {
		self.ppm_type = ppm_type;
	}

    /// width setter
	pub fn set_width(&mut self, width: usize) {
		self.width = width;
	}

    /// height setter
	pub fn set_height(&mut self, height: usize) {
		self.height = height;
	}

    /// nb_colors setter
	pub fn set_nb_colors(&mut self, nb_colors: usize) {
		self.nb_colors = nb_colors;
	}

    /// pixels setter
	pub fn set_pixels(&mut self, pixels: Vec<Pixel>) {
		self.pixels = pixels;
	}

    /// Get a rgb pixel from a ppm pixel
    fn get_pixel_from_str(ppm_pix: Option<&str>) -> Result<u8, String> {
        match ppm_pix {
            Some(value) => match value.parse::<u8>() {
                Ok(value) => Ok(value),
                Err(_err) => Err(String::from("Fichier ppm non conforme"))
            },
            None => Err(String::from("Fichier ppm non conforme"))
        }
    }


    /// Invert image pixels
    pub fn invert(&mut self) {
        let mut pixels: Vec<Pixel> = Vec::new();
        for pixel in &self.pixels {
            pixels.push(pixel.invert());
        }
        self.set_pixels(pixels);
    }

    /// Convert image pixels to greyscale
    pub fn greyscale(&mut self) {
        let mut pixels: Vec<Pixel> = Vec::new();
        for pixel in &mut self.pixels {
            pixels.push(pixel.greyscale());
        }
        self.set_pixels(pixels);
    }



    /// Create a new Image from a ppm file
    pub fn new_with_file(filename: &Path) -> Result<Self, String> {

    	// let file = File::open(filename).expect("Impossible d'ouvrir le fichier");
        let file = match File::open(filename){
            Ok(file) => file,
            Err(_e) => return Err(String::from("Impossible d'ouvrir le fichier")),
        };

        let mut file_buffer = BufReader::new(file);

        let mut pixel_vec: Vec<Pixel> = Vec::new();


        let mut img = Image::new_empty();

        img.read_header(&mut file_buffer)?;


        // Reading ppm's pixel data

        let mut line = String::from("");
        
        loop {
            let _read_size = match file_buffer.read_line(&mut line) {
                Ok(len) => {
                    if len == 0 {
                        break;
                    }
                    len
                },
                Err(_e) => return Err(String::from("Fichier ppm non conforme"))
            };

            // Remove comment from the line
            if line.contains("#") {
	            line = line.split("#").take(1).collect();
	        	if line.len() == 0 {
	                line.clear();
	            	continue;
	            }
	        }

            let mut pixel = line.split_ascii_whitespace();

            while let Some(line_pixel) = pixel.next() {

                let r = Self::get_pixel_from_str(Some(line_pixel))?;
                let g = Self::get_pixel_from_str(pixel.next())?;
                let b = Self::get_pixel_from_str(pixel.next())?;

                pixel_vec.push(Pixel::new(r, g, b));
            }

            line.clear();
        }
        
        if pixel_vec.len() != img.width * img.height {
            return Err(String::from(format!("Fichier ppm non conforme, le nombre de pixels ne respecte pas les dimensions {}, {}, {}, {}, {}", pixel_vec.len(), img.ppm_type, img.width, img.height, img.nb_colors)))
        }

        img.set_pixels(pixel_vec);

        Ok(img)
    }



    /// Create a new Image from a binary encoded ppm file
    pub fn new_with_file_bin(filename: &Path) -> Result<Self, String> {

    	// let file = File::open(filename)?;
        let file = match File::open(filename){
            Ok(file) => file,
            Err(_e) => return Err(String::from("Can't open the file")),
        };

        let mut line_vec = Vec::<u8>::new();
        let mut file_buffer = BufReader::new(file);
        
        let mut pixel_vec: Vec<Pixel> = Vec::new();


        let mut img = Image::new_empty();

        img.read_header(&mut file_buffer)?;


        // Reading ppm's pixel binary data

        loop {

            match file_buffer.read_until(255, &mut line_vec) {
                Ok(len) => {
                    if len == 0 {
                        break;
                    }
                    len
                },
                Err(_e) =>  return Err(String::from("Fichier ppm non conforme"))
            };

            // Remove comment from the line
            // get index of ["#"] and delete all the followings
            // ...

            while line_vec.len() >= 3 {

                let r = line_vec.remove(0);
                let g = line_vec.remove(0);
                let b = line_vec.remove(0);

                pixel_vec.push(Pixel::new(r, g, b));
            }
        }

        if pixel_vec.len() != img.width * img.height {
            return Err(String::from(format!("Fichier ppm non conforme, le nombre de pixels ne respecte pas les dimensions {}, {}, {}, {}, {}", pixel_vec.len(), img.ppm_type, img.width, img.height, img.nb_colors)))
        }

        img.set_pixels(pixel_vec);

        Ok(img)
    }


    /// Reading the ppm's header's data
    fn read_header(&mut self, buffer: &mut BufReader<File>) -> Result<(), String>{

        let mut line = String::from("");
        
        let mut n = 1;

        while n < 5 {

        	// let len = buffer.read_line(&mut line)?;
            match buffer.read_line(&mut line) {
                Ok(len) => {
                    if len == 0 {
                        return Err(String::from("Fichier ppm non conforme"));
                    }
                },
                Err(_e) => return Err(String::from("Fichier ppm non conforme"))
            };

            // Remove comment from the line
            if line.contains("#") {
	            line = line.split("#").take(1).collect();
	        	if line.len() == 0 {
	                line.clear();
	            	continue;
	            }
	        }

            for info in line.split_ascii_whitespace() {

                match n {
                	1 => self.set_type(info.to_string()),
                	2 => self.set_width(info.parse().unwrap()),
                	3 => self.set_height(info.parse().unwrap()),
                	4 => self.set_nb_colors(info.parse().unwrap()),
                	_ => return Err(String::from("Fichier ppm non conforme")),
                }

                n += 1;

            }

            line.clear();
        }

        Ok(())
    }


    /// Save an Image as ppn P3
    pub fn save(&self, filename: &Path) -> std::io::Result<()> {

        let file = File::create(filename)?;
        let mut file_buffer = BufWriter::new(file);

        file_buffer.write(format!("P3\n{} {}\n{}\n", self.width, self.height, self.nb_colors).as_bytes())?;

        let mut cur_width = 0;
        let mut cur_line_size = 0;

        for pixel in &self.pixels {
            // New line
            if cur_width == self.width || cur_line_size > 70 {
                file_buffer.write(b"\n")?;
                cur_width = 0;
                cur_line_size = 0;
            }

            file_buffer.write(format!("{} {} {}  ", pixel.red(), pixel.green(), pixel.blue()).as_bytes())?;

            cur_width += 1;
            cur_line_size += 7;
        }

        file_buffer.flush()
    }

    /// Save an Image as binary ppn P6
    pub fn save_bin(&self, filename: &Path) -> std::io::Result<()> {

        let file = File::create(filename)?;
        let mut file_buffer = BufWriter::new(file);

        file_buffer.write(format!("P6\n{} {}\n{}\n", self.width, self.height, self.nb_colors).as_bytes())?;

        for pixel in &self.pixels {
            file_buffer.write(&[pixel.red(), pixel.green(), pixel.blue()])?;
        }

        file_buffer.flush()
    }

}





/*******************************
****** TESTS & BENCHMARK *******
*******************************/


// ERROR lors du chargement de la crate test : use of unstable library feature 'test'
// Impossible de trouver une alternative malgré nos recherches...
// #![feature(test)]
// extern crate test;




#[cfg(test)]
mod tests {

	// use super::*;
    // use test::Bencher;


    // PROBLEM : new_with_file return type is Result<Image,...> and not Image

    // fn get_test_image() -> Image {
    //     Image::new_with_file(Path::new("src/img_samples/test-xs.ppm"))
    // }


    // fn get_test_image_bin() -> Image {
    //     Image::new_with_file_bin(Path::new("src/img_samples/test-xs_bin.ppm"))
    // }

	// #[test]
	// fn test_image_invert(){
	//     let mut image = get_test_image();
	//     image.invert();
	//     let pixel = image.pixels.pop().unwrap();
	//     assert_eq!(pixel.display(), "205, 155, 105");
	// }

	// #[test]
	// fn test_image_greyscale(){
	//     let mut image = get_test_image();
	//     image.greyscale();
	//     let pixel = image.pixels.pop().unwrap();
    //      assert_eq!(pixel.display(), "100, 100, 100");
	// }

    // #[test]
    // fn test_image_p3_to_p6(){
    //     let img = get_test_image();
    // }



    // ERREUR de chargement de la crate test
    // #[bench]
    // fn bench_new_with_file(b: &mut Bencher) {
    //     b.iter(|| Image::new_with_file("src/img_samples/test.ppm"));
    // }

}
