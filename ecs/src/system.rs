use crate::entity::{EntityManager, EntityQueryTable};

pub trait System<T>: Send + Sync {
    fn update(
        &mut self,
        manager: &mut EntityManager,
        table: &mut EntityQueryTable,
        data: &T,
    ) -> Option<()>;
}

pub trait MultiThreadSystem: Send + Sync {
    fn update(&mut self, manager: &mut EntityManager, table: &mut EntityQueryTable) -> Option<()>;
}
