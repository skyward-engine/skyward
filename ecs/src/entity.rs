use std::{any::TypeId, collections::HashMap};

use crate::component::{
    self, cast_manager_mut_unsafe, Component, ComponentManager, SimpleComponentManager,
    TypedComponentManager,
};

#[derive(Clone, Copy, Debug, PartialEq, PartialOrd, Eq, Ord, Hash)]
pub struct Entity {
    id: u32,
    alive: bool,
}

impl Entity {
    pub fn new(id: u32) -> Self {
        Self { id, alive: true }
    }
}

struct EntityContainer {
    entities: Vec<Entity>,
    dead_idx: Vec<usize>,
}

impl EntityContainer {
    pub fn new() -> Self {
        Self {
            entities: vec![],
            dead_idx: vec![],
        }
    }

    pub fn has(&self, entity_id: usize) -> bool {
        entity_id < self.entities.len()
    }

    pub fn entity(&mut self) -> usize {
        if !self.dead_idx.is_empty() {
            let index = self.dead_idx.remove(0) as usize;
            let mut entity = self.entities[index];

            entity.alive = true;
            return index;
        }

        let id = (self.entities.len() + 1).try_into().unwrap();
        let entity = Entity::new(id);

        self.entities.push(entity);
        (id - 1).try_into().unwrap()
    }

    pub fn remove(&mut self, entity_id: usize) {
        if !self.has(entity_id) {
            return;
        }

        self.dead_idx.push(entity_id);
        self.entities[entity_id].alive = false;
    }
}

pub struct EntityManager {
    container: EntityContainer,
    managers: HashMap<TypeId, Box<dyn ComponentManager>>,
    frame_map: HashMap<TypeId, u64>,
    frame: u64,
}

pub struct TupleData<'a> {
    entities: &'a Vec<usize>,
    type_id: TypeId,
    update_frame: u64,
}

macro_rules! tuple {
    ($($T:ident),+) => {
        impl<$($T : component::Component,)+> Tuple for ($($T, )+) {
            fn for_every_type<K, V>(manager: &mut EntityManager, mut f: K) -> Option<()>
                where K: FnMut(TupleData) -> V
            {
                $(
                    {
                        let type_id = TypeId::of::<$T>();
                        let entities = manager.query_entity_ids::<$T>()?;
                        let update_frame = manager.get_updated_frame::<$T>();

                        f(
                            TupleData {
                                entities,
                                type_id,
                                update_frame,
                            }
                        );
                    }
                )+

                None
            }
        }
    };
}

macro_rules! query {
    ($name:ident<$($T:ident),+>) => {
        pub fn $name<$($T : component::Component,)+>(&mut self, entity: usize) -> Option<($(&mut $T,)+)> {
            Some((
                $(
                    {
                        let type_id = TypeId::of::<$T>();
                        let manager: &mut SimpleComponentManager<$T> = cast_manager_mut_unsafe(self.managers.get(&type_id)?);

                        manager.component_mut(entity).unwrap()
                    },
                )+
            ))
        }
    }
}

pub trait Tuple {
    fn for_every_type<K, V>(manager: &mut EntityManager, f: K) -> Option<()>
    where
        K: FnMut(TupleData) -> V;
}

tuple!(T1);
tuple!(T1, T2);
tuple!(T1, T2, T3);
tuple!(T1, T2, T3, T4);
tuple!(T1, T2, T3, T4, T5);
tuple!(T1, T2, T3, T4, T5, T6);

impl EntityManager {
    pub fn new() -> Self {
        Self {
            container: EntityContainer::new(),
            managers: HashMap::new(),
            frame_map: HashMap::new(),
            frame: 0,
        }
    }

    pub fn get_updated_frame<T: 'static + Component>(&self) -> u64 {
        let type_id = TypeId::of::<T>();
        let retrieved = self.frame_map.get(&type_id);

        if let Some(id) = retrieved {
            return *id;
        }

