/// Takes a list of expressions and calls `HtmlChild::from` on each of them
#[macro_export]
macro_rules! ch {
    [$($ch:expr),* $(,)?] => {
        vec![
            $($crate::HtmlChild::from($ch)),*
        ]
    };
}

/// Takes a list of
#[macro_export]
macro_rules! attrs {
    ($($name:ident : $value:expr),* $(,)?) => {
        vec![
            $($crate::HtmlAttribute {
                name: stringify!($name).into(),
                value: $value.into(),
            }),*
        ]
    };
}
