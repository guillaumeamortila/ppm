

/// Pixel structure
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Pixel {
	r: u8,
	g: u8,
	b: u8
}

/// Pixel functions
impl Pixel {
    /// Create a new Pixel
	pub fn new(red: u8, green: u8, blue: u8) -> Self {
		Pixel{r:red, g:green, b:blue}
	}

    /// Get the red color of the pixel
	pub fn red(&self) -> u8 {
		self.r
	}

    /// Get the green color of the pixel
	pub fn green(&self) -> u8 {
		self.g
	}

    /// Get the blue color of the pixel
	pub fn blue(&self) -> u8 {
		self.b
	}

    /// Get the pixel colors as a String
	pub fn display(&self) -> String{
		format!("{}, {}, {}", self.r, self.g, self.b)
	}

    /// Invert pixel colors
    pub fn invert(&self) -> Pixel {
        Pixel::new(!self.r, !self.g, !self.b)
    }

    /// Converts pixel to greyscale pixel
    pub fn greyscale(&self) -> Pixel {
        let grey = ((self.r as u16 + self.g as u16 + self.b as u16) / 3) as u8;
        Pixel::new(grey, grey, grey)
    }

    /// Check if two pixels are identical
    pub fn eq(self, other: Pixel) -> bool {
        self.r == other.r && self.g == other.g && self.b == other.b
    }

}



#[cfg(test)]
mod tests {

    use super::Pixel;

    fn get_test_pixel() -> Pixel {
    	Pixel::new(50,100,150)
    }

    #[test]
    fn test_red() {
    	assert_eq!(get_test_pixel().red(), 50)
    }

    #[test]
    fn test_green() {
    	assert_eq!(get_test_pixel().green(), 100)
    }

    #[test]
    fn test_blue() {
    	assert_eq!(get_test_pixel().blue(), 150)
    }

    #[test]
    fn test_display() {
    	assert_eq!(get_test_pixel().display(), "50, 100, 150")
    }

    #[test]
    fn test_invert() { // 255 - R, 255 - G, 255 - B
        let pix = get_test_pixel();
        assert_eq!(pix.invert().display(), "205, 155, 105")
    }

    #[test]
    fn test_greyscale() { // 255 - R, 255 - G, 255 - B
        let pix = get_test_pixel();
        assert_eq!(pix.greyscale().display(), "100, 100, 100")
    }

    #[test]
    fn test_eq() {
        assert!(get_test_pixel().eq(get_test_pixel()))
    }
}

