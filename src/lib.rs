#[derive(Debug, Clone, Default, Copy, PartialEq)]
pub struct Color32f {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl From<[f32; 3]> for Color32f {
    fn from(xs: [f32; 3]) -> Self {
        Self {
            r: xs[0],
            g: xs[1],
            b: xs[2],
        }
    }
}

impl Into<[f32; 3]> for Color32f {
    fn into(self) -> [f32; 3] {
        [self.r, self.g, self.b]
    }
}

#[derive(Debug, Clone, Default, Copy, PartialEq)]
pub struct Color8u {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl Color8u {
    pub fn from_normalized(xs: impl Into<[f32; 3]>) -> Self {
        let xs = xs.into();

        Self {
            r: (255.9 * xs[0]) as u8,
            g: (255.9 * xs[1]) as u8,
            b: (255.9 * xs[2]) as u8,
        }
    }
}
