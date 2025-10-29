use std::{
    cell::{RefCell, UnsafeCell},
    fmt::Debug,
    rc::Rc,
};

use ahash::{HashMap, HashMapExt};
use ustr::Ustr;

use super::Actor;


thread_local! {
    static ACTOR_REGISTRY: ActorRegistry = ActorRegistry::new();
}

/// Registry for storing actors.
pub struct ActorRegistry {
    actors: RefCell<HashMap<Ustr, Rc<UnsafeCell<dyn Actor>>>>,
}


impl Debug for ActorRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let actors_ref = self.actors.borrow();
        let keys: Vec<&Ustr> = actors_ref.keys().collect();
        f.debug_struct(stringify!(ActorRegistry))
            .field("actors", &keys)
            .finish()
    }
}

impl Default for ActorRegistry {
    fn default() -> Self {
        Self::new()
    }
}


impl ActorRegistry {
    pub fn new() -> Self {
        Self {
            actors: RefCell::new(HashMap::new()),
        }
    }

    pub fn insert(&self, id: Ustr, actor: Rc<UnsafeCell<dyn Actor>>) {
        let mut actors = self.actors.borrow_mut();
        if actors.contains_key(&id) {
            log::warn!("Replacing existing actor with id: {id}");
        }
        actors.insert(id, actor); //保存对应的 actor 到 actorRegistry中
    }

    pub fn get(&self, id: &Ustr) -> Option<Rc<UnsafeCell<dyn Actor>>> {
        self.actors.borrow().get(id).cloned()
    }

    /// Returns the number of registered actors.
    pub fn len(&self) -> usize {
        self.actors.borrow().len()
    }

    /// Checks if the registry is empty.
    pub fn is_empty(&self) -> bool {
        self.actors.borrow().is_empty()
    }

    /// Removes an actor from the registry.
    pub fn remove(&self, id: &Ustr) -> Option<Rc<UnsafeCell<dyn Actor>>> {
        self.actors.borrow_mut().remove(id)
    }

    /// Checks if an actor with the `id` exists.
    pub fn contains(&self, id: &Ustr) -> bool {
        self.actors.borrow().contains_key(id)
    }
}

pub fn get_actor_registry() -> &'static ActorRegistry {
    ACTOR_REGISTRY.with(|registry| unsafe {
        // SAFETY: We return a static reference that lives for the lifetime of the thread.
        // Since this is thread_local storage, each thread has its own instance.
        // The transmute extends the lifetime to 'static which is safe because
        // thread_local ensures the registry lives for the thread's entire lifetime.
        std::mem::transmute::<&ActorRegistry, &'static ActorRegistry>(registry)
    })
}


/// Registers an actor. 单线程内，不安全的变更方式
pub fn register_actor<T>(actor: T) -> Rc<UnsafeCell<T>>
where
    T: Actor + 'static,
{
    let actor_id = actor.id();
    let actor_ref = Rc::new(UnsafeCell::new(actor)); // 创建 actor的应用

    // Register as Actor (message handling only) 只能使用使用1次， 所以需要用 clone()方法
    let actor_trait_ref: Rc<UnsafeCell<dyn Actor>> = actor_ref.clone(); // 复制
    get_actor_registry() // 创建一个默认的 actorRegistry, 可以存多个 actor
        .insert(actor_id, actor_trait_ref); // 保存 actor

    actor_ref
}

pub fn get_actor(id: &Ustr) -> Option<Rc<UnsafeCell<dyn Actor>>> {
    get_actor_registry().get(id)
}

/// Returns a mutable reference to the registered actor of type `T` for the `id`.
///
/// # Safety
///
/// This function bypasses Rust's borrow checker and type safety.
/// Caller must ensure:
/// - Actor with `id` exists in registry.
/// - No other mutable references to the same actor exist.
/// - Type `T` matches the actual actor type.
///
/// # Panics
///
/// Panics if no actor with the specified `id` is found in the registry.
#[allow(clippy::mut_from_ref)]
pub fn get_actor_unchecked<T: Actor>(id: &Ustr) -> &mut T {
    let actor = get_actor(id).unwrap_or_else(|| panic!("Actor for {id} not found"));
    // SAFETY: Caller must ensure no aliasing and correct type 解析出对应可变的类型
    unsafe { &mut *(actor.get() as *mut _ as *mut T) }
}

/// Safely attempts to get a mutable reference to the registered actor.
///
/// Returns `None` if the actor is not found, avoiding panics.
#[allow(clippy::mut_from_ref)]
pub fn try_get_actor_unchecked<T: Actor>(id: &Ustr) -> Option<&mut T> {
    let actor = get_actor(id)?;
    // SAFETY: Registry guarantees valid actor pointers
    Some(unsafe { &mut *(actor.get() as *mut _ as *mut T) })
}

/// Checks if an actor with the `id` exists in the registry.
pub fn actor_exists(id: &Ustr) -> bool {
    get_actor_registry().contains(id)
}

/// Returns the number of registered actors.
pub fn actor_count() -> usize {
    get_actor_registry().len()
}

#[cfg(test)]
/// Clears the actor registry (for test isolation).
pub fn clear_actor_registry() {
    // SAFETY: Clearing registry actors; tests should run single-threaded for actor registry
    get_actor_registry().actors.borrow_mut().clear();
}
