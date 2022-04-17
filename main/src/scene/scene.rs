/*
This scene is an memory efficient ECS system
*/

use slotmap::*;
use std::collections::HashMap;
use std::cell::{RefCell, Ref, RefMut};
use std::thread::current;
use super::{EntityId};
use nanomesh_macros::entity;

type Arena<T> = DenseSlotMap<EntityId, T>;

pub trait Entity {
    fn get_id() -> u64;
    fn get_next(&self) -> Option<(EntityId, u64)>;
    fn set_next(&mut self, next_id: EntityId, next_type_id: u64);
}

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

    fn get_entities_internal(&self, type_id: u64) -> Option<Ref<Arena<T>>> {
        match self.entities_per_type.get(&type_id) {
            Some(vec) => {
                let any = vec.as_any();
                let refcell = any.downcast_ref::<RefCell<Arena<Entity>>>().unwrap();
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
                self.entities_per_type.insert(T::get_id(), Box::new(RefCell::new(slotmap)));
                id
            }
        }
    }

    /// Attach two entities together. If entities were already attached, they will end up be attached as well.
    /// ⚠️ Avoid attaching several entities of the same type. This will result in undefined behaviour
    fn attach_entities<TA: Entity+'static, TB: Entity+'static>(&mut self, entity_id_a: EntityId, entity_id_b: EntityId) -> Result<(), ()> {

        let mut entities_a = self.get_entities_mut::<TA>().unwrap();
        let entity_a = entities_a.get_mut(entity_id_a).unwrap();

        entity_a.set_next(entity_id_b, TB::get_id());

        std::mem::drop(entities_a);

        let mut entities_b = self.get_entities_mut::<TB>().unwrap();
        let entity_b = entities_b.get_mut(entity_id_b).unwrap();

        entity_b.set_next(entity_id_a, TA::get_id());
        
        std::mem::drop(entities_b);

        Ok(())
    }

    fn delete_entity<T: Entity+'static>(&mut self, entity_a: T, entity_b: T) {
        // Todo
    }

    fn get_attached_entity<TA: Entity+'static, TB: Entity+'static>(&mut self, entity_id: EntityId) -> Option<EntityId> {
        let entities_a = self.get_entities::<TA>().unwrap();
        let mut current_entity = entities_a.get(entity_id).unwrap();
        loop {
            match current_entity.get_next() {
                Some((next_id, next_type_id)) => {

                    if next_type_id == TB::get_id() {
                        return Some(next_id);
                    }

                    if next_id == entity_id {
                        println!("looped but not found");
                        return None;
                    }

                    //current_entity = 
                },
                None => return None
            }
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
    fn can_add_retreive_entity() {
        let mut scene = Scene::new();
        let entity_id = scene.add_entity(MyEntityA { next: None, my_value: 123 });

        let entities = scene.get_entities::<MyEntityA>().unwrap();
        let entity = entities.get(entity_id).unwrap();

        assert_eq!(123, entity.my_value);
    }

    #[test]
    fn can_add_similar_entities() {
        let mut scene = Scene::new();

        for i in 0..1000000 {
            scene.add_entity(MyEntityA { next: None, my_value: 1 });
        }

        let entities = scene.get_entities::<MyEntityA>().unwrap();
        assert_eq!(1000000, entities.len());
    }

    #[test]
    fn can_attach_two_entities() {
        let mut scene = Scene::new();

        let a_id = scene.add_entity(MyEntityA { next: None, my_value: 42 });
        let b_id = scene.add_entity(MyEntityB { next: None, my_value: 69 });

        scene.attach_entities::<MyEntityA, MyEntityB>(a_id, b_id).unwrap();

        let result_id = scene.get_attached_entity::<MyEntityA, MyEntityB>(a_id).unwrap();

        let entities_b = scene.get_entities::<MyEntityB>().unwrap();
        let result = entities_b.get(result_id).unwrap();

        assert_eq!(69, result.my_value);
    }

    #[test]
    fn can_attach_three_entities() {
        let mut scene = Scene::new();

        let first_id = scene.add_entity(MyEntityA { next: None, my_value: 1 });
        let second_id = scene.add_entity(MyEntityB { next: None, my_value: 2 });
        let third_id = scene.add_entity(MyEntityC { next: None, my_value: 3 });

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
}