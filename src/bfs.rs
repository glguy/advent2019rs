use std::hash::Hash;
use std::collections::{HashSet, VecDeque};

#[derive(Clone)]
pub struct Bfs<S, C, TFn, CFn> {
    queue: VecDeque<S>,
    visited: HashSet<C>,
    transition: TFn,
    characterize: CFn,
}

impl<S, C, TFn, CFn> Bfs<S, C, TFn, CFn>
    where
        S: Clone,
        C: Eq + Hash,
        TFn: Fn(&S) -> Vec<S>,
        CFn: Fn(&S) -> C,
{
    pub fn new(
        initial: S,
        transition: TFn,
        characterize: CFn,
    ) -> Bfs<S, C, TFn, CFn>
    {
        Bfs {
            queue: VecDeque::from(vec![initial]),
            visited: HashSet::new(),
            transition,
            characterize,
        }
    }
}

impl<S, C, TFn, CFn> Iterator for Bfs<S, C, TFn, CFn>
    where
        S: Clone,
        C: Eq + Hash,
        TFn: Fn(&S) -> Vec<S>,
        CFn: Fn(&S) -> C,
{
    type Item = S;

    fn next(&mut self) -> Option<Self::Item> {
        while let Some(s) = self.queue.pop_front() {
            if self.visited.insert((self.characterize)(&s)) {
                self.queue.extend((self.transition)(&s));
                return Some(s);
            }
        }
        None
    }
}