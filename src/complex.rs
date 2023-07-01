use std::fmt;

///Complex number (x + bi), i^2 = -1: <https://en.wikipedia.org/wiki/Complex_number>
pub struct Complex {
    /// Real part
    pub x: f64,
    /// Imaginary part
    pub y: f64,
}

impl Complex {
    pub fn new(x: f64, y: f64) -> Complex {
        Complex { x, y }
    }

    ///Addition
    pub fn add(&self, c: &Complex) -> Complex {
        let new_x = self.x + c.x;
        let new_y = self.y + c.y;
        Complex { x: new_x, y: new_y }
    }

    ///Subtraction, add the negative of c
    pub fn subtract(&self, c: &Complex) -> Complex {
        let negated_c = c.multiply_real(-1.0);
        self.add(&negated_c)
    }

    ///Multiply with a real
    pub fn multiply_real(&self, r: f64) -> Complex {
        let new_x = r * self.x;
        let new_y = r * self.y;
        Complex { x: new_x, y: new_y }
    }

    ///Multiply with a complex
    pub fn multiply(&self, c: &Complex) -> Complex {
        let new_x = self.x.mul_add(c.x, -self.y * c.y);
        let new_y = self.x.mul_add(c.y, self.y * c.x);
        Complex { x: new_x, y: new_y }
    }

    ///Square the complex
    pub fn squared(&self) -> Complex {
        let new_x = self.x.mul_add(self.x, -self.y * self.y);
        let new_y = 2.0 * self.x * self.y;
        Complex { x: new_x, y: new_y }
    }

    ///Calculate the absolute value of the complex (Pythagorean length of the complex, seen as x vector in the complex plane)
    pub fn abs(&self) -> f64 {
        f64::sqrt(self.x.mul_add(self.x, self.y * self.y))
    }
}

impl fmt::Debug for Complex {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{} {} {}i",
            self.x,
            if self.y > 0.0 { '+' } else { '-' },
            self.y.abs()
        )
    }
}
//Complex
