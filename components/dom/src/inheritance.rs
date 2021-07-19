pub trait DerivedFrom<T: Castable>: Castable {}

pub trait Castable: Sized {
    fn upcast<T>(&self) -> &T
    where
        T: Castable,
        Self: DerivedFrom<T>,
    {
        unsafe { std::mem::transmute(self) }
    }

    fn downcast<T>(&self) -> &T
    where
        T: DerivedFrom<Self>,
    {
        unsafe { std::mem::transmute(self) }
    }
}
