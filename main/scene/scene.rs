use slotmap::DenseSlotMap;
use std::collections::HashMap;
use std::cell::{RefCell, Ref, RefMut};
use super::*;
use nanomesh_macros::HelloMacro;

type Arena<T> = DenseSlotMap<EntityId, T>;

pub trait HelloMacro {
    fn hello_macro();
}

pub trait Entity {
    fn get_id() -> u32;
    fn attachement_id(&self) -> Option<EntityId> { None }
    fn set_attachement_id(&mut self, id: EntityId) { }
}

#[derive(Default)]
#[derive(HelloMacro)]
pub struct Attachment {
    id: EntityId,
    attached_entity_type: u32,
    attached_entity: EntityId,
    next_attachement: EntityId,
}

impl Entity for Attachment { 
    fn get_id() -> u32 { 2 }
}

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

impl Entity for Transform { 
    fn get_id() -> u32 { 1 }
    fn attachement_id(&self) -> Option<EntityId> { self.attachement_id }
}

pub struct Scene {
    entities_per_type: HashMap::<u32, Box<dyn Entities>>,
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
                let mut slotmap = Arena::new();
                let id = slotmap.insert(entity);
                self.entities_per_type.insert(T::get_id(), Box::new(RefCell::new(slotmap)));
                id
            }
        }
    }

    /// Attach two entities together. If entities were already attached, they will end up be attached as well.
    /// ⚠️ Avoid attaching several entities of the same type. This will result in undefined behaviour
    fn attach_entities<TA: Entity+'static, TB: Entity+'static>(&mut self, entity_id_a: EntityId, entity_id_b: EntityId) -> Result<(), ()> {

        if self.get_entities_mut::<Attachment>().is_none() {
            let mut slotmap = Arena::<Attachment>::new();
            self.entities_per_type.insert(Attachment::get_id(), Box::new(RefCell::new(slotmap)));
        }

        let entities = self.get_entities::<TA>().unwrap();
        let entity_a = entities.get(entity_id_a).unwrap();
        let entity_b = entities.get(entity_id_b).unwrap();

        let mut attachements = self.get_entities_mut::<Attachment>().unwrap();

        // Get or create attachement for entity A
        let attachement_id_a = match entity_a.attachement_id() {
            Some(attachement_id) => {
                attachement_id
            },
            None => {
                let key = attachements.insert(Attachment::default());
                key
            }
        };

        // Get or create attachement for entity B
        let attachement_id_b = match entity_a.attachement_id() {
            Some(attachement_id) => {
                attachement_id
            },
            None => {
                let key = attachements.insert(Attachment::default());
                key
            }
        };

        {
            let mut attachement_a = attachements.get_mut(attachement_id_a).unwrap(); // fails
            attachement_a.id = attachement_id_a;
            attachement_a.attached_entity = entity_id_b;
            attachement_a.attached_entity_type = TB::get_id();
            attachement_a.next_attachement = attachement_id_b;
        }

        {
            let mut attachement_b = attachements.get_mut(attachement_id_a).unwrap();
            attachement_b.id = attachement_id_b;
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
                let mut current_attachement = attachements.get(entity.attachement_id().unwrap()).unwrap();
                let first_attachement = current_attachement;
                loop {
                    if current_attachement.attached_entity_type == TB::get_id() {
                        return Some(current_attachement.attached_entity);
                    }
                    current_attachement = attachements.get(current_attachement.next_attachement).unwrap();
                    if current_attachement.attached_entity == first_attachement.attached_entity {
                        return None;
                    }
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

    pub struct MyEntityA {
        pub attachement_id: Option<EntityId>,
        pub my_value: u32,
    }

    impl Entity for MyEntityA {
        fn get_id() -> u32 {
            123
        }
        fn attachement_id(&self) -> Option<EntityId> {
            self.attachement_id
        }
        fn set_attachement_id(&mut self, id: EntityId) { self.attachement_id = Some(id); }
    }

    pub struct MyEntityB {
        pub attachement_id: Option<EntityId>,
        pub my_value: u32,
    }

    impl Entity for MyEntityB {
        fn get_id() -> u32 {
            124
        }
        fn attachement_id(&self) -> Option<EntityId> {
            self.attachement_id
        }
        fn set_attachement_id(&mut self, id: EntityId) { self.attachement_id = Some(id); }
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

    #[test]
    fn say_hello() {
        Attachment::hello_macro();
    }
}