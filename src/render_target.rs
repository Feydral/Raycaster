pub mod termcanvas;

pub type Rgb = [u8; 3];

pub trait RenderTarget {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_pixel(&mut self, x: u32, y: u32, color: Rgb);
    fn sync_size(&mut self);
    fn present(&mut self);
}
