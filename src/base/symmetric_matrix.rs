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
}