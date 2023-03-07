mod cross_the_river;
mod dfs;

fn copy<T: Copy>(any: T) -> (T, T) {
    (any, any)
}
fn main() {
    cross_the_river::cross_the_river();
    let s = copy(1);
}
