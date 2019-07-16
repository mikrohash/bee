pub use std::collections::HashMap;
pub use lazy_static::lazy_static;
pub use bee_transaction::SharedTransaction;
pub use bee_transaction::TransactionBytes;

lazy_static! {
    // common TypedSupervisors
    pub static ref SUPERVISOR_U8 : SharedTypedSupervisor<'static, u8> = SharedTypedSupervisor::new();
    pub static ref SUPERVISOR_STRING : SharedTypedSupervisor<'static, String> = SharedTypedSupervisor::new();
    pub static ref SUPERVISOR_TX : SharedTypedSupervisor<'static, SharedTransaction> = SharedTypedSupervisor::new();
    pub static ref SUPERVISOR_TX_BYTES : SharedTypedSupervisor<'static, TransactionBytes> = SharedTypedSupervisor::new();
}

/// Can listen to effects.
pub trait Entity<T> {
    fn on_effect(&self, effect : &T);
}

pub struct EntityBox<T> {
    inner : Box<Entity<T>>
}

impl<T> EntityBox<T> {
    /// Wraps an Entity into a new EntityBox.
    pub fn new(entity : Box<Entity<T>>) -> Self {
        EntityBox {
            inner : entity
        }
    }
}

impl<T> EntityBox<T> {
    fn on_effect(&self, effect : &T) {
        self.inner.on_effect(effect);
    }
}

#[allow(unsafe_code)]
unsafe impl<T> Send for EntityBox<T> {}

/// Responsible for effect dispatching
pub struct Environment<T> {
    entities : Vec<EntityBox<T>>
}

impl<T> Environment<T> {

    fn new() -> Self {
        Environment {
            entities : Vec::new()
        }
    }

    pub fn add_entity(&mut self, entity : EntityBox<T>) {
        self.entities.push(entity);
    }

    pub fn send_effect(&self, effect : T) {
        for entity in &self.entities {
            entity.on_effect(&effect);
        }
    }
}

use std::sync::{Arc, Mutex};
/// A multiple-access, thread-safe smart pointer, pointing to an environment.
pub struct SharedEnvironment<T> {
    pointer : Arc<Mutex<Environment<T>>>
}

impl<T> SharedEnvironment<T> {
    pub fn new(environment : Environment<T>) -> Self {
        SharedEnvironment{
            pointer : Arc::new(Mutex::new(environment))
        }
    }

    pub fn add_entity(&self, entity : EntityBox<T>) {
        self.pointer.lock().unwrap().add_entity(entity);
    }

    pub fn send_effect(&self, effect : T) {
        self.pointer.lock().unwrap().send_effect(effect);
    }
}

impl<T> std::clone::Clone for SharedEnvironment<T> {
    fn clone(&self) -> Self {
        SharedEnvironment {
            pointer: self.pointer.clone()
        }
    }
}

/// Each instance is responsible for all environments of all supervisors for a custom type.
pub struct TypedSupervisor<'a, T> {
    // option required to allow new() to be const, so TypedSupervisor can be const
    // arc<mutex<>> makes HashMap mutable without unstable feature for mutable references in const fn
    environments : HashMap<(usize, &'a str), SharedEnvironment<T>>
}

impl<'a, T> TypedSupervisor<'a, T> {
    pub fn new() -> Self {
        TypedSupervisor {
            environments : HashMap::new()
        }
    }

    pub fn get_environment(&mut self, session : &EEESession, name : &'a str) -> SharedEnvironment<T> {
        let environment : &SharedEnvironment<T> = self.environments.entry((session.id, name))
            .or_insert(SharedEnvironment::new(Environment::new()));
        environment.clone()
    }
}

pub struct SharedTypedSupervisor<'a, T> {
    pointer : Arc<Mutex<TypedSupervisor<'a, T>>>
}

impl<'a, T> SharedTypedSupervisor<'a, T> {
    pub fn new() -> Self {
        TypedSupervisor::new().into()
    }

    pub fn get_environment(&self, session : &EEESession, name : &'a str) -> SharedEnvironment<T> {
        self.pointer.lock().unwrap().get_environment(session, name)
    }
}

impl<'a, T> From<TypedSupervisor<'a, T>> for SharedTypedSupervisor<'a, T> {
    fn from(typed_supervisor : TypedSupervisor<'a, T>) -> Self {
        SharedTypedSupervisor {
            pointer : Arc::new(Mutex::new(typed_supervisor))
        }
    }
}

/// Just an identification allowing subvisors to differentiate environments belonging to different supervisors.
pub struct EEESession {
    id : usize
}

impl EEESession {

    pub fn new() -> Self {
        // TODO give unique id
        EEESession { id : 0 }
    }
}