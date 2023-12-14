pub trait Position {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

impl<T> Position for (T, T)
where
    T: Copy + TryInto<i32>,
{
    fn x(&self) -> i32 {
        self.0.try_into().unwrap_or(0)
    }

    fn y(&self) -> i32 {
        self.1.try_into().unwrap_or(0)
    }
}
