pub mod arena;

pub use arena::*;

#[cfg(test)]
mod tests {
    use crate::{EntityHandler, Arena, ArenaScope, arena};

    struct LinkedList<'s> {
        ends: Option<(EntityHandler<'s>, EntityHandler<'s>)>,
        len: usize,
    }

    #[derive(Copy, Clone, Debug)]
    struct Node<'s, T> {
        value: T,
        prev: Option<EntityHandler<'s>>,
        next: Option<EntityHandler<'s>>,
    }

    impl<'s> LinkedList<'s> {
        fn new() -> Self {
            Self {
                ends: None,
                len: 0,
            }
        }

        fn push_front<'a, T>(&mut self, elem: T, arena: &'a mut Arena<'s,Node<'s, T>>) {
            let node_handler = arena.alloc(Node::<'s, T> {
                value: elem,
                prev: None,
                next: None
            });

            if let Some((head_handler, _)) = &mut self.ends {
                node_handler.get_mut(arena).next = Some(*head_handler);
                head_handler.get_mut(arena).prev = Some(node_handler);
                *head_handler = node_handler;
            } else {
                self.ends = Some((node_handler, node_handler))
            }
        }

        fn pop_front<'a, T>(&mut self, arena: &'a mut Arena<'s, Node<'s, T>>) -> Option<&'a mut Node<'s, T>> {
            if let Some((head_handler, tail_handler)) = &mut self.ends {
                let res = *head_handler;
                if head_handler == tail_handler {
                    self.ends = None
                } else {
                    *head_handler = head_handler.get_ref(arena).next.unwrap()
                }
                Some(res.get_mut(arena))
            } else {
                None
            }
        }
    }
    #[test]
    fn it_works() {
        arena::scoped_arena(PushPop);
    }


    fn push_pop<'s>(arena: &mut Arena<'s, Node<'s, i32>>) {
        let mut ls = LinkedList::new();
        ls.push_front(1, arena);
        ls.push_front(2, arena);

        assert_eq!(ls.pop_front(arena).map(|n| n.value), Some(2));
        assert_eq!(ls.pop_front(arena).map(|n| n.value), Some(1));
        assert_eq!(ls.pop_front(arena).map(|n| n.value), None);
    }

    struct PushPop;
    impl<'s> ArenaScope<'s, ()> for PushPop {
        type Entity = Node<'s, i32>;

        fn scoped(self, mut arena: Arena<'s, Node<'s, i32>>) {
            push_pop(&mut arena)
        }
    }
}
