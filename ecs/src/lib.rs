use std::any::Any;

pub trait With<T> {
    fn get(&self) -> &T;
}

pub trait Container {
    fn get_attr<T: Any>(&self) -> Vec<&T>;
    fn get<T: Any>(&self) -> Option<&T>;
    fn with_attr<T: Any>(&mut self, attr: T);
    fn with<T: Any>(self, attr: T) -> Self;
}
