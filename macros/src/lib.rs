#[macro_export]
macro_rules! hashmap {
    ( $( $key:expr => $value:expr ),* ) => {
        {
            let mut map = ::std::collections::HashMap::new();
            $( map.insert($key, $value); )*
            map
        }
    };
    ( $( $key:expr => $value:expr, )* ) => {
        macros::hashmap!( $( $key => $value ),* )
    };
}