use std::fmt;


///Complex number (a + bi), i^2 = -1: https://en.wikipedia.org/wiki/Complex_number
pub struct Complex {
    /// Real part
    a: f64,
    /// Imaginary part
    b: f64,
}

impl Complex {
    pub fn new(a: f64, b: f64) -> Complex {
        Complex { a, b }
    }

    ///Addition
    pub fn add(&self, c: &Complex) -> Complex {
        let new_a = self.a + c.a;
        let new_b = self.b + c.b;
        Complex { a: new_a, b: new_b }
    }

    ///Subtraction, add the negative of c
    pub fn subtract(&self, c: &Complex) -> Complex {
        let negated_c = c.multiply_real(-1.0);
        self.add(&negated_c)
    }

    ///Multiply with a real
    pub fn multiply_real(&self, r: f64) -> Complex {
        let new_a = r * self.a;
        let new_b = r * self.b;
        Complex { a: new_a, b: new_b }
    }

    ///Multiply with a complex
    pub fn multiply(&self, c: &Complex) -> Complex {
        let new_a = self.a * c.a - self.b * c.b;
        let new_b = self.a * c.b + self.b * c.a;
        Complex { a: new_a, b: new_b }
    }

    ///Square the complex
    pub fn squared(&self) -> Complex {
        let new_a = self.a * self.a - self.b * self.b;
        let new_b = 2.0 * self.a * self.b;
        Complex { a: new_a, b: new_b }
    }

    ///Calculate the absolute value of the complex (Pythagorean length of the complex, seen as a vector in the complex plane)
    pub fn abs(&self) -> f64 {
        f64::sqrt(self.a * self.a + self.b * self.b)
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} {} {}i", self.a, if self.b > 0.0 {'+'} else {'-'}, self.b.abs())
    }
}
//Complex