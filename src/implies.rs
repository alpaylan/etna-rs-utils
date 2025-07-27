pub trait Implies<T> {
    fn implies<F>(&self, other: F) -> Option<bool>
    where
        F: Fn() -> T;
}

impl Implies<bool> for bool {
    fn implies<F>(&self, other: F) -> Option<bool>
    where
        F: Fn() -> bool,
    {
        match self {
            true => Some(other()),
            false => None, // discard false
        }
    }
}

impl Implies<bool> for Option<bool> {
    fn implies<F>(&self, other: F) -> Option<bool>
    where
        F: Fn() -> bool,
    {
        match self {
            Some(true) => Some(other()),
            Some(false) | None => None,
        }
    }
}

impl Implies<Option<bool>> for bool {
    fn implies<F>(&self, other: F) -> Option<bool>
    where
        F: Fn() -> Option<bool>,
    {
        match self {
            true => other(),
            false => None, // discard false
        }
    }
}

impl Implies<Option<bool>> for Option<bool> {
    fn implies<F>(&self, other: F) -> Option<bool>
    where
        F: Fn() -> Option<bool>,
    {
        match self {
            Some(true) => other(),
            Some(false) | None => None,
        }
    }
}