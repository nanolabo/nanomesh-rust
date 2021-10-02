use std::ops::*;

#[derive(Debug)]
pub struct Vector3 {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector3
{
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        return Vector3 { 
            x: x,
            y: y,
            z: z
        }
    }

    pub fn magnitude(&self) -> f64 {
        return (self * self).sqrt();
    }

    pub fn normalized(&self) -> Vector3
    {
        let magnitude = self.magnitude();
        return self / magnitude;
    }
}

impl Add<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn add(self, _rhs: &Vector3) -> Vector3 {
        return Vector3 { 
            x: self.x + _rhs.x,
            y: self.y + _rhs.y,
            z: self.z + _rhs.z
        }
    }
}

impl Sub<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn sub(self, _rhs: &Vector3) -> Vector3 {
        return Vector3 { 
            x: self.x - _rhs.x,
            y: self.y - _rhs.y,
            z: self.z - _rhs.z
        }
    }
}

impl Div<f64> for &Vector3 {
    type Output = Vector3;
    fn div(self, _rhs: f64) -> Vector3 {
        return Vector3 { 
            x: self.x / _rhs,
            y: self.y / _rhs,
            z: self.z / _rhs,
        }
    }
}

// Implementation of dot product
impl Mul<&Vector3> for &Vector3 {
    type Output = f64;
    fn mul(self, _rhs: &Vector3) -> f64 {
        return self.x * _rhs.x + self.y * _rhs.y + self.z * _rhs.z;
    }
}

// Implementation of cross product
impl BitXor<&Vector3> for &Vector3 {
    type Output = Vector3;
    fn bitxor(self, _rhs: &Vector3) -> Vector3 {
        return Vector3 {
            x: self.y * _rhs.z - self.z * _rhs.y,
            y: self.z * _rhs.x - self.x * _rhs.z,
            z: self.x * _rhs.y - self.y * _rhs.x
        }
    }
}

impl PartialEq for Vector3 {
    fn eq(&self, other: &Self) -> bool {
        self.x == other.x && self.y == other.y && self.z == other.z
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assert_approx_eq::*;

    #[test]
    fn add() {
        let a = Vector3 { x: 1., y: 2., z: 3. };
        let b = Vector3 { x: 4., y: 5., z: 6. };
        let c = Vector3 { x: 5., y: 7., z: 9. };
        assert_eq!(&a + &b, c);
        assert_ne!(&a + &b, a);
    }

    #[test]
    fn substract() {
        let a = Vector3 { x: 1., y: 2., z: 3. };
        let b = Vector3 { x: 4., y: 5., z: 6. };
        let c = Vector3 { x: -3., y: -3., z: -3. };
        assert_eq!(&a - &b, c);
        assert_ne!(&a - &b, a);
    }

    #[test]
    fn divide() {
        let a = Vector3 { x: 1., y: 2., z: -3. };
        let b = 2.;
        let c = Vector3 { x: 0.5, y: 1., z: -1.5 };
        assert_eq!(&a / b, c);
        assert_ne!(&a / b, a);
    }

    #[test]
    fn dot() {
        let a = Vector3 { x: 1., y: 2., z: -3. };
        let b = Vector3 { x: 4., y: -5., z: 6. };
        let c = 4. - 10. - 18.;
        assert_eq!(&a * &b, c);
        assert_ne!(&a * &b, 0.);
    }

    #[test]
    fn cross() {
        let a = Vector3 { x: 1., y: 2., z: -3. };
        let b = Vector3 { x: 4., y: -5., z: 6. };
        let c = Vector3 { x: -3.0, y: -18.0, z: -13.0 };
        assert_eq!(&a ^ &b, c);
        assert_ne!(&a ^ &b, a);
    }

    #[test]
    fn magnitude() {
        assert_eq!(Vector3 { x: 0., y: 0., z: 0. }.magnitude(), 0.);
        assert_eq!(Vector3 { x: 1., y: 0., z: 0. }.magnitude(), 1.);
        assert_eq!(Vector3 { x: -1., y: 0., z: 0. }.magnitude(), 1.);
        assert_eq!(Vector3 { x: 4., y: -5., z: 6. }.magnitude(), 8.774964387392123);
    }

    #[test]
    fn normalize() {
        let a = Vector3 { x: 10., y: 0., z: 0. };
        let b = Vector3 { x: 1.25, y: -520., z: 12. };
        let c = Vector3 { x: 1., y: 0., z: 0. };
        assert_eq!(a.normalized(), c);
        assert_approx_eq!(b.normalized().magnitude(), 1., f64::EPSILON);
    }
}
