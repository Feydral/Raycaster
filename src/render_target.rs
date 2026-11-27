#[cfg(feature = "nanofb")]
pub mod nanofb;
#[cfg(feature = "termcanvas")]
pub mod termcanvas;

pub type Rgb = [u8; 3];

pub trait RenderTarget {
    fn width(&self) -> u32;
    fn height(&self) -> u32;
    fn set_pixel(&mut self, x: u32, y: u32, color: Rgb);
    fn sync_size(&mut self);
    fn present(&mut self);
}

pub fn quantize(color: Rgb, levels: u32) -> Rgb {
    let brightest = color[0].max(color[1]).max(color[2]);

    if brightest == 0 {
        return color;
    }

    let steps = (levels - 1) as f32;
    let level = (brightest as f32 / 255.0 * steps).round();
    let scale = level / steps * 255.0 / brightest as f32;

    color.map(|channel| (channel as f32 * scale).round().min(255.0) as u8)
}
