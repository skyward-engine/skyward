use std::{
    borrow::BorrowMut,
    sync::{Arc, Mutex},
    thread,
};

use crate::{
    component::Component,
    entity::{EntityManager, EntityQueryTable},
    system::{MultiThreadSystem, System},
};

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
pub enum SystemType {
    Init,
    Loop,
}

pub struct SystemContainer<T> {
    loop_systems: Vec<Arc<Mutex<dyn System<T>>>>,
    init_systems: Vec<Arc<Mutex<dyn System<T>>>>,
    threaded_loop_systems: Mutex<Vec<Arc<Mutex<dyn MultiThreadSystem + Send + Sync>>>>,
    threaded_init_systems: Mutex<Vec<Arc<Mutex<dyn MultiThreadSystem + Send + Sync>>>>,
}

pub struct World<F> {
    pub entity_manager: EntityManager,
    pub entity_query_table: EntityQueryTable,
    pub system_container: SystemContainer<F>,
}

unsafe impl<T> Send for World<T> {}
unsafe impl<T> Sync for World<T> {}

impl<F> World<F> {
    pub fn new() -> Self {
        Self {
            entity_manager: EntityManager::new(),
            entity_query_table: EntityQueryTable::new(),
            system_container: SystemContainer {
                loop_systems: vec![],
                init_systems: vec![],
                threaded_init_systems: Mutex::new(vec![]),
                threaded_loop_systems: Mutex::new(vec![]),
            },
        }
    }

    pub fn boxed(self) -> Box<Self> {
        Box::new(self)
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

    pub fn with_threaded_system<T>(&mut self, system_type: SystemType, system: T) -> &mut Self
    where
        T: MultiThreadSystem + Send + Sync + 'static,
    {
        let reference_counted = Arc::new(Mutex::new(system));
        let systems = match system_type {
            SystemType::Init => &mut self.system_container.threaded_loop_systems,
            SystemType::Loop => &mut self.system_container.threaded_init_systems,
        };

        systems
            .lock()
            .expect("Unable to lock systems! Did you push outside of initialization block?")
            .push(reference_counted);

        self
    }

    pub fn update(&mut self, system_type: SystemType, data: &F) {
        let systems = match system_type {
            SystemType::Init => &mut self.system_container.loop_systems,
            SystemType::Loop => &mut self.system_container.init_systems,
        };

        for system in systems.iter_mut() {
            system.lock().unwrap().update(
                &mut self.entity_manager,
                &mut self.entity_query_table,
                data,
            );

            self.entity_manager.tick_frame();
        }

        let systems = match system_type {
            SystemType::Init => &self.system_container.threaded_loop_systems,
            SystemType::Loop => &self.system_container.threaded_init_systems,
        };

        // todo: figure this out
        thread::spawn(move || {
            for system in systems
                .lock()
                .expect("Unable to lock systems! Did you push outside of initialization block?")
                .iter_mut()
            {
                system
                    .lock()
                    .expect("Unable to lock system! Is the thread busy?")
                    .update(&mut self.entity_manager, &mut self.entity_query_table);

                self.entity_manager.tick_frame();
            }
        });
    }
}

pub trait Leak<T> {
    fn leaked(self) -> &'static mut T;
}

impl<T> Leak<World<T>> for Box<World<T>> {
    fn leaked(self) -> &'static mut World<T> {
        Box::leak(self)
    }
}
