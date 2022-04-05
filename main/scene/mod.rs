pub mod scene;
pub use scene::Scene as Scene;

pub mod tobject;
pub use tobject::TObject as TObject;

pub type EntityId = slotmap::DefaultKey;