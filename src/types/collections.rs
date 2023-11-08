pub mod enumset {
    use std::{collections::HashSet, marker::PhantomData};

    use num_traits::int::PrimInt;
    use strum::IntoEnumIterator;

    pub trait SuitableNumber: PrimInt + Default + Copy {}
    impl<T> SuitableNumber for T where T: PrimInt + Default + Copy {}

    pub trait SuitableEnum<T: SuitableNumber>: Into<T> + IntoEnumIterator + Copy {}
    impl<T, Y> SuitableEnum<Y> for T
    where
        T: Into<Y> + IntoEnumIterator + Copy,
        Y: SuitableNumber,
    {
    }

    #[derive(Clone, Default)]
    pub struct EnumSet<K, T>
    where
        T: SuitableNumber,
        K: SuitableEnum<T>,
    {
        data: T,
        _dat: PhantomData<K>,

        // iterator data
        iter_data: Option<K::Iterator>,
    }

    impl<K, T> EnumSet<K, T>
    where
        T: SuitableNumber,
        K: SuitableEnum<T>,
    {
        pub fn new() -> Self {
            Self {
                data: T::zero(),
                _dat: PhantomData::default(),
                iter_data: None,
            }
        }

        pub fn contains(&self, x: K) -> bool {
            (self.data & K::into(x)) > T::zero()
        }
        pub fn insert(&mut self, x: K) {
            self.data = self.data | K::into(x);
        }
        pub fn remove(&mut self, x: K) {
            self.data = self.data & !(K::into(x));
        }

        pub fn repr(&self) -> T {
            self.data
        }

        pub fn iter(&self) -> EnumSetIter<'_, K, T> {
            EnumSetIter {
                reference: self,
                last_iter: K::iter(),
            }
        }
    }

    impl<K, T> From<T> for EnumSet<K, T>
    where
        T: SuitableNumber,
        K: SuitableEnum<T>,
    {
        fn from(value: T) -> Self {
            Self {
                data: value,
                _dat: PhantomData::default(),
                iter_data: None,
            }
        }
    }

    pub struct EnumSetIter<'a, K, T>
    where
        T: SuitableNumber,
        K: SuitableEnum<T>,
    {
        reference: &'a EnumSet<K, T>,
        last_iter: K::Iterator,
    }

    impl<'a, K, T> Iterator for EnumSetIter<'a, K, T>
    where
        T: SuitableNumber,
        K: SuitableEnum<T>,
    {
        type Item = K;

        fn next(&mut self) -> Option<Self::Item> {
            while let Some(curr_enum_variant) = self.last_iter.next() {
                if self.reference.contains(curr_enum_variant) {
                    return Some(curr_enum_variant);
                }
            }
            None
        }
    }

    impl<K, T> std::ops::BitAnd<EnumSet<K, T>> for EnumSet<K, T>
    where
        T: SuitableNumber,
        K: SuitableEnum<T>,
    {
        type Output = Self;

        fn bitand(mut self, rhs: EnumSet<K, T>) -> Self::Output {
            for item in rhs.iter() {
                self.insert(item);
            }
            self
        }
    }

    #[cfg(test)]
    mod tests {
        use strum::EnumIter;

        use super::EnumSet;

        #[repr(u32)]
        #[derive(EnumIter, Clone, Copy)]
        enum Scalar {
            First = 1 << 1,
            Second = 1 << 2,
            Third = 1 << 3,
            Fourth = 1 << 4,
            Fifth = 1 << 5,
            Sixth = 1 << 6,
        }
        impl Into<u32> for Scalar {
            fn into(self) -> u32 {
                self as u32
            }
        }

        type Set = EnumSet<Scalar, u32>;

        fn construct() -> Set {
            let test_val = Scalar::First as u32 | Scalar::Second as u32;
            let set = Set::from(test_val);
            set
        }

        #[test]
        pub fn test_new() {
            let _ = construct();
        }

        #[test]
        pub fn test_insert() {
            let mut set = construct();
            set.insert(Scalar::Fifth);
            use Scalar as S;

            assert_eq!(
                set.repr(),
                (S::First as u32) | (S::Second as u32) | (S::Fifth as u32)
            );
        }

        #[test]
        pub fn test_remove() {
            let mut set = construct();
            set.remove(Scalar::First);

            assert_eq!(set.repr(), Scalar::Second as u32);
            let as_vec = set.iter().collect::<Vec<_>>();
            assert_eq!(set.iter_data().collect::<Vec<Scalar>>::(), vec![Scalar::First]);
        }
    }
}
