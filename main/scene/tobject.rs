use std::any::Any;
use super::*;

pub trait TObject {
    fn as_any(&self) -> &dyn Any;
    fn next_component(&self) -> EntityId;
}

pub struct Transform {
    pub component: EntityId,
    pub parent: EntityId,
    pub next_child: EntityId,
}

impl Transform {
    fn iter<'a>(&'a self, scene: &'a Scene) -> ComponentIterator<'a> {
        ComponentIterator::<'a> {
            current_entity: self.component,
            scene: scene
        }
    }

    // Todo: Add as child, delete, reparent
    // Todo: Iter on components of a specific type
}

impl TObject for Transform {

    fn as_any(&self) -> &dyn Any {
        self
    }

    fn next_component(&self) -> EntityId {
        return self.component;
    }
}

pub struct ComponentIterator<'a> {
    current_entity: EntityId,
    scene: &'a Scene,
}

// impl Iterator for ComponentIterator<'_> {
//     type Item = EntityId;

//     fn next(&mut self) -> Option<Self::Item> {
//         match self.scene.get_internal(self.current_entity) {
//             Some(object) => {
//                 let entity = self.current_entity;
//                 self.current_entity = object.next_component();
//                 Some(entity)
//             },
//             None => None
//         }
//     }
// }