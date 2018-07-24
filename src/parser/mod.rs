#[macro_use]
mod macros {
    #[macro_export]
    macro_rules! cast_to_type {
        ($($x:expr, $y:expr),*) => {
            {
                $(
                    let record = $x::from($y?);
                )*
                record
            }

        }
    }
}

pub mod gtfs;
