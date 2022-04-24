/*
This scene is an memory efficient ECS system
*/

use slotmap::*;
use std::collections::HashMap;
use std::cell::{RefCell, Ref, RefMut};
use super::{EntityId};
use nanomesh_macros::entity;

type Arena<T> = DenseSlotMap<EntityId, T>;

pub trait Entity {
    fn get_id() -> u64;
    fn get_attachement_id(&self) -> Option<EntityId> { None }
    fn set_attachement_id(&mut self, id: EntityId) { }
}

#[entity]
#[derive(Clone)]
pub struct Attachment {
    attached_entity_type: u64,
    attached_entity: EntityId,
    next_attachement: EntityId,
}

impl Default for Attachment {
    fn default() -> Attachment {
        Attachment { attachement_id: None, attached_entity_type: 0, attached_entity: EntityId::default(), next_attachement: EntityId::default() }
    }
}

pub struct Scene {
    entities_per_type: HashMap::<u64, Box<dyn Entities>>,
}

impl Scene {

    pub fn new() -> Self {
        Scene { entities_per_type: HashMap::new() }
    }

    pub fn get_entities_mut<T: Entity+'static>(&self) -> Option<RefMut<Arena<T>>> {
        match self.entities_per_type.get(&T::get_id()) {
            Some(vec) => {
                let any = vec.as_any();
                let refcell = any.downcast_ref::<RefCell<Arena<T>>>().unwrap();
                Some(refcell.borrow_mut())
            },
            None => None
        }
    }

    pub fn get_entities<T: Entity+'static>(&self) -> Option<Ref<Arena<T>>> {
        match self.entities_per_type.get(&T::get_id()) {
            Some(vec) => {
                let any = vec.as_any();
                let refcell = any.downcast_ref::<RefCell<Arena<T>>>().unwrap();
                Some(refcell.borrow())
            },
            None => None
        }
    }

    pub fn add_entity<T: Entity+'static>(&mut self, entity: T) -> EntityId {
        match self.entities_per_type.get_mut(&T::get_id()) {
            Some(vec) => {
                let any = vec.as_any();
                let components = any.downcast_ref::<RefCell<Arena<T>>>().unwrap();
                components.borrow_mut().insert(entity)
            },
            None => {
                let mut slotmap = Arena::with_key();
                let id = slotmap.insert(entity);
                self.entities_per_type.insert(T::get_id(), Box::new(RefCell::new(slotmap)));
                id
            }
        }
    }

    /// Attach two entities together. If entities were already attached, they will end up be attached as well.
    /// ⚠️ Avoid attaching several entities of the same type. This will result in undefined behaviour
    pub fn attach_entities<TA: Entity+'static, TB: Entity+'static>(&mut self, entity_id_a: EntityId, entity_id_b: EntityId) -> Result<(), ()> {

        // Make sure we can handle attachements
        if self.get_entities_mut::<Attachment>().is_none() {
            let slotmap = Arena::<Attachment>::with_key();
            self.entities_per_type.insert(Attachment::get_id(), Box::new(RefCell::new(slotmap)));
        }

        // Todo: can be combined with previous statement
        let mut attachements = self.get_entities_mut::<Attachment>().unwrap();

        let mut entities_b = self.get_entities_mut::<TB>().unwrap();
        let entity_b = entities_b.get_mut(entity_id_b).unwrap();

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

        std::mem::drop(entities_b);

        let mut entities_a = self.get_entities_mut::<TA>().unwrap();
        let entity_a = entities_a.get_mut(entity_id_a).unwrap();

        // Get or create attachement for entity A
        match entity_a.get_attachement_id() {
            Some(attachement_id) => {
                // Si A avait déja un attachement, A doit maintenant pointer vers B, et B prendre l'ancien pointage de A
                let attachement_a_clone = attachements.get_mut(attachement_id).unwrap().clone();

                let mut attachement_b = attachements.get_mut(attachement_id_b).unwrap();
                attachement_b.attached_entity = attachement_a_clone.attached_entity;
                attachement_b.attached_entity_type = attachement_a_clone.attached_entity_type;
                attachement_b.next_attachement = attachement_a_clone.next_attachement;

                let mut attachement_a = attachements.get_mut(attachement_id).unwrap();
                attachement_a.attached_entity = entity_id_b;
                attachement_a.attached_entity_type = TB::get_id();
                attachement_a.next_attachement = attachement_id_b;
            },
            None => {
                // Si A avait pas d'attachement, A doit pointer vers B, et B vers A

                let mut attachement_a = Attachment::default();
                attachement_a.attached_entity = entity_id_b;
                attachement_a.attached_entity_type = TB::get_id();
                attachement_a.next_attachement = attachement_id_b;

                let key = attachements.insert(attachement_a);

                let mut attachement_b = attachements.get_mut(attachement_id_b).unwrap();
                attachement_b.attached_entity = entity_id_a;
                attachement_b.attached_entity_type = TA::get_id();
                attachement_b.next_attachement = key;

                entity_a.set_attachement_id(key);
            }
        };

        std::mem::drop(entities_a);

        Ok(())
    }

    pub fn delete_entity<T: Entity+'static>(&mut self, entity_id: EntityId) -> Result<(), ()> {
        let mut entities = self.get_entities_mut::<T>().unwrap();
        match entities.get(entity_id) {
            Some(entity) => {
                match entity.get_attachement_id() {
                    Some(attachement_id) => {
                        let mut attachements = self.get_entities_mut::<Attachment>().unwrap();
                        let mut current_attachement = attachements.get_mut(attachement_id).unwrap();
                        let first_attachement = current_attachement.clone();
                        loop {
                            if current_attachement.attached_entity_type == T::get_id() && current_attachement.attached_entity == entity_id {
                                // Remap attachements
                                current_attachement.next_attachement = first_attachement.next_attachement;
                                current_attachement.attached_entity_type = first_attachement.attached_entity_type;
                                current_attachement.attached_entity = first_attachement.attached_entity;
                                break;
                            }
                            // Next round
                            let k = current_attachement.next_attachement;
                            current_attachement = attachements.get_mut(k).unwrap();
                        }
                        attachements.remove(attachement_id);
                    },
                    None => {}
                }
                entities.remove(entity_id);
                Ok(())
            },
            None => Err(())
        }
    }

    pub fn get_attached_entity<TA: Entity+'static, TB: Entity+'static>(&mut self, entity_id: EntityId) -> Option<EntityId> {
        let entities_a = self.get_entities::<TA>().unwrap();
        let entity = entities_a.get(entity_id).unwrap();
        match self.get_entities::<Attachment>() {
            Some(attachements) => {
                match entity.get_attachement_id() {
                    Some(attachement_id) => {
                        let mut current_attachement = attachements.get(attachement_id).unwrap();
                        loop {
                            if current_attachement.attached_entity_type == TB::get_id() {
                                return Some(current_attachement.attached_entity);
                            }
                            // Prevent looping undefinitely if there is none of the requested entity type amongst attachements
                            if current_attachement.attached_entity_type == TA::get_id() && current_attachement.attached_entity == entity_id {
                                return None;
                            }
                            // Next round
                            current_attachement = attachements.get(current_attachement.next_attachement).unwrap();
                        }
                    },
                    None => None
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

    use std::borrow::Borrow;

    use nanomesh_macros::entity;

    use super::*;

    #[entity]
    pub struct MyEntityA {
        pub my_value: u32,
    }

    #[entity]
    pub struct MyEntityB {
        pub my_value: u32,
    }

    #[entity]
    pub struct MyEntityC {
        pub my_value: u32,
    }

    #[test]
    fn add_retreive_entity() {
        let mut scene = Scene::new();
        let entity_id = scene.add_entity(MyEntityA { attachement_id: None, my_value: 123 });

        let entities = scene.get_entities::<MyEntityA>().unwrap();
        let entity = entities.get(entity_id).unwrap();

        assert_eq!(123, entity.my_value);
    }

    #[test]
    fn add_similar_entities() {
        let mut scene = Scene::new();

        for i in 0..1000000 {
            scene.add_entity(MyEntityA { attachement_id: None, my_value: 1 });
        }

        let entities = scene.get_entities::<MyEntityA>().unwrap();
        assert_eq!(1000000, entities.len());
    }

    #[test]
    fn attach_two_entities() {
        let mut scene = Scene::new();

        let a_id = scene.add_entity(MyEntityA { attachement_id: None, my_value: 42 });
        let b_id = scene.add_entity(MyEntityB { attachement_id: None, my_value: 69 });

        scene.attach_entities::<MyEntityA, MyEntityB>(a_id, b_id).unwrap();

        let result_id = scene.get_attached_entity::<MyEntityA, MyEntityB>(a_id).unwrap();

        let entities_b = scene.get_entities::<MyEntityB>().unwrap();
        let result = entities_b.get(result_id).unwrap();

        assert_eq!(69, result.my_value);
    }

    #[test]
    fn attach_three_entities() {
        let mut scene = Scene::new();

        let first_id = scene.add_entity(MyEntityA { attachement_id: None, my_value: 1 });
        let second_id = scene.add_entity(MyEntityB { attachement_id: None, my_value: 2 });
        let third_id = scene.add_entity(MyEntityC { attachement_id: None, my_value: 3 });

        scene.attach_entities::<MyEntityA, MyEntityB>(first_id, second_id).unwrap();
        scene.attach_entities::<MyEntityA, MyEntityC>(first_id, third_id).unwrap();

        {
            let result_id = scene.get_attached_entity::<MyEntityA, MyEntityB>(first_id).unwrap();
            let entities = scene.get_entities::<MyEntityB>().unwrap();
            let result = entities.get(result_id).unwrap();
    
            assert_eq!(2, result.my_value);
        }

        {
            let result_id = scene.get_attached_entity::<MyEntityA, MyEntityC>(first_id).unwrap();
            let entities = scene.get_entities::<MyEntityC>().unwrap();
            let result = entities.get(result_id).unwrap();
    
            assert_eq!(3, result.my_value);
        }
    }

    #[test]
    fn get_non_attached_entity_returns_none() {
        let mut scene = Scene::new();

        let a_id = scene.add_entity(MyEntityA { attachement_id: None, my_value: 42 });
        let b_id = scene.add_entity(MyEntityB { attachement_id: None, my_value: 69 });

        scene.attach_entities::<MyEntityA, MyEntityB>(a_id, b_id).unwrap();

        // There is no MyEntityC attached!
        assert_eq!(None, scene.get_attached_entity::<MyEntityA, MyEntityC>(a_id));
    }

    #[test]
    fn get_no_attachements_returns_none() {
        let mut scene = Scene::new();

        let a_id = scene.add_entity(MyEntityA { attachement_id: None, my_value: 42 });

        // There is no MyEntityB attached!
        assert_eq!(None, scene.get_attached_entity::<MyEntityA, MyEntityB>(a_id));
    }

    #[test]
    fn get_no_attachements_on_entity_returns_none() {
        let mut scene = Scene::new();

        let a_id = scene.add_entity(MyEntityA { attachement_id: None, my_value: 42 });
        // Attachements exists but a doesn't have any
        scene.add_entity(Attachment::default());

        // There is no MyEntityB attached!
        assert_eq!(None, scene.get_attached_entity::<MyEntityA, MyEntityB>(a_id));
    }

    #[test]
    fn delete_entity() {
        let mut scene = Scene::new();

        let first_id = scene.add_entity(MyEntityA { attachement_id: None, my_value: 1 });
        let second_id = scene.add_entity(MyEntityB { attachement_id: None, my_value: 2 });
        let third_id = scene.add_entity(MyEntityC { attachement_id: None, my_value: 3 });

        scene.attach_entities::<MyEntityA, MyEntityB>(first_id, second_id).unwrap();
        scene.attach_entities::<MyEntityA, MyEntityC>(first_id, third_id).unwrap();

        // Remove B
        scene.delete_entity::<MyEntityB>(second_id).unwrap();

        {
            // Is not attached anymore
            assert!(scene.get_attached_entity::<MyEntityA, MyEntityB>(first_id).is_none());
            let entities = scene.get_entities::<MyEntityB>().unwrap();
            // Entity does not exist anymore
            assert!(entities.get(first_id).is_none());
        }

        {
            let result_id = scene.get_attached_entity::<MyEntityA, MyEntityC>(first_id).unwrap();
            let entities = scene.get_entities::<MyEntityC>().unwrap();
            let result = entities.get(result_id).unwrap();
    
            assert_eq!(3, result.my_value);
        }
    }
}