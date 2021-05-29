use std::marker::PhantomData;

#[derive(Debug)]
pub struct Arena<'s, T> {
    inner: Vec<T>,
    _marker: PhantomData<*mut &'s ()>,
}


#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct EntityHandler<'s> {
    addr: usize,
    _marker: PhantomData<*mut &'s ()>,
}



impl<'s, T> Arena<'s, T> {
    pub fn alloc(&mut self, x: T) -> EntityHandler<'s>  {
        self.inner.push(x);
        EntityHandler {
            addr: self.inner.len() - 1,
            _marker: Default::default()
        }
    }

    pub fn pre_alloc(&mut self) -> EntityHandler<'static> {
        self.inner.push(unsafe { std::mem::MaybeUninit::uninit().assume_init() });
        EntityHandler {
            addr: self.inner.len() - 1,
            _marker: Default::default()
        }
    }

    pub fn init_with(&mut self, vacant: EntityHandler<'static>, init: impl FnOnce(EntityHandler<'s>) -> T) -> EntityHandler<'s>  {
        let handler = EntityHandler {
            addr: vacant.addr,
            _marker: Default::default()
        };
        self.inner[vacant.addr] = init(handler);
        handler
    }
}

impl<'s> EntityHandler<'s> {
    pub fn get_ref<'a, T>(self, arena: &'a Arena<'s, T>) -> &'a T {
        unsafe { arena.inner.get_unchecked(self.addr) }
    }

    pub fn get_mut<'a, T>(self, arena: &'a mut Arena<'s, T>) -> &'a mut T {
        unsafe { arena.inner.get_unchecked_mut(self.addr) }
    }
}

pub trait ArenaScope<'s, R> {
    type Entity;
    fn scoped(self, arena: Arena<'s, Self::Entity>) -> R;
}

impl<'a, 's, T, R> ArenaScope<'s, R> for Box<dyn FnOnce(Arena<'s, T>) -> R + 'a>
{
    type Entity = T;

    fn scoped(self, arena: Arena<'s, T>) -> R {
        self(arena)
    }
}


pub fn scoped_arena<R>(scoped: impl for<'a> ArenaScope<'a, R>) -> R {
    scoped.scoped(Arena {
        inner: Default::default(),
        _marker: Default::default()
    })
}
