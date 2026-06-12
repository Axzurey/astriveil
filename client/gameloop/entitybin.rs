use shared::world::entities::entity::Entity;

pub struct EntityBin {
    entities: Vec<Box<dyn Entity>>
}

impl EntityBin {
    pub fn new() -> Self {
        Self {
            entities: Vec::new()
        }
    }

    pub fn add_entity(&mut self, entity: Box<dyn Entity>) {
        self.entities.push(entity);
    }

    pub fn update(&mut self, device: &wgpu::Device) {
        for entity in &mut self.entities {
            entity.update(device);
        }
    }
    pub fn get_entities_with<T: Entity + 'static>(&self) -> impl std::iter::Iterator<Item = &T> {
        self.entities.iter().filter_map(|e| e.as_any().downcast_ref::<T>())
    }
    pub fn get_entities_with_mut<T: Entity + 'static>(&mut self) -> impl std::iter::Iterator<Item = &mut T> {
        self.entities.iter_mut().filter_map(|e| e.as_any_mut().downcast_mut::<T>())
    }
}