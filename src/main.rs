extern crate ppm;

use std::path::Path;


fn main(){

	/*** INPUTS ************/

    // NEW IMAGE
    let mut image = ppm::image::Image::new_with_file(Path::new("src/img_samples/test-s.ppm")).unwrap();

    // NEW IMAGE BIN
    // let image = ppm::image::Image::new_with_file_bin(Path::new("src/img_samples/Clown_bin_256.ppm")).unwrap();


    /*** TRANSFORMATIONS ***/
    
    image.invert();
    // image.greyscale();


    /*** OUTPUTS ***********/

    println!("{:?}\n\n", image);

    // SAVE IMG
    image.save(Path::new("src/img_dst/test_result.ppm"));

    // SAVE IMG BIN
    // image.save_bin(Path::new("src/img_dst/test_result-bin.ppm"));


}