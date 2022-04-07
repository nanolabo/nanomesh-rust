use slotmap::*;
use std::collections::HashMap;
use std::cell::{RefCell, Ref, RefMut};
use super::{EntityId};
use nanomesh_macros::Entity;

type Arena<T> = DenseSlotMap<EntityId, T>;

pub trait HelloMacro {
    fn hello_macro();
    fn scum();
}

pub trait Entity {
    fn get_id() -> u64;
    fn get_attachement_id(&self) -> Option<EntityId> { None }
    fn set_attachement_id(&mut self, id: EntityId) { }
}

#[derive(Default)]
#[derive(Entity)]
pub struct Attachment {
    attachement_id: Option<EntityId>,
    attached_entity_type: u64,
    attached_entity: EntityId,
    next_attachement: EntityId,
}

#[derive(Entity)]
pub struct Transform {
    parent_id: EntityId,
    child_id: EntityId,
    attachement_id: Option<EntityId>,
}

// impl Transform {
//     pub fn new(scene: &mut Scene2, entity_id: EntityId, parent_id: EntityId) -> EntityId {
//         let transform = Transform { parent_id: parent_id, child_id: EntityId::default(), attached_entity: EntityAttachment::default() };
//         scene.add_entity(transform)
//     }
// }

pub struct Scene {
    entities_per_type: HashMap::<u64, Box<dyn Entities>>,
}

impl Scene {

    fn new() -> Self {
        Scene { entities_per_type: HashMap::new() }
    }

    fn get_entities_mut<T: Entity+'static>(&self) -> Option<RefMut<Arena<T>>> {
        match self.entities_per_type.get(&T::get_id()) {
            Some(vec) => {
                let any = vec.as_any();
                let refcell = any.downcast_ref::<RefCell<Arena<T>>>().unwrap();
                Some(refcell.borrow_mut())
            },
            None => None
        }
    }

    fn get_entities<T: Entity+'static>(&self) -> Option<Ref<Arena<T>>> {
        match self.entities_per_type.get(&T::get_id()) {
            Some(vec) => {
                let any = vec.as_any();
                let refcell = any.downcast_ref::<RefCell<Arena<T>>>().unwrap();
                Some(refcell.borrow())
            },
            None => None
        }
    }

    fn add_entity<T: Entity+'static>(&mut self, entity: T) -> EntityId {
        match self.entities_per_type.get_mut(&T::get_id()) {
            Some(vec) => {
                let any = vec.as_any();
                let components = any.downcast_ref::<RefCell<Arena<T>>>().unwrap();
                components.borrow_mut().insert(entity)
            },
            None => {
                let mut slotmap = Arena::with_key();
                let id = slotmap.insert(entity);
                println!("id:{}", id);
                self.entities_per_type.insert(T::get_id(), Box::new(RefCell::new(slotmap)));
                id
            }
        }
    }

    /// Attach two entities together. If entities were already attached, they will end up be attached as well.
    /// ⚠️ Avoid attaching several entities of the same type. This will result in undefined behaviour
    fn attach_entities<TA: Entity+'static, TB: Entity+'static>(&mut self, entity_id_a: EntityId, entity_id_b: EntityId) -> Result<(), ()> {

        if self.get_entities_mut::<Attachment>().is_none() {
            let mut slotmap = Arena::<Attachment>::with_key();
            self.entities_per_type.insert(Attachment::get_id(), Box::new(RefCell::new(slotmap)));
        }

        let mut entitiesA = self.get_entities_mut::<TA>().unwrap();
        let entity_a = entitiesA.get_mut(entity_id_a).unwrap();

        let mut attachements = self.get_entities_mut::<Attachment>().unwrap();

        // Get or create attachement for entity A
        let attachement_id_a = match entity_a.get_attachement_id() {
            Some(attachement_id) => {
                attachement_id
            },
            None => {
                let key = attachements.insert(Attachment::default());
                entity_a.set_attachement_id(key);
                key
            }
        };

        std::mem::drop(entitiesA);

        let mut entitiesB = self.get_entities_mut::<TB>().unwrap();
        let entity_b = entitiesB.get_mut(entity_id_b).unwrap();

        // Get or create attachement for entity B
        let attachement_id_b = match entity_b.get_attachement_id() {
            Some(attachement_id) => {
                attachement_id
            },
            None => {
                let key = attachements.insert(Attachment::default());
                entity_b.set_attachement_id(key);
                key
            }
        };

        std::mem::drop(entitiesB);

        println!("a:{}", attachement_id_a);
        println!("b:{}", attachement_id_b);

        {
            let mut attachement_a = attachements.get_mut(attachement_id_a).unwrap();
            //attachement_a.id = attachement_id_a;
            attachement_a.attached_entity = entity_id_b;
            attachement_a.attached_entity_type = TB::get_id();
            attachement_a.next_attachement = attachement_id_b;
        }

        {
            let mut attachement_b = attachements.get_mut(attachement_id_a).unwrap();
            //attachement_b.id = attachement_id_b;
            attachement_b.attached_entity = entity_id_a;
            attachement_b.attached_entity_type = TA::get_id();
            attachement_b.next_attachement = attachement_id_a;
        }

        // attachement_a.next_attachement = attachement_b.id;

        Ok(())
    }

    fn delete_entity<T: Entity+'static>(&mut self, entity_a: T, entity_b: T) {
        // Todo
    }

    fn get_attached_entity<TA: Entity+'static, TB: Entity+'static>(&mut self, entity_id: EntityId) -> Option<EntityId> {
        let entities_a = self.get_entities::<TA>().unwrap();
        let entity = entities_a.get(entity_id).unwrap();
        match self.get_entities::<Attachment>() {
            Some(attachements) => {
                let attachement_id = entity.get_attachement_id().unwrap();
                let mut current_attachement = attachements.get(attachement_id).unwrap();
                println!("current att id:{}", attachement_id);
                loop {
                    if current_attachement.attached_entity_type == TB::get_id() {
                        return Some(current_attachement.attached_entity);
                    }
                    println!("next att id:{}", current_attachement.next_attachement);
                    current_attachement = attachements.get(current_attachement.next_attachement).unwrap();
                    // PROBLEME
                    // if current_attachement.attached_entity == first_attachement.attached_entity {
                    //     println!("merde");
                    //     return None;
                    // }
                }
            },
            None => None
        }
    }
}

pub trait Entities {
    fn as_any(&self) -> &dyn std::any::Any;
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any;
}

impl<T: 'static> Entities for RefCell<Arena<T>> {
    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
    fn as_any_mut(&mut self) -> &mut dyn std::any::Any {
        self as &mut dyn std::any::Any
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::any::Any;

    #[derive(Entity)]
    pub struct MyEntityA {
        pub attachement_id: Option<EntityId>,
        pub my_value: u32,
    }

    #[derive(Entity)]
    pub struct MyEntityB {
        pub attachement_id: Option<EntityId>,
        pub my_value: u32,
    }

    #[test]
    fn can_retreive_objects2() {
        let mut scene = Scene::new();

        let a_id = scene.add_entity(MyEntityA { attachement_id: None, my_value: 42 });
        let b_id = scene.add_entity(MyEntityB { attachement_id: None, my_value: 69 });

        scene.attach_entities::<MyEntityA, MyEntityB>(a_id, b_id).unwrap();

        let result_id = scene.get_attached_entity::<MyEntityA, MyEntityB>(a_id).unwrap();

        let entities_b = scene.get_entities::<MyEntityA>().unwrap();
        let result = entities_b.get(result_id).unwrap();

        assert_eq!(69, result.my_value);
    }
}