        return 0;
    }

    pub fn get_updated_frame_type(&self, type_id: TypeId) -> u64 {
        let retrieved = self.frame_map.get(&type_id);

        if let Some(id) = retrieved {
            return *id;
        }

        return 0;
    }

    pub fn register<T>(&mut self) -> &mut Self
    where
        T: 'static + Component,
    {
        let type_id = TypeId::of::<T>();

        if self.managers.contains_key(&type_id) {
            return self;
        }

        self.managers
            .insert(type_id, Box::new(SimpleComponentManager::<T>::new()));
        self.frame_map.insert(type_id, self.frame);

        self
    }

    pub fn entity(&mut self) -> usize {
        self.container.entity()
    }

    pub fn remove_entity(&mut self, entity_id: usize) {
        for entry in self.managers.iter_mut() {
            let type_id = entry.0;
            let manager = entry.1;

            manager.clear(entity_id);
            self.frame_map.insert(*type_id, self.frame + 1);
        }

        self.container.remove(entity_id);
    }

    pub fn tick_frame(&mut self) {
        self.frame += 1;
    }

    pub fn entity_with<T>(&mut self, entity_id: usize, component: T) -> &mut Self
    where
        T: 'static + Component,
    {
        let type_id = TypeId::of::<T>();

        if !self.managers.contains_key(&type_id) {
            self.register::<T>();
        }

        if let Some(manager) = self.borrow_manager_mut::<T>() {
            manager.with(entity_id, component);
        }

        self.frame_map.insert(type_id, self.frame);
        self
    }

    pub fn borrow_manager<T: 'static + Component>(&self) -> Option<&SimpleComponentManager<T>> {
        let type_id = TypeId::of::<T>();
        let inner = self.managers.get(&type_id)?.as_ref();

        Some(component::borrow_manager_ref(inner))
    }

    pub fn borrow_manager_mut<T: 'static + Component>(
        &mut self,
    ) -> Option<&mut SimpleComponentManager<T>> {
        let type_id = TypeId::of::<T>();
        let inner = self.managers.get_mut(&type_id)?.as_mut();

        Some(component::borrow_mut_manager(inner))
    }

    pub fn query_entity_ids<T: 'static + Component>(&self) -> Option<&Vec<usize>> {
        Some(&self.borrow_manager::<T>()?.entities)
    }

    pub fn query<T: Component>(&mut self) -> Option<&mut Vec<T>> {
        Some(
            component::cast_manager_mut_unsafe(self.managers.get(&TypeId::of::<T>()).unwrap())
                .borrow_components_mut(),
        )
    }

    query!(query_entity<T>);
    query!(query_entity_two<T1, T2>);
    query!(query_entity_three<T, T2, T3>);
    query!(query_entity_four<T1, T2, T3, T4>);
}

pub struct EntityQueryTable {
    query_cache: HashMap<TypeId, Vec<usize>>,
    frames: HashMap<TypeId, u64>,
}

impl EntityQueryTable {
    pub fn new() -> Self {
        Self {
            query_cache: HashMap::new(),
            frames: HashMap::new(),
        }
    }

    pub fn query_single<T>(&mut self, manager: &mut EntityManager) -> Option<&Vec<usize>>
    where
        T: Component,
    {
        let type_id = TypeId::of::<T>();

        if !self.query_cache.contains_key(&type_id) {
            self.query_cache.insert(type_id, vec![]);

            let entities = &manager.borrow_manager::<T>()?.entities;
            let cache = self.query_cache.get_mut(&type_id)?;

            cache.clear();

            for entity in entities {
                cache.push(*entity);
            }
        }

        self.query_cache.get(&type_id)
    }

    pub fn query<T>(&mut self, manager: &mut EntityManager) -> Option<Vec<usize>>
    where
        T: Tuple,
    {
        let mut vec = Vec::<usize>::new();

        T::for_every_type::<_, Option<()>>(manager, |data| {
            let type_id = data.type_id;
            let entities = data.entities;

            let mut update = true;

            if !self.query_cache.contains_key(&type_id) {
                self.query_cache.insert(type_id, vec![]);
            } else {
                let updated_frame = self.frames.get(&type_id);

                if data.update_frame == *updated_frame.unwrap_or(&(data.update_frame + 1)) {
                    update = false;
                }
            }

            let cache = self.query_cache.get_mut(&type_id)?;

            if update {
                cache.clear();

                for entity in entities {
                    cache.push(*entity);
                }

                self.frames.insert(type_id, data.update_frame);
            }

            for entity in cache.iter() {
                vec.push(*entity);
            }

            None
        });

        T::for_every_type::<_, Option<()>>(manager, |data| {
            let entities = data.entities;

            let mut to_remove = Vec::<usize>::new();

            for e in &vec {
                if entities.contains(e) {
                    continue;
                }

                to_remove.push(*e);
            }

            for entity in &to_remove {
                vec.retain(|e| e == entity);
            }

            None
        });

        Some(vec)
    }
}
