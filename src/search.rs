use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;

pub trait SearchSpec {
    type SearchState;
    fn branch(&self, state: &Self::SearchState) -> Vec<Self::SearchState>;
    fn is_goal(&self, state: &Self::SearchState) -> bool;
}

pub fn best_first_search<T>(searcher: T, init_state: T::SearchState) -> Option<T::SearchState>
where
    T: SearchSpec,
    T::SearchState: Eq + Ord + Hash,
{
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(init_state));
    let mut visited = HashSet::new();
    while let Some(Reverse(st)) = queue.pop() {
        if searcher.is_goal(&st) { return Some(st); }
        if !visited.contains(&st) {
            queue.extend(searcher.branch(&st).into_iter().map(|child| Reverse(child)));
            visited.insert(st);
        }
    }
    None
}
