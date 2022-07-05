/// A macro that automatically implements the TryFrom trait if the macro has key-value pairs
/// 
/// <https://stackoverflow.com/a/57578431/10521417>
#[macro_export]
macro_rules! back_to_enum {
    ($(#[$meta:meta])* $vis:vis enum $name:ident {
        $($(#[$vmeta:meta])* $vname:ident $(= $val:expr)?,)*
    }) => {
        $(#[$meta])*
        $vis enum $name {
            $($(#[$vmeta])* $vname $(= $val)?,)*
        }

        impl std::convert::TryFrom<u32> for $name {
            type Error = ();

            fn try_from(v: u32) -> Result<Self, Self::Error> {
                match v {
                    $(x if x == $name::$vname as u32 => Ok($name::$vname),)*
                    _ => Err(()),
                }
            }
        }
    }
}

/// A macro that conditionally runs code depending on the feature that is enabled.
/// This is useful when you want to "duck-type" one struct with another for testing
/// purposes
/// 
/// <https://stackoverflow.com/a/72744251/10521417>
#[macro_export]
macro_rules! cfg_match {
    ( other => {$($tt:tt)*} ) => ( $($tt)* );
    ( $cfg:meta => $expansion:tt $(, $($rest:tt)+)? ) => (
        #[cfg($cfg)]
        cfg_match! { other => $expansion }
        $($(
            #[cfg(not($cfg))]
            cfg_match! { other => $rest }
        )?)?
    );
} 