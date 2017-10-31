pub trait Grid {
    type Element: Clone;

    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn data(&self) -> &[Self::Element];
    fn data_mut(&mut self) -> &mut [Self::Element];


    fn num_pixels(&self) -> usize {
        (self.width() as usize) * (self.height() as usize)
    }
    fn dimensions(&self) -> (u32, u32) {
        (self.width(), self.height())
    }
}
