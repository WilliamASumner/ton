use image::Rgb;

#[derive(Debug, Copy, Clone)]
pub struct Color {
    red: f64,
    green: f64,
    blue: f64,
}

impl Color {
    pub fn new(r: f64, g: f64, b: f64) -> Color {
        Color {
            red: r,
            green: g,
            blue: b,
        }
    }

    pub fn r_val(&self) -> f64 {
        self.red
    }

    pub fn g_val(&self) -> f64 {
        self.green
    }

    pub fn b_val(&self) -> f64 {
        self.blue
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
pub enum MaterialType {
    Diffuse,
    Specular,
    DiffSpec,
    Refractive,
}

#[derive(Debug,Copy,Clone)]
pub struct Material {
    pub diff_col: Color,
    pub spec_col: Color,
    pub specularity: f64,
    pub mat_type: MaterialType,
}

impl Material {
    pub fn new(diffuse: Color, spec: Color, shine: f64,mat: MaterialType) -> Material {
        Material {
            diff_col: diffuse,
            spec_col: spec,
            specularity: shine,
            mat_type: mat,
        }
    }
}

// Helpful colors
#[allow(dead_code)]
pub const RED: Color = Color {
    red: 1.,
    green: 0.,
    blue: 0.,
};
#[allow(dead_code)]
pub const GREEN: Color = Color {
    red: 0.,
    green: 1.,
    blue: 0.,
};
#[allow(dead_code)]
pub const BLUE: Color = Color {
    red: 0.,
    green: 0.,
    blue: 1.,
};
#[allow(dead_code)]
pub const BLACK: Color = Color {
    red: 0.,
    green: 0.,
    blue: 0.,
};
#[allow(dead_code)]
pub const GREY: Color = Color {
    red: 0.5,
    green: 0.5,
    blue: 0.5,
};
#[allow(dead_code)]
pub const WHITE: Color = Color {
    red: 1.,
    green: 1.,
    blue: 1.,
};
