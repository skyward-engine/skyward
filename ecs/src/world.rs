use std::marker::PhantomData;

use crate::{
    component::Component,
    entity::{EntityManager, EntityQueryTable},
    system::System,
};

pub struct World<F> {
    entity_manager: EntityManager,
    entity_query_table: EntityQueryTable,
    systems: Vec<Box<dyn System<F>>>,
    phantom: PhantomData<F>,
}

impl<F> World<F> {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            entity_query_table: EntityQueryTable::new(),
            systems: vec![],
            phantom: PhantomData,
        }
    }

    pub fn entity(&mut self) -> usize {
        self.entity_manager.entity()
    }

    pub fn remove_entity(&mut self, entity: usize) {
        self.entity_manager.remove_entity(entity);
    }

    pub fn register<T>(&mut self) -> &mut Self
    where
        T: Component + 'static,
    {
        {
            self.entity_manager.register::<T>();
        }

        self
    }

    pub fn with<T>(&mut self, entity: usize, component: T) -> &mut Self
    where
        T: Component + 'static,
    {
        {
            self.entity_manager.entity_with(entity, component);
        }

        self
    }

    pub fn with_system<T>(&mut self, system: T) -> &mut Self
    where
        T: System<F> + 'static,
    {
        {
            self.systems.push(Box::new(system));
        }
        self
    }

    pub fn update(&mut self, data: &F) {
        for system in self.systems.iter_mut() {
            system.update(&mut self.entity_manager, &mut self.entity_query_table, data);
            self.entity_manager.tick_frame();
        }
    }
}
