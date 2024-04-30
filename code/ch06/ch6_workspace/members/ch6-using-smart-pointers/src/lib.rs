pub fn is_strong<T: AsRef<str>>(password: T) -> bool {
    password.as_ref().len() > 5
}

pub fn is_strong2<T>(password: T) -> bool
where
    T: AsRef<str>,
{
    password.as_ref().len() > 5
}

pub fn is_strong3<T>(password: T) -> bool
where
    T: Into<String>,
{
    password.into().len() > 5
}
