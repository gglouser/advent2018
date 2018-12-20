use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};
use std::hash::Hash;

// SearchSpec State and Token
//
// The best_first_search function will examine nodes in order of least cost.
// Rather than impose an arbitrary data type for cost, the order is
// determined by the Ord implementation of the associated State type.
//
// We also want to remember which states we have visited already. We could
// require an Eq + Hash bound on State in addition to Ord, but there
// may be states that are equivalent but have different costs. For example,
// in pathfinding, two states might refer to the same position but reach
// it by different paths and therefore have different costs. In that case,
// the states would be equal according to Eq but not equal according to Ord.
// To avoid Eq/Ord conflicts, states are memoized by their Token.

pub trait SearchSpec {
    type State;
    type Token;
    fn branch(&self, state: &Self::State) -> Vec<Self::State>;
    fn is_goal(&self, state: &Self::State) -> bool;
    fn token(&self, state: &Self::State) -> Self::Token;
}

pub fn best_first_search<T>(searcher: T, init_state: T::State) -> Option<T::State>
where
    T: SearchSpec,
    T::State: Ord,
    T::Token: Eq + Hash,
{
    let mut queue = BinaryHeap::new();
    queue.push(Reverse(init_state));
    let mut visited = HashSet::new();
    while let Some(Reverse(st)) = queue.pop() {
        if searcher.is_goal(&st) { return Some(st); }
        if visited.insert(searcher.token(&st)) {
            queue.extend(searcher.branch(&st).into_iter().map(|child| Reverse(child)));
        }
    }
    None
}
