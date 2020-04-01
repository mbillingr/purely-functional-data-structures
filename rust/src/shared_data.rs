pub use impl_::Share;

#[cfg(not(feature = "leaky"))]
mod impl_ {
    use std::ops::Deref;
    use std::rc::Rc;

    #[derive(Clone)]
    pub struct Share<T>(Rc<T>);

    impl<T> Share<T> {
        pub fn new(x: T) -> Self {
            Share(Rc::new(x))
        }

        pub fn ptr_eq(lhs: &Self, rhs: &Self) -> bool {
            Rc::ptr_eq(&lhs.0, &rhs.0)
        }
    }

    impl<T> Deref for Share<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &*self.0
        }
    }

    impl<T: PartialEq> PartialEq for Share<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
}

#[cfg(feature = "leaky")]
mod impl_ {
    use std::ops::Deref;

    #[derive(Clone)]
    pub struct Share<T: 'static>(&'static T);

    impl<T> Share<T> {
        pub fn new(x: T) -> Self {
            Share(Box::leak(Box::new(x)))
        }

        pub fn ptr_eq(lhs: &Self, rhs: &Self) -> bool {
            std::ptr::eq(&lhs.0, &rhs.0)
        }
    }

    impl<T> Deref for Share<T> {
        type Target = T;
        fn deref(&self) -> &T {
            &*self.0
        }
    }

    impl<T: PartialEq> PartialEq for Share<T> {
        fn eq(&self, other: &Self) -> bool {
            self.0 == other.0
        }
    }
}
