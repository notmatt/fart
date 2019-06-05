/// User-defined, runtime "constants".
///
///
///
/// ```
/// user_const! {
///     const NUMBER_OF_PARTICLES: usize = 1234;
/// }
/// ```
#[macro_export]
macro_rules! user_const {
    (
        $(
            const $name:ident : $ty:ty = $default:expr ;
        )*
    ) => {
        fart::prelude::lazy_static! { $(
            static ref $name: $ty = {
                use std::{env, fmt::Debug, str::FromStr};

                fn types_used_with_user_const_must_impl_FromStr<T: FromStr>() {}
                types_used_with_user_const_must_impl_FromStr::<$ty>();

                fn types_used_with_user_const_must_impl_Debug<T: Debug>() {}
                types_used_with_user_const_must_impl_FromStr::<$ty>();

                let env_var_name = concat!("FART_USER_CONST_", stringify!($name));
                let value = match env::var(env_var_name) {
                    Err(_) => $default,
                    Ok(s) => {
                        s.parse().expect(
                            &format!(
                                "Parsing user const `{}` from {:?} failed",
                                stringify!($name),
                                s
                            )
                        )
                    }
                };

                eprintln!(
                    "fart: const {}: {} = {:?};",
                    stringify!($name),
                    stringify!($ty),
                    value
                );

                value
            };
        )* }
    };
}
