pub trait Vec2 {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
}

impl<T> Vec2 for (T, T)
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

pub trait Vec3 {
    fn x(&self) -> i32;
    fn y(&self) -> i32;
    fn z(&self) -> i32;
}

impl<T> Vec3 for (T, T)
where
    T: Copy + TryInto<i32>,
{
    fn x(&self) -> i32 {
        self.0.try_into().unwrap_or(0)
    }

    fn y(&self) -> i32 {
        self.1.try_into().unwrap_or(0)
    }

    fn z(&self) -> i32 {
        0
    }
}

impl<T> Vec3 for (T, T, T)
where
    T: Copy + TryInto<i32>,
{
    fn x(&self) -> i32 {
        self.0.try_into().unwrap_or(0)
    }

    fn y(&self) -> i32 {
        self.1.try_into().unwrap_or(0)
    }

    fn z(&self) -> i32 {
        self.2.try_into().unwrap_or(0)
    }
}
