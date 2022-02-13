#[macro_export]
macro_rules! Red {
    () => {
        Fixed(197)
    };
}

#[macro_export]
macro_rules! LightRed {
    () => {
        Fixed(198)
    };
}

#[macro_export]
macro_rules! Yellow {
    () => {
        Fixed(220)
    };
}

#[macro_export]
macro_rules! LightYellow {
    () => {
        Fixed(221)
    };
}

#[macro_export]
macro_rules! Grey {
    () => {
        Fixed(245)
    };
}

#[macro_export]
macro_rules! Cyan {
    () => {
        Fixed(69)
    };
}

pub use {Cyan, /*Yellow, LightYellow,*/ Grey, LightRed, Red};
