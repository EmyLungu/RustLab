use std::fmt::Display;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

#[derive(PartialEq, Debug, Clone, Copy)]
struct Complex {
    real: f64,
    imag: f64,
}
impl Complex {
    fn new<T, U>(real: T, imag: U) -> Self
    where
        f64: From<T>,
        f64: From<U>,
    {
        Complex {
            real: f64::from(real),
            imag: f64::from(imag),
        }
    }

    fn conjugate(&self) -> Self {
        Complex {
            real: self.real,
            imag: -self.imag,
        }
    }
}

impl<T> Add<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;

    fn add(self, rhs: T) -> Self::Output {
        let rhs = Complex::from(rhs);
        Complex {
            real: self.real + rhs.real,
            imag: self.imag + rhs.imag,
        }
    }
}

impl<T> AddAssign<T> for Complex
where
    Complex: From<T>,
{
    fn add_assign(&mut self, rhs: T) {
        let rhs = Complex::from(rhs);
        self.real += rhs.real;
        self.imag += rhs.imag;
    }
}

impl<T> Sub<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;

    fn sub(self, rhs: T) -> Self::Output {
        let rhs = Complex::from(rhs);
        Complex {
            real: self.real - rhs.real,
            imag: self.imag - rhs.imag,
        }
    }
}

impl<T> SubAssign<T> for Complex
where
    Complex: From<T>,
{
    fn sub_assign(&mut self, rhs: T) {
        let rhs = Complex::from(rhs);
        self.real -= rhs.real;
        self.imag -= rhs.imag;
    }
}

impl<T> Mul<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;

    fn mul(self, rhs: T) -> Self::Output {
        let rhs = Complex::from(rhs);
        Complex {
            real: self.real * rhs.real - self.imag * rhs.imag,
            imag: self.real * rhs.imag + self.imag * rhs.real,
        }
    }
}

impl<T> MulAssign<T> for Complex
where
    Complex: From<T>,
{
    fn mul_assign(&mut self, rhs: T) {
        let rhs = Complex::from(rhs);
        let real = self.real * rhs.real - self.imag * rhs.imag;
        let imag = self.real * rhs.imag + self.imag * rhs.real;
        self.real = real;
        self.imag = imag;
    }
}

impl<T> Div<T> for Complex
where
    Complex: From<T>,
{
    type Output = Complex;

    fn div(self, rhs: T) -> Self::Output {
        let rhs = Complex::from(rhs);
        let divisor = rhs.real.powi(2) + rhs.imag.powi(2);
        Complex {
            real: (self.real * rhs.real + self.imag * rhs.imag) / divisor,
            imag: (self.imag * rhs.real - self.real * rhs.imag) / divisor,
        }
    }
}

impl<T> DivAssign<T> for Complex
where
    Complex: From<T>,
{
    fn div_assign(&mut self, rhs: T) {
        let rhs = Complex::from(rhs);
        let divisor = rhs.real.powi(2) + rhs.imag.powi(2);
        let real = (self.real * rhs.real + self.imag * rhs.imag) / divisor;
        let imag = (self.imag * rhs.real - self.real * rhs.imag) / divisor;
        self.real = real;
        self.imag = imag;
    }
}

impl Neg for Complex {
    type Output = Complex;
    fn neg(self) -> Self::Output {
        Complex {
            real: -self.real,
            imag: -self.imag,
        }
    }
}

impl Display for Complex {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let is_real_zero = self.real == 0.0;
        let is_imag_zero = self.imag == 0.0;

        if is_real_zero && is_imag_zero {
            return write!(f, "0");
        }

        if is_real_zero {
            return write!(f, "{}i", self.imag);
        }

        if is_imag_zero {
            return write!(f, "{}", self.real);
        }

        let plus = if self.imag > 0.0 {
            String::from("+")
        } else {
            String::new()
        };

        write!(f, "{}{}{}i", self.real, plus, self.imag)
    }
}

impl From<i32> for Complex {
    fn from(value: i32) -> Self {
        Complex {
            real: value as f64,
            imag: 0.0,
        }
    }
}

impl From<f64> for Complex {
    fn from(value: f64) -> Self {
        Complex {
            real: value,
            imag: 0.0,
        }
    }
}

fn eq_rel(x: f64, y: f64) -> bool {
    (x - y).abs() < 0.001
}
macro_rules! assert_eq_rel {
    ($x:expr, $y: expr) => {
        let x = $x as f64;
        let y = $y as f64;
        let r = eq_rel(x, y);
        assert!(r, "{} != {}", x, y);
    };
}

fn main() {
    let a = Complex::new(1.0, 2.0);
    assert_eq_rel!(a.real, 1);
    assert_eq_rel!(a.imag, 2);

    let b = Complex::new(2.0, 3);
    let c = a + b;
    assert_eq_rel!(c.real, 3);
    assert_eq_rel!(c.imag, 5);

    let d = c - a;
    assert_eq!(b, d);

    let e = (a * d).conjugate();
    assert_eq_rel!(e.imag, -7);

    let f = (a + b - d) * c;
    assert_eq!(f, Complex::new(-7, 11));

    assert_eq!(Complex::new(1, 2).to_string(), "1+2i");
    assert_eq!(Complex::new(1, -2).to_string(), "1-2i");
    assert_eq!(Complex::new(0, 5).to_string(), "5i");
    assert_eq!(Complex::new(7, 0).to_string(), "7");
    assert_eq!(Complex::new(0, 0).to_string(), "0");

    let h = Complex::new(-4, -5);
    let i = h - (h + 5) * 2.0;
    assert_eq_rel!(i.real, -6);

    let j = -i + i;
    assert_eq_rel!(j.real, 0);
    assert_eq_rel!(j.imag, 0);

    // BONUS

    let mut k = Complex::new(3, -5);
    let l = Complex::new(6, 7);
    k += l;
    assert_eq_rel!(k.real, 9);
    assert_eq_rel!(k.imag, 2);

    let m = Complex::new(3, -9);
    k -= m;
    assert_eq_rel!(k.real, 6);
    assert_eq_rel!(k.imag, 11);

    let n = Complex::new(-3, 2);
    k *= n;
    assert_eq_rel!(k.real, -40);
    assert_eq_rel!(k.imag, -21);

    let mut o = Complex::new(-40, -20.0);
    let p = Complex::new(6, -2);
    o /= p;
    assert_eq_rel!(o.real, -5);
    assert_eq_rel!(o.imag, -5);

    println!("ok!");
}
