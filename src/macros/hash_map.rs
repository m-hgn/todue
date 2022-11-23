#[macro_export]
macro_rules! hash_map {
    { $($key:expr => $value:expr),+ } => {{
        let mut m = HashMap::default();
        $( m.insert($key, $value); )+
        m
    }};
}
