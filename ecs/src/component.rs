use std::{
    any::{Any, TypeId},
    collections::HashMap,
    mem::transmute,
};

/// A `Component` is a piece of data that can be linked to an entity.
///
/// # Deriving
///
/// This trait can be automatically implemented for a struct using the `ecs-macros::EntityComponent` derive macro:
///
/// ```
/// #[derive(EntityComponent)]
/// struct MyStruct;
/// ```
///
/// This will generate the following implementation:
///
/// ```
/// impl Component for MyStruct {}
/// ```
///
/// # Examples
///
/// ```
/// #[derive(EntityComponent)]
/// struct Position {
///     x: i32,
///     y: i32,
/// }
///
/// #[derive(Component)]
/// struct Named(&'static str);
/// ```
///
/// These structs can be used as components within a `World` system.
pub trait Component: Sized + Any {}

/// `SimpleComponentManager` is a struct that stores and manages components, entities, and entity indexes.
/// It implements the [ComponentManager] and [TypedComponentManager] traits.
///
/// # Fields
///
/// - `components`: A vector of data components.
/// - `entities`: A vector of entities that hold data of type `T`.
/// - `entity_idx`: A `HashMap` that maps an entity's ID to its component index. This is used to query the contents of the `components` vector.
///
/// # Type Parameters
///
/// - `T`: The type of component being managed. Must implement the `Component` trait.
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

/// `ComponentManager` is a marker trait that defines type-independent functions for managing components.
/// It is implemented for the [SimpleComponentManager] struct and allows for bridging between [SimpleComponentManager] and [TypedComponentManager]
/// without having to store the [SimpleComponentManager] struct. This allows for dynamic generic types instead of being limited to a single generic type.
///
/// # Methods
///
/// - `has`: Returns a boolean indicating whether the given entity has a component of this type.
/// - `clear`: Removes the component of this type from the given entity.
/// - `get_type_id`: Returns the `TypeId` of the component type being managed.
pub trait ComponentManager: Any + As<dyn Any> {
    fn has(&self, entity: usize) -> bool;
    fn clear(&mut self, entity_id: usize);
    fn get_type_id(&self) -> TypeId;
}

/// `TypedComponentManager` is a trait that defines type-dependent functions for managing components. It is separated from [ComponentManager]
/// so that the [ComponentManager] object can be stored without inferring the type parameter.
///
/// # Type Parameters
///
/// - `T`: The type of component being managed. Must implement the `Component` trait.
///
/// # Methods
///
/// - `with`: Associates a component of type `T` with the given entity.
/// - `component`: Returns a reference to the component of type `T` for the given entity, if it exists.
/// - `component_mut`: Returns a mutable reference to the component of type `T` for the given entity, if it exists.
pub trait TypedComponentManager<T>: ComponentManager {
    fn with(&mut self, entity: usize, component: T);
    fn component(&self, entity: usize) -> Option<&T>;
    fn component_mut(&mut self, entity: usize) -> Option<&mut T>;
}

pub trait As<T: ?Sized> {
    fn borrow_type(&self) -> &T;
    fn borrow_type_mut(&mut self) -> &mut T;
}
