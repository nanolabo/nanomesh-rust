use std::ops::*;
use std::fmt::*;

#[derive(Debug, Copy, Clone)]
pub struct DVec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl DVec3
{
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return DVec3 { 
            x: x,
            y: y,
            z: z
        }
    }

    pub fn new2d(x: f64, y: f64) -> Self {
        return DVec3 { 
            x: x,
            y: y,
            z: 0.
        }
    }

    pub fn magnitude(&self) -> f64 {
        return (self * self).sqrt();
    }

    pub fn normalized(&self) -> DVec3
    {
        let magnitude = self.magnitude();
        return self / magnitude;
    }

    pub fn distance_to_line(&self, a: &DVec3, b: &DVec3, ) -> f64
    {
        let dir = &(a - b).normalized();
        let distance = (dir ^ &(self - a)).magnitude();
        return distance;
    }
}

impl Add<&DVec3> for &DVec3 {
    type Output = DVec3;
    fn add(self, _rhs: &DVec3) -> DVec3 {
        return DVec3 { 
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl Sub<&DVec3> for &DVec3 {
    type Output = DVec3;
    fn sub(self, _rhs: &DVec3) -> DVec3 {
        return DVec3 { 
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z
        }
    }
}

impl Neg for &DVec3 {
    type Output = DVec3;
    fn neg(self) -> DVec3 {
        return DVec3 { 
            x: -self.x,
            y: -self.y,
            z: -self.z
        }
    }
}

impl Mul<f64> for &DVec3 {
    type Output = DVec3;
    fn mul(self, _rhs: f64) -> DVec3 {
        return DVec3 { 
            x: self.x * _rhs,
            y: self.y * _rhs,
            z: self.z * _rhs,
        }
    }
}

impl Div<f64> for &DVec3 {
    type Output = DVec3;
    fn div(self, _rhs: f64) -> DVec3 {
        return DVec3 { 
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

// Implementation of dot product
impl Mul<&DVec3> for &DVec3 {
    type Output = f64;
    fn mul(self, _rhs: &DVec3) -> f64 {
        return self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z;
    }
}

// Implementation of cross product
impl BitXor<&DVec3> for &DVec3 {
    type Output = DVec3;
    fn bitxor(self, _rhs: &DVec3) -> DVec3 {
        return DVec3 {
            x: self.y * _rhs.z - self.z * _rhs.y,
            y: self.z * _rhs.x - self.x * _rhs.z,
            z: self.x * _rhs.y - self.y * _rhs.x
        }
    }
}

impl PartialEq for DVec3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

impl Default for DVec3 {
    fn default() -> Self {
        DVec3 { x: 0., y: 0., z: 0. }
    }
}

impl Display for DVec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<x:{} y:{} z:{}>", self.x, self.y, self.z)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::*;

    #[test]
    fn add() {
        let a = DVec3 { x: 1., y: 2., z: 3. };
        let b = DVec3 { x: 4., y: 5., z: 6. };
        let c = DVec3 { x: 5., y: 7., z: 9. };
        assert_eq!(&a + &b, c);
        assert_ne!(&a + &b, a);
    }

    #[test]
    fn substract() {
        let a = DVec3 { x: 1., y: 2., z: 3. };
        let b = DVec3 { x: 4., y: 5., z: 6. };
        let c = DVec3 { x: -3., y: -3., z: -3. };
        assert_eq!(&a - &b, c);
        assert_ne!(&a - &b, a);
    }

    #[test]
    fn divide() {
        let a = DVec3 { x: 1., y: 2., z: -3. };
        let b = 2.;
        let c = DVec3 { x: 0.5, y: 1., z: -1.5 };
        assert_eq!(&a / b, c);
        assert_ne!(&a / b, a);
    }

    #[test]
    fn dot() {
        let a = DVec3 { x: 1., y: 2., z: -3. };
        let b = DVec3 { x: 4., y: -5., z: 6. };
        let c = 4. - 10. - 18.;
        assert_eq!(&a * &b, c);
        assert_ne!(&a * &b, 0.);
    }

    #[test]
    fn cross() {
        let a = DVec3 { x: 1., y: 2., z: -3. };
        let b = DVec3 { x: 4., y: -5., z: 6. };
        let c = DVec3 { x: -3.0, y: -18.0, z: -13.0 };
        assert_eq!(&a ^ &b, c);
        assert_ne!(&a ^ &b, a);
    }

    #[test]
    fn magnitude() {
        assert_eq!(DVec3 { x: 0., y: 0., z: 0. }.magnitude(), 0.);
        assert_eq!(DVec3 { x: 1., y: 0., z: 0. }.magnitude(), 1.);
        assert_eq!(DVec3 { x: -1., y: 0., z: 0. }.magnitude(), 1.);
        assert_eq!(DVec3 { x: 4., y: -5., z: 6. }.magnitude(), 8.774964387392123);
    }

    #[test]
    fn normalize() {
        let a = DVec3 { x: 10., y: 0., z: 0. };
        let b = DVec3 { x: 1.25, y: -520., z: 12. };
        let c = DVec3 { x: 1., y: 0., z: 0. };
        assert_eq!(a.normalized(), c);
        assert_approx_eq!(b.normalized().magnitude(), 1., f64::EPSILON);
    }

    #[test]
    fn distance_to_line() {
        let a = DVec3 { x: 1., y: 1., z: 0. };
        let b = DVec3 { x: 1., y: 3., z: 0. };
        let c = DVec3 { x: 2., y: 2., z: 0. };
        assert_eq!(c.distance_to_line(&a, &b), 1.);
        assert_approx_eq!(b.distance_to_line(&a, &c), 2.0_f64.sqrt(), 0.00001);
    }
}
