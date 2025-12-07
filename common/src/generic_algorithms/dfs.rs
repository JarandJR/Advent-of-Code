use std::{collections::HashMap, hash::Hash};

pub fn dfs<S, V, BaseFn, NextFn, CombineFn>(
    state: S,
    memo: &mut HashMap<S, V>,
    base: &BaseFn,
    next: &NextFn,
    combine: &CombineFn,
) -> V
where
    S: Eq + Hash + Clone,
    V: Clone + Default,
    BaseFn: Fn(&S) -> Option<V>,
    NextFn: Fn(&S) -> Vec<S>,
    CombineFn: Fn(V, V) -> V,
{
    if let Some(cached) = memo.get(&state) {
        return cached.clone();
    }

    if let Some(value) = base(&state) {
        return value;
    }

    let mut result = V::default();
    for s in next(&state) {
        let val = dfs(s, memo, base, next, combine);
        result = combine(result, val);
    }

    memo.insert(state.clone(), result.clone());
    result
}
