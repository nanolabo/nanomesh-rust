use super::vector3::Vector3;
use std::ops::*;
use std::fmt::*;

#[derive(Debug, Copy, Clone)]
pub struct SymmetricMatrix {
    pub m: [f64; 10]
}

impl SymmetricMatrix {
    pub fn default_uninitalized() -> SymmetricMatrix {
        SymmetricMatrix {
            m: [-1.0; 10]
        }
    }

    pub fn default_zeroes() -> SymmetricMatrix {
        SymmetricMatrix {
            m: [0.0; 10]
        }
    }

    pub fn from_normal(normal: &Vector3, dot: &f64) -> SymmetricMatrix {
        SymmetricMatrix {
            m: [normal.x * normal.x, normal.x * normal.y, normal.x * normal.z, normal.x * dot,
                normal.y * normal.y, normal.y * normal.z, normal.y * dot,
                normal.z * normal.z, normal.z * dot,
                dot * dot]
        }
    }

    pub fn get_det_x(self) -> f64 {
        self.m[1] * self.m[5] * self.m[8] +
        self.m[3] * self.m[4] * self.m[7] +
        self.m[2] * self.m[6] * self.m[5] -
        self.m[3] * self.m[5] * self.m[5] -
        self.m[1] * self.m[6] * self.m[7] -
        self.m[2] * self.m[4] * self.m[8]
    }

    pub fn get_det_y(self) -> f64 {
        self.m[0] * self.m[5] * self.m[8] +
        self.m[3] * self.m[1] * self.m[7] +
        self.m[2] * self.m[6] * self.m[2] -
        self.m[3] * self.m[5] * self.m[2] -
        self.m[0] * self.m[6] * self.m[7] -
        self.m[2] * self.m[1] * self.m[8]
    }

    pub fn get_det_z(self) -> f64 {
        self.m[0] * self.m[4] * self.m[8] +
        self.m[3] * self.m[1] * self.m[5] +
        self.m[1] * self.m[6] * self.m[2] -
        self.m[3] * self.m[4] * self.m[2] -
        self.m[0] * self.m[6] * self.m[5] -
        self.m[1] * self.m[1] * self.m[8]
    }

    pub fn get_det_xyz(self) -> f64 {
        self.m[0] * self.m[4] * self.m[7] +
        self.m[2] * self.m[1] * self.m[5] +
        self.m[1] * self.m[5] * self.m[2] -
        self.m[2] * self.m[4] * self.m[2] -
        self.m[0] * self.m[5] * self.m[5] -
        self.m[1] * self.m[1] * self.m[7]
    }

    pub fn quadric_distance_to_vertex(self, position: &Vector3) -> f64 {
        self.m[0] * position.x * position.x + 2.0 * self.m[1] * position.x * position.y + 2.0 * self.m[2] * position.x * position.z + 2.0 * self.m[3] * position.x +
        self.m[4] * position.y * position.y + 2.0 * self.m[5] * position.y * position.z + 2.0 * self.m[6] * position.y +
        self.m[7] * position.z * position.z + 2.0 * self.m[8] * position.z +
        self.m[9]
    }
}

impl AddAssign for SymmetricMatrix {
    fn add_assign(&mut self, other: Self) {
        *self = Self {
            m: [self.m[0] + other.m[0], self.m[1] + other.m[1], self.m[2] + other.m[2], self.m[3] + other.m[3],
                self.m[4] + other.m[4], self.m[5] + other.m[5], self.m[6] + other.m[6],
                self.m[7] + other.m[7], self.m[8] + other.m[8],
                self.m[9] + other.m[9]]
        };
    }
}

impl Add<&SymmetricMatrix> for &SymmetricMatrix {
    type Output = SymmetricMatrix;
    fn add(self, other: &SymmetricMatrix) -> SymmetricMatrix {
        SymmetricMatrix {
            m: [self.m[0] + other.m[0], self.m[1] + other.m[1], self.m[2] + other.m[2], self.m[3] + other.m[3],
                self.m[4] + other.m[4], self.m[5] + other.m[5], self.m[6] + other.m[6],
                self.m[7] + other.m[7], self.m[8] + other.m[8],
                self.m[9] + other.m[9]]
        }
    }
}

impl Display for SymmetricMatrix {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "<{} {} {} {} | {} {} {} | {} {} | {}>", self.m[0], self.m[1], self.m[2], self.m[3], self.m[4], self.m[5], self.m[6], self.m[7], self.m[8], self.m[9])
    }
}