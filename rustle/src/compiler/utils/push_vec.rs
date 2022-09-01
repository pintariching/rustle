// Pushes all `items` into `vec` using `push`, therefore mutating the vec.

fn push_vec<T>(vec: &mut Vec<T>, items: Vec<T>) {
    vec.extend(items);
}
