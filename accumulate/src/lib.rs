/// What should the type of _function be?
pub fn map<T, K, F: FnMut(T) -> K>(input: Vec<T>, mut function: F) -> Vec<K> {
    let mut result = Vec::new();

    for element in input {
        result.push(function(element))
    }

    result
}
