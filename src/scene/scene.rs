use crate::objects::SceneObject;
pub type ObjectId = slotmap::DefaultKey;

pub trait Scene {

    fn add<T: 'static + SceneObject>(&mut self, scene_object: T) -> slotmap::DefaultKey;

    fn update(&self); // Todo: add default implementation

    fn get<T : 'static + SceneObject>(&self, id: ObjectId) -> Result<&T, ()> {
        let borrowed_object = self.get_internal(id).unwrap();
        let it1 = borrowed_object.as_any();
        match it1.downcast_ref::<T>() {
            Some(i) => Ok(i),
            None => Err(()),
        }
    }

    fn get_internal(&self, id: ObjectId) -> Option<&Box<dyn SceneObject>>;
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;
    use std::any::Any;
    use slotmap::DenseSlotMap;

    pub struct TestScene {
        pub objects: DenseSlotMap<ObjectId, Box<dyn SceneObject>>,
    }

    impl TestScene {
        pub fn new() -> Self {
            TestScene {
                objects: DenseSlotMap::new(),
            }
        }
    }

    impl TestScene {

        fn get<T : 'static + SceneObject>(&self, id: ObjectId) -> Result<&T, ()> {
            let borrowed_object = self.get_internal(id).unwrap();
            let it1 = borrowed_object.as_any();
            match it1.downcast_ref::<T>() {
                Some(i) => Ok(i),
                None => Err(()),
            }
        }

        fn get_internal(&self, id: ObjectId) -> Option<&Box<dyn SceneObject>> {
            self.objects.get(id)
        }
    }
    
    impl Scene for TestScene {
    
        fn add<T: 'static + SceneObject>(&mut self, scene_object: T) -> ObjectId {
            self.objects.insert(Box::new(scene_object))
        }
    
        fn update(&self) {
            for object in self.objects.iter() {
                object.1.update();
            }
        }

        fn get_internal(&self, id: ObjectId) -> Option<&Box<dyn SceneObject>> {
            self.objects.get(id)
        }
    }

    pub struct TestObject {
        pub rendered: RefCell<u32>,
    }

    impl SceneObject for TestObject {
        fn update(&self) {
            *self.rendered.borrow_mut() += 1;
        }

        fn as_any(&self) -> &dyn Any {
            self
        }
    }

    #[test]
    fn can_retreive_objects() {
        let mut scene = TestScene::new();

        let object1 = TestObject { rendered: RefCell::new(12) };
        let object2 = TestObject { rendered: RefCell::new(42) };

        let id1 = scene.add(object1);
        let id2 = scene.add(object2);

        match scene.get::<TestObject>(id1) {
            Ok(i) => assert_eq!(12, *i.rendered.borrow()),
            Err(_) => panic!(),
        }

        match scene.get::<TestObject>(id2) {
            Ok(i) => assert_eq!(42, *i.rendered.borrow()),
            Err(_) => panic!(),
        }
    }
}