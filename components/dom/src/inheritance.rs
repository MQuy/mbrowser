use std::rc::Rc;

pub trait DerivedFrom<T: Castable>: Castable {}

pub trait Castable: Sized {
    fn upcast<T>(&self) -> &T
    where
        T: Castable,
        Self: DerivedFrom<T>,
    {
        unsafe { &*(self as *const Self as *const T) }
    }

    fn downcast<T>(&self) -> &T
    where
        T: DerivedFrom<Self>,
    {
        unsafe { &*(self as *const Self as *const T) }
    }
}

pub fn downcast<T: Castable, U: DerivedFrom<T>>(node: Rc<T>) -> Rc<U> {
    unsafe { Rc::from_raw(Rc::into_raw(node) as *const U) }
}

pub fn upcast<T: Castable, U: DerivedFrom<T>>(node: Rc<U>) -> Rc<T> {
    unsafe { Rc::from_raw(Rc::into_raw(node) as *const T) }
}
