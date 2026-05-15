use crate::engine::ecs::coordinator::Coordinator;

pub struct EditorContext<'a> {
    pub coordinator: &'a mut Coordinator,
}