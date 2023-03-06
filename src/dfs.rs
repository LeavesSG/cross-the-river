use std::{collections::HashMap, hash::Hash};

pub trait Connect<'a, Vert, Iter>
where
    Vert: 'a,
    Iter: Iterator<Item = Vert>,
{
    fn get_adj(&'a self) -> Iter;
}

pub fn dfs<'a, Vert, Iter>(ptr: Vert, target: Vert, visited: &mut HashMap<Vert, Option<Vert>>)
where
    Vert: 'a + Eq + Hash + Connect<'a, Vert, Iter>,
    Iter: Iterator<Item = Vert>,
{
    let adj = ptr.get_adj();
    visited.insert(ptr, None);
    if ptr == target {
        return;
    }

    for next in adj {
        if visited.get(&next).is_none() {
            visited.insert(next, Some(ptr));
            dfs::<Vert, Iter>(next, target, visited);
        }
    }
}

pub fn find_path<'a, T, U, P>(start: T, target: T)
where
    T: 'a + Eq + Hash + Connect<'a, T, U>,
    U: Iterator<Item = T>,
{
    let mut visited = HashMap::new();
    visited.insert(start, None::<T>);
    dfs::<'a, T, U>(start, target, &mut visited);
}
