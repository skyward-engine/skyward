use std::{
    any::{Any, TypeId},
    collections::HashMap,
    mem::transmute,
};

/// Components are any kind of storage which can be linked to an entity.
///
/// This trait can automatically be implemented by using the `ecs-macors::Component` derive macro, which
/// will generate the following code:
/// ```
/// impl Component for MyStruct {}
/// ```
///
/// Examples of components, which are supported by our [World] system.
/// ```
/// #[derive(Component)]
/// struct Position {
///     x: i32,
///     y: i32,
/// }
/// ```
///
/// ```
/// #[derive(Component)]
/// struct Named(&'static str);
/// ```
pub trait Component: Sized + Any {}

/// SimpleComponentManager stores the following things:
/// - Components
///     - These are all of the data components, stored in a vector.
/// - Entities
///     - These are all of the entities which hold data within this manager, so every entity which has
///       any kind of data with the type of T.
/// - Entity Indexes
///     - These indexes are the indexes of the entities' data, these indexes are used to query the contents
///       of the components vector. This is represented in a HashMap<EntityId, ComponentIndex>
///
/// This type implements the two ComponentManager trais: [ComponentManager] and [TypedComponentManager]. Go to
/// their respective docs for further information on these.
///
/// This is a wrapper around the [ComponentManager] trait.
pub struct SimpleComponentManager<T>
where
    T: Component,
{
    pub components: Vec<T>,
    pub entities: Vec<usize>,
    pub entity_idx: HashMap<usize, usize>,
}

impl<T> SimpleComponentManager<T>
where
    T: Component,
{
    pub fn new() -> Self {
        Self {
            components: vec![],
            entities: vec![],
            entity_idx: HashMap::new(),
        }
    }

    pub fn borrow_components_mut(&mut self) -> &mut Vec<T> {
        &mut self.components
    }
}

impl<T> ComponentManager for SimpleComponentManager<T>
where
    T: Component,
{
    fn has(&self, entity: usize) -> bool {
        self.entities.contains(&entity)
    }

    fn clear(&mut self, entity: usize) {
        if !self.has(entity) {
            return;
        }

        let index = *self.entity_idx.get(&entity).unwrap();

        self.entity_idx
            .insert(*self.entities.last().unwrap(), index);
        self.components.swap_remove(index);
        self.entities.swap_remove(index);
        self.entity_idx.remove(&entity);
    }

    fn get_type_id(&self) -> TypeId {
        TypeId::of::<T>()
    }
}

impl<T> TypedComponentManager<T> for SimpleComponentManager<T>
where
    T: Component,
{
    fn with(&mut self, entity: usize, component: T) {
        if self.has(entity) {
            return;
        }

        self.components.push(component);
        self.entities.push(entity);
        self.entity_idx.insert(entity, self.components.len() - 1);
    }

    fn component(&self, entity: usize) -> Option<&T> {
        let index = self.entity_idx.get(&entity)?;
        Some(&self.components[*index])
    }

    fn component_mut(&mut self, entity: usize) -> Option<&mut T> {
        let index = self.entity_idx.get(&entity)?;
        Some(&mut self.components[*index])
    }
}

impl<T> As<dyn Any> for SimpleComponentManager<T>
where
    T: Component,
{
    fn borrow_type(&self) -> &dyn Any {
        self as &dyn Any
    }

    fn borrow_type_mut(&mut self) -> &mut dyn Any {
        self as &mut dyn Any
    }
}

pub fn borrow_manager_ref<T>(manager: &dyn ComponentManager) -> &T
where
    T: 'static + ComponentManager,
{
    let manager = manager.borrow_type();
    let downcast = manager.downcast_ref::<T>().unwrap();

    downcast
}

pub fn borrow_mut_manager<T>(manager: &mut dyn ComponentManager) -> &mut T
where
    T: 'static + ComponentManager,
{
    let manager = manager.borrow_type_mut();
    let downcast = manager.downcast_mut::<T>().unwrap();

    downcast
}

pub fn cast_manager_mut_unsafe<T: 'static + Component>(
    manager: &Box<dyn ComponentManager>,
) -> &mut SimpleComponentManager<T> {
    let ptr = borrow_manager_ref(manager.as_ref()) as *const SimpleComponentManager<T>
        as *mut SimpleComponentManager<T>;
    unsafe { transmute(ptr) }
}

/// This trait holds all of the type-independent functions, which means
/// all type-indepdendent functions should also be added to this trait.
///
/// This is a marker type, allowing to bridge between [SimpleComponentManager] and [ComponentManager]
/// without having to store the [SimpleComponentManager] struct, which means we can have dynamic
/// generic types instead of being limited to a singular generic type.
pub trait ComponentManager: Any + As<dyn Any> {
    fn has(&self, entity: usize) -> bool;
    fn clear(&mut self, entity_id: usize);
    fn get_type_id(&self) -> TypeId;
}

/// This trait holds all of the type-dependent functions, this is separated from [ComponentManager] so
/// we can store the ComponentManager object without infering the type parameter. More information on this
/// can be found under the [ComponentManager] docs.
pub trait TypedComponentManager<T>: ComponentManager {
    fn with(&mut self, entity: usize, component: T);
    fn component(&self, entity: usize) -> Option<&T>;
    fn component_mut(&mut self, entity: usize) -> Option<&mut T>;
}

pub trait As<T: ?Sized> {
    fn borrow_type(&self) -> &T;
    fn borrow_type_mut(&mut self) -> &mut T;
}
