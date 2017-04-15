#[derive(Copy, Clone)]
pub struct Complex {
    pub real: f64,
    pub i: f64
}

impl Complex {
    pub fn add(&self, addend: Complex) -> Complex {
        Complex {
            real: &self.real + addend.real,
            i: &self.i + addend.i
        }
    }

    pub fn subtract(&self, subtrahend: Complex) -> Complex {
        Complex {
            real: &self.real - subtrahend.real,
            i: &self.i - subtrahend.i
        }
    }

    pub fn multiply(&self, multiplier: Complex) -> Complex {
        Complex {
            real: &self.real * multiplier.real - &self.i * multiplier.i,
            i: &self.i * multiplier.real + &self.real * multiplier.i
        }
    }

    pub fn divide(&self, dividend: Complex) -> Complex {
        Complex {
            real: (&self.real * dividend.real + &self.i * dividend.i) / (dividend.real * dividend.real + dividend.i * dividend.i),
            i: (&self.i * dividend.real - &self.real * dividend.i) / (dividend.real * dividend.real + dividend.i * dividend.i)
        }
    }
}
