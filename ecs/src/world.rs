use std::{
    marker::PhantomData,
    sync::{Arc, Mutex},
};

use crate::{
    component::Component,
    entity::{EntityManager, EntityQueryTable},
    system::System,
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SystemType {
    Init,
    Loop,
}

pub struct SystemContainer<T> {
    loop_systems: Vec<Arc<Mutex<dyn System<T>>>>,
    init_systems: Vec<Arc<Mutex<dyn System<T>>>>,
}

pub struct World<F> {
    pub entity_manager: EntityManager,
    pub entity_query_table: EntityQueryTable,
    pub system_container: SystemContainer<F>,
}

impl<F> World<F> {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            entity_query_table: EntityQueryTable::new(),
            system_container: SystemContainer {
                loop_systems: vec![],
                init_systems: vec![],
            },
        }
    }

    pub fn entity(&mut self) -> usize {
        self.entity_manager.entity()
    }

    pub fn entity_at(&mut self, id: usize) -> usize {
        self.entity_manager.entity_at(id)
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

    pub fn with_system<T>(&mut self, system_type: SystemType, system: T) -> &mut Self
    where
        T: System<F> + 'static,
    {
        let reference_counted = Arc::new(Mutex::new(system));
        let systems = match system_type {
            SystemType::Init => &mut self.system_container.loop_systems,
            SystemType::Loop => &mut self.system_container.init_systems,
        };

        systems.push(reference_counted);

        self
    }

    pub fn update(&mut self, system_type: SystemType, data: &F) {
        let systems = match system_type {
            SystemType::Init => &mut self.system_container.loop_systems,
            SystemType::Loop => &mut self.system_container.init_systems,
        };

        for system in systems.iter_mut() {
            let mut system = system.lock().unwrap();

            system.update(&mut self.entity_manager, &mut self.entity_query_table, data);

            self.entity_manager.tick_frame();
        }
    }
}
