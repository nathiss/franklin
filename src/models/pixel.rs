#[derive(Debug, Clone, PartialEq, Default)]
pub struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

impl Pixel {
    pub const fn white() -> Self {
        Pixel::new(255, 255, 255)
    }

    pub const fn grayscale(grayscale: u8) -> Self {
        Pixel::new(grayscale, grayscale, grayscale)
    }

    pub const fn new(r: u8, g: u8, b: u8) -> Self {
        Pixel { r, g, b }
    }

    pub fn r(&mut self, r: u8) {
        self.r = r;
    }

    pub fn g(&mut self, g: u8) {
        self.g = g;
    }

    pub fn b(&mut self, b: u8) {
        self.b = b;
    }

    pub fn get_r(&self) -> u8 {
        self.r
    }

    pub fn get_g(&self) -> u8 {
        self.g
    }

    pub fn get_b(&self) -> u8 {
        self.b
    }

    pub fn set_grayscale(&mut self, grayscale: u8) {
        self.r(grayscale);
        self.g(grayscale);
        self.b(grayscale);
    }

    pub fn as_slice(&self) -> [u8; 3] {
        [self.r, self.g, self.b]
    }
}

#[cfg(test)]
#[allow(non_snake_case)]
mod test {
    use super::*;

    #[test]
    fn white_createAnInstance_allFieldsSetToMaxValue() {
        let white = Pixel::white();

        assert_eq!(255, white.get_r());
        assert_eq!(255, white.get_g());
        assert_eq!(255, white.get_b());
    }

    #[test]
    fn grayscale_withCustomValue_allFieldsSetToCorrectValue() {
        let grayscale = Pixel::grayscale(127);

        assert_eq!(127, grayscale.get_r());
        assert_eq!(127, grayscale.get_g());
        assert_eq!(127, grayscale.get_b());
    }

    #[test]
    fn new_withCustomValues_allFieldsSetToCorrectValues() {
        let pixel = Pixel::new(125, 126, 127);

        assert_eq!(125, pixel.get_r());
        assert_eq!(126, pixel.get_g());
        assert_eq!(127, pixel.get_b());
    }

    #[test]
    fn r_withDifferentValue_valueOfRedChangedAccordingly() {
        let mut pixel = Pixel::new(125, 126, 127);

        pixel.r(128);

        assert_eq!(128, pixel.get_r());
    }

    #[test]
    fn g_withDifferentValue_valueOfGreenChangedAccordingly() {
        let mut pixel = Pixel::new(125, 126, 127);

        pixel.g(128);

        assert_eq!(128, pixel.get_g());
    }

    #[test]
    fn b_withDifferentValue_valueOfBlueChangedAccordingly() {
        let mut pixel = Pixel::new(125, 126, 127);

        pixel.b(128);

        assert_eq!(128, pixel.get_b());
    }

    #[test]
    fn set_grayscale_withDifferentValue_allFieldsSetToCorrectValues() {
        let mut pixel = Pixel::new(125, 126, 127);

        pixel.set_grayscale(128);

        assert_eq!(128, pixel.get_r());
        assert_eq!(128, pixel.get_g());
        assert_eq!(128, pixel.get_b());
    }

    #[test]
    fn as_slice_fieldsWithDifferentValues_returnsSliceWithCorrectValues() {
        let pixel = Pixel::new(125, 126, 127);

        let slice = pixel.as_slice();

        assert_eq!([125, 126, 127], slice);
    }
}
