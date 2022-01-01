/// `Array<'a, T>` is equivalent to `Cow<'a, [T]>`
///
/// This type exists because limitations in rustc prevent Cow<'static, [HtmlChild]> from compiling:
/// https://github.com/rust-lang/rust/issues/23714
#[derive(Debug, Clone, PartialEq, Hash)]
pub enum Array<'a, T> {
    Borrowed(&'a [T]),
    Owned(Vec<T>),
}

impl<'a, T> From<&'a [T]> for Array<'a, T> {
    fn from(a: &'a [T]) -> Self {
        Self::Borrowed(a)
    }
}

impl<T> From<Vec<T>> for Array<'_, T> {
    fn from(a: Vec<T>) -> Self {
        Self::Owned(a)
    }
}
