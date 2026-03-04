use std::borrow::Cow;

pub fn normalize_for_compare<'a>(bytes: &'a [u8]) -> Cow<'a, [u8]> {
    Cow::Borrowed(bytes)
}
