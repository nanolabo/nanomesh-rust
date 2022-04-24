pub mod scene;
pub use scene::Scene as Scene;

use slotmap::*;
use std::fmt::{Display, Result, Formatter};
new_key_type! {
    pub struct EntityId;
}

impl Display for EntityId {
    fn fmt(&self, f: &mut Formatter) -> Result {
        write!(f, "{}", self.0.as_ffi())
    }
}
