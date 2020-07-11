use image::Rgb;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn add(&mut self, other: &Color) {
        self.red += other.red;
        self.green += other.green;
        self.blue += other.blue;
    }

    pub fn clamp(&self) -> Color {
        Color {
            red: self.red.max(0.0).min(1.0),
            green: self.green.max(0.0).min(1.0),
            blue: self.blue.max(0.0).min(1.0),
        }
    }

    pub fn mult(&self, fact: f64) -> Color {
        assert!(fact >= 0.);
        Color {
            red: self.red * fact,
            green: self.green * fact,
            blue: self.blue * fact,
        }
    }

    pub fn mix(&self, other: &Color, fact: f64) -> Color {
        Color {
            red: (self.red * fact + other.red * (1.0 - fact)),
            green: (self.green * fact + other.green * (1.0 - fact)),
            blue: (self.blue * fact + other.blue * (1.0 - fact)),
        }
    }

    pub fn to_rgb(&self) -> Rgb<u8> {
        Rgb([
            (self.red * 255.) as u8,
            (self.green * 255.) as u8,
            (self.blue * 255.) as u8,
        ])
    }
}

#[derive(Debug,Copy,Clone)]
pub enum Material {
    Diffuse {
        diff_col: Color
    },
    Specular {
        spec_col: Color
    },
    Mixed { 
        diff_col: Color,
        spec_col: Color,
        spec_factor: f64,
    },
    Refractive {
        spec_col: Color,
        refr_col: Color,
        ior: f64,
    },
}

impl Material {
    pub fn diffuse(col : Color) -> Self {
        Material::Diffuse {
            diff_col: col,
        }
    }

    pub fn specular(col: Color) -> Self {
        Material::Specular {
            spec_col: col,
        }
    }

    pub fn mixed(dcol: Color, scol: Color, fact: f64) -> Self {
        Material::Mixed {
            diff_col: dcol,
            spec_col: scol,
            spec_factor: fact,
        }
    }

    pub fn refractive(scol: Color, rcol: Color, ior: f64) -> Self {
        Material::Refractive {
            spec_col: scol,
            refr_col: rcol,
            ior: ior,
        }
    }

}
