pub type ObjectId = slotmap::DefaultKey;

pub trait Scene<'a> {
    fn add<T: SceneObject>(&mut self, scene_object: &'a T) -> slotmap::DefaultKey;
    fn update(&self); // Todo: add default implementation
    //fn iter(&self) -> Box<dyn std::iter::Iterator<Item=&Box<&'a dyn SceneObject>>>;
}

pub trait SceneObject {
    fn update(&self) {}
    fn load(&self) {}
    fn unload(&self) {}
}

pub struct Mesh {

}

impl SceneObject for Mesh {
    fn update(&self) {

    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    use slotmap::DenseSlotMap;

    pub struct TestScene<'a> {
        pub objects: DenseSlotMap<ObjectId, Box<&'a dyn SceneObject>>,
    }
    
    impl TestScene<'_> {
        pub fn new() -> Self {
            TestScene {
                objects: DenseSlotMap::new(),
            }
        }
    }
    
    
    impl<'a> Scene<'a> for TestScene<'a> {
    
        fn add<T: SceneObject>(&mut self, scene_object: &'a T) -> ObjectId {
            self.objects.insert(Box::new(scene_object))
        }
    
        fn update(&self) {
    
            let v: Vec<&Box<&dyn SceneObject>> = self.objects.values().collect();
            let k = self.objects.values().enumerate();
    
            for object in self.objects.iter() {
                object.1.update();
            }
        }
    
        // fn iter(&self) -> Box<dyn std::iter::Iterator<Item=&Box<&'a dyn SceneObject>>> {
        //     //let v: Vec<&Box<&dyn SceneObject>> = self.objects.values().collect();
        //     return Box::new(self.objects.values());
        // }
    }

    pub struct TestObject {
        pub rendered: RefCell<u32>,
    }

    impl SceneObject for TestObject {
        fn update(&self) {
            *self.rendered.borrow_mut() += 1;
        }
    }

    #[test]
    fn scene_updates_objects() {
        let mut scene = TestScene::new();
        let object = TestObject { rendered: RefCell::new(0) };
        scene.add(&object);
        scene.update();
        assert_eq!(1, *object.rendered.borrow());
    }
}
