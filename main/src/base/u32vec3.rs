use std::ops::*;
use std::fmt::*;

#[derive(Debug, Copy, Clone)]
pub struct U32Vec3(u32, u32, u32);

impl U32Vec3
{
    pub fn new(s0: u32, s1: u32, s2: u32) -> Self {
        return U32Vec3 { 
            0: s0,
            1: s1,
            2: s2
        }
    }
}

impl PartialEq for U32Vec3 {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0 && self.1 == other.1 && self.2 == other.2
    }
}

impl Default for U32Vec3 {
    fn default() -> Self {
        U32Vec3 { 0: 0, 1: 0, 2: 0 }
    }
}

impl Display for U32Vec3 {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<0:{} 1:{} 2:{}>", self.0, self.1, self.2)
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
