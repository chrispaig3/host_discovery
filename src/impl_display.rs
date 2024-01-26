// impl_display: Implements the Display trait for OperatingSystem and Architecture
#[macro_export]
macro_rules! impl_display {
    ($type:ident) => {
        impl Display for $type {
            fn fmt(&self, f: &mut Formatter<'_>) -> Result {
                write!(f, "{:?}", self)
            }
        }
    };
}
