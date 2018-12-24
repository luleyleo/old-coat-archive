/// A trait that has a single function `name` that returns a `struct`s name.
/// 
/// ```
/// use coat::Named;
/// struct SomeStruct;
/// impl Named for SomeStruct {
///     fn name() -> &'static str {
///         "SomeStruct"
///     }
/// }
/// ```
/// 
/// This allows printing a "path" when there is an error
/// like: `/Root/Container/AddButton`
/// It is automatically implement for `Iid`s when they are
/// created by the `ids!()` macro.
pub trait Named {
    fn name() -> &'static str;
}
