#[macro_export]
macro_rules! direct_into_build {
    ($t:ty, $builder:ty => $out:ty) => {
        impl Into<$out> for $t {
            fn into(self) -> $out {
                Into::<$builder>::into(self).build()
            }
        }
    };
}
