pub mod stats;
pub mod surface;
pub mod triangulate;
pub mod curve;

#[derive(thiserror::Error, Debug, Eq, PartialEq)]
pub enum Error {
    #[error("Could not lower point to 2D for triangulation")]
    CouldNotLower,

    #[error("Could not convert into a Surface")]
    UnknownSurfaceType,

    #[error("Could not convert into a Curve")]
    UnknownCurveType,

    #[error("Closed NURBS and b-spline surfaces are not implemented")]
    ClosedSurface,

    #[error("Self-intersecting NURBS and b-spline surfaces are not implemented")]
    SelfIntersectingSurface,

    #[error("Closed NURBS and b-spline curves are not implemented")]
    ClosedCurve,

    #[error("Self-intersecting NURBS and b-spline curves are not implemented")]
    SelfIntersectingCurve,
}

#[cfg(test)]
#[allow(non_snake_case)]
mod triangulate_tests {
    use super::*;

    #[test]
    fn dummy_test() {
        assert_eq!(1, 1); 
    }
}