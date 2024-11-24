use crate::helpers::types::vec3;

pub struct ImageSpace {
    pub orig: vec3,
    pub delta: f64,
    pub xdir: vec3,
    pub ydir: vec3,
}

impl ImageSpace {
    /// ### Maps image space coords to world coords
    pub fn pixel_center_at(&self, ix: u32, iy: u32) -> vec3 {
        self.orig + self.pixel_offset(ix as f64, iy as f64)
    }

    /// ### Maps image space coords to world coords
    pub fn pixel_lefttop_at(&self, ix: u32, iy: u32) -> vec3 {
        self.orig + self.pixel_offset(ix as f64 - 0.5, iy as f64 - 0.5)
    }

    #[inline]
    // ### Pixel offset of given number of pixels in world coords
    pub fn pixel_offset(&self, ndx: f64, ndy: f64) -> vec3 {
        (self.xdir * self.delta * ndx) + (self.ydir * self.delta * ndy)
    }
}
