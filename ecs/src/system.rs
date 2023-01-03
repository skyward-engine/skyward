use crate::entity::{EntityManager, EntityQueryTable};

pub trait System<T> {
    fn update(
        &mut self,
        manager: &mut EntityManager,
        table: &mut EntityQueryTable,
        data: &T,
    ) -> Option<()>;
}
