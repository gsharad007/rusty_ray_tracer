use core::array::IntoIter;
use core::iter::Zip;
use core::slice::Iter;
use core::slice::IterMut;

pub trait ArrayBase: Sized {
    type Item;
    // type SizedArray;

    /// Returns base array consuming
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], tuple.get_array());
    /// ```
    #[must_use]
    fn get_array(self) -> [Self::Item; 4];

    /// Returns base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], *tuple.get_array_ref());
    /// ```
    #[must_use]
    fn get_array_ref(&self) -> &[Self::Item; 4];

    /// Returns a mutable base array reference
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// let mut tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!([1.0, 2.0, 3.0, 4.0], *tuple.get_array_mut());
    /// tuple.get_array_mut()[0] += 10.0;
    /// tuple.get_array_mut()[1] += 10.0;
    /// tuple.get_array_mut()[2] += 10.0;
    /// tuple.get_array_mut()[3] += 10.0;
    /// assert_eq!([11.0, 12.0, 13.0, 14.0], *tuple.get_array_mut());
    /// ```
    #[must_use]
    fn get_array_mut(&mut self) -> &mut [Self::Item; 4];

    /// Creates an iterator from a value.
    ///
    /// See the [module-level documentation] for more.
    ///
    /// [module-level documentation]: crate::iter
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!(
    ///     vec![1.0, 2.0, 3.0, 4.0],
    ///     tuple.clone().into_iter().collect::<Vec<_>>()
    /// );
    /// let mut it = tuple.into_iter();
    /// assert_eq!(Some(1.0), it.next());
    /// assert_eq!(Some(2.0), it.next());
    /// assert_eq!(Some(3.0), it.next());
    /// assert_eq!(Some(4.0), it.next());
    /// assert_eq!(None, it.next());
    /// ```
    #[must_use]
    fn into_iter(self) -> IntoIter<Self::Item, 4_usize> {
        self.get_array().into_iter()
    }

    /// Returns an iterator over the slice.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// let tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// assert_eq!(
    ///     vec![&1.0, &2.0, &3.0, &4.0],
    ///     tuple.iter().collect::<Vec<_>>()
    /// );
    /// let mut it = tuple.iter();
    /// assert_eq!(Some(&1.0), it.next());
    /// assert_eq!(Some(&2.0), it.next());
    /// assert_eq!(Some(&3.0), it.next());
    /// assert_eq!(Some(&4.0), it.next());
    /// assert_eq!(None, it.next());
    /// ```
    #[must_use]
    fn iter(&self) -> Iter<'_, Self::Item> {
        self.get_array_ref().iter()
    }

    /// Returns an iterator that allows modifying each value.
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// let mut tuple = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// tuple.iter_mut().for_each(|a| *a += 10.0);
    /// assert_eq!(
    ///     vec![&11.0, &12.0, &13.0, &14.0],
    ///     tuple.iter_mut().collect::<Vec<_>>()
    /// );
    /// let mut it = tuple.iter();
    /// assert_eq!(Some(&11.0), it.next());
    /// assert_eq!(Some(&12.0), it.next());
    /// assert_eq!(Some(&13.0), it.next());
    /// assert_eq!(Some(&14.0), it.next());
    /// assert_eq!(None, it.next());
    /// ```
    #[must_use]
    fn iter_mut(&mut self) -> IterMut<'_, Self::Item> {
        self.get_array_mut().iter_mut()
    }

    /// Creates an iterator from zipping both Tuples iterators into one using a closure consuming them both
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// let b = Tuple::new(4.0, 3.0, 2.0, 1.0);
    /// assert_eq!(
    ///     vec![(1.0, 4.0), (2.0, 3.0), (3.0, 2.0), (4.0, 1.0)],
    ///     a.clone().into_zip(b.clone()).collect::<Vec<_>>()
    /// );
    /// let mut it = a.into_zip(b);
    /// assert_eq!(Some((1.0, 4.0)), it.next());
    /// assert_eq!(Some((2.0, 3.0)), it.next());
    /// assert_eq!(Some((3.0, 2.0)), it.next());
    /// assert_eq!(Some((4.0, 1.0)), it.next());
    /// assert_eq!(None, it.next());
    /// ```
    fn into_zip(
        self,
        other: Self,
    ) -> std::iter::Zip<
        std::array::IntoIter<Self::Item, 4_usize>,
        std::array::IntoIter<Self::Item, 4_usize>,
    > {
        self.into_iter().zip(other.into_iter())
    }

    /// Creates an iterator from zipping both Tuples iterators into one using a closure by reference
    ///
    /// # Examples
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// let b = Tuple::new(4.0, 3.0, 2.0, 1.0);
    /// assert_eq!(
    ///     vec![(&1.0, &4.0), (&2.0, &3.0), (&3.0, &2.0), (&4.0, &1.0)],
    ///     a.zip(&b).collect::<Vec<_>>()
    /// );
    /// let mut it = a.zip(&b);
    /// assert_eq!(Some((&1.0, &4.0)), it.next());
    /// assert_eq!(Some((&2.0, &3.0)), it.next());
    /// assert_eq!(Some((&3.0, &2.0)), it.next());
    /// assert_eq!(Some((&4.0, &1.0)), it.next());
    /// assert_eq!(None, it.next());
    /// ```
    fn zip<'a, 'b>(&'a self, other: &'b Self) -> Zip<Iter<'a, Self::Item>, Iter<'b, Self::Item>> {
        self.iter().zip(other.iter())
    }

    /// Combines both Tuples into one using a closure
    ///
    /// # Examples
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::tuples::Tuple;
    /// let a = Tuple::new(1.0, 2.0, 3.0, 4.0);
    /// let b = Tuple::new(4.0, 3.0, 2.0, 1.0);
    /// assert_eq!(
    ///     [5.0, 5.0, 5.0, 5.0],
    ///     Tuple::zip_for_each_collect(a, b, |a, b| a + b).get_array()
    /// );
    /// ```
    #[must_use]
    fn zip_for_each_collect(
        self,
        other: Self,
        f: impl Fn(Self::Item, Self::Item) -> Self::Item,
    ) -> Self
    where
        Self::Item: Copy,
    {
        let mut result = self;
        result
            .iter_mut()
            .zip(other.into_iter())
            .for_each(|(i, j)| *i = f(*i, j));
        result
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Clone)]
    pub struct Tuple {
        coords: [f32; 4],
    }
    impl ArrayBase for Tuple {
        type Item = f32;
        // type SizedArray = [f32; 4];

        fn get_array(self) -> [f32; 4] {
            self.coords
        }

        fn get_array_ref(&self) -> &[f32; 4] {
            &self.coords
        }

        fn get_array_mut(&mut self) -> &mut [f32; 4] {
            &mut self.coords
        }
    }

    #[test]
    fn get_array() {
        let tuple = Tuple {
            coords: [1.0, 2.0, 3.0, 4.0],
        };
        assert_eq!([1.0, 2.0, 3.0, 4.0], *tuple.get_array_ref());
        assert_eq!([1.0, 2.0, 3.0, 4.0], tuple.clone().get_array());
        assert_eq!([1.0, 2.0, 3.0, 4.0], *tuple.get_array_ref());
    }

    #[test]
    fn get_array_mut() {
        let mut tuple = Tuple {
            coords: [1.0, 2.0, 3.0, 4.0],
        };
        assert_eq!([1.0, 2.0, 3.0, 4.0], *tuple.get_array_mut());
        tuple.get_array_mut()[0] += 10.0;
        tuple.get_array_mut()[1] += 10.0;
        tuple.get_array_mut()[2] += 10.0;
        tuple.get_array_mut()[3] += 10.0;
        assert_eq!([11.0, 12.0, 13.0, 14.0], *tuple.get_array_mut());
        assert_eq!([11.0, 12.0, 13.0, 14.0], tuple.clone().get_array());
        assert_eq!([11.0, 12.0, 13.0, 14.0], *tuple.get_array_ref());
    }

    #[test]
    fn get_array_default() {
        let tuple: Tuple = Tuple {
            coords: Default::default(),
        };
        assert_eq!([0.0, 0.0, 0.0, 0.0], tuple.get_array());
    }

    #[test]
    fn into_iter() {
        let tuple = Tuple {
            coords: [1.0, 2.0, 3.0, 4.0],
        };
        assert_eq!(
            vec![1.0, 2.0, 3.0, 4.0],
            tuple.clone().into_iter().collect::<Vec<_>>()
        );
        let mut it = tuple.into_iter();
        assert_eq!(Some(1.0), it.next());
        assert_eq!(Some(2.0), it.next());
        assert_eq!(Some(3.0), it.next());
        assert_eq!(Some(4.0), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn iter() {
        let tuple = Tuple {
            coords: [1.0, 2.0, 3.0, 4.0],
        };
        assert_eq!(
            vec![&1.0, &2.0, &3.0, &4.0],
            tuple.iter().collect::<Vec<_>>()
        );
        let mut it = tuple.iter();
        assert_eq!(Some(&1.0), it.next());
        assert_eq!(Some(&2.0), it.next());
        assert_eq!(Some(&3.0), it.next());
        assert_eq!(Some(&4.0), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn iter_mut() {
        let mut tuple = Tuple {
            coords: [1.0, 2.0, 3.0, 4.0],
        };
        tuple.iter_mut().for_each(|a| *a += 10.0);
        assert_eq!(
            vec![&11.0, &12.0, &13.0, &14.0],
            tuple.iter_mut().collect::<Vec<_>>()
        );
        let mut it = tuple.iter();
        assert_eq!(Some(&11.0), it.next());
        assert_eq!(Some(&12.0), it.next());
        assert_eq!(Some(&13.0), it.next());
        assert_eq!(Some(&14.0), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn into_zip() {
        let a = Tuple {
            coords: [1.0, 2.0, 3.0, 4.0],
        };
        let b = Tuple {
            coords: [4.0, 3.0, 2.0, 1.0],
        };
        assert_eq!(
            vec![(1.0, 4.0), (2.0, 3.0), (3.0, 2.0), (4.0, 1.0)],
            a.clone().into_zip(b.clone()).collect::<Vec<_>>()
        );
        let mut it = a.into_zip(b);
        assert_eq!(Some((1.0, 4.0)), it.next());
        assert_eq!(Some((2.0, 3.0)), it.next());
        assert_eq!(Some((3.0, 2.0)), it.next());
        assert_eq!(Some((4.0, 1.0)), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn zip() {
        let a = Tuple {
            coords: [1.0, 2.0, 3.0, 4.0],
        };
        let b = Tuple {
            coords: [4.0, 3.0, 2.0, 1.0],
        };
        assert_eq!(
            vec![(&1.0, &4.0), (&2.0, &3.0), (&3.0, &2.0), (&4.0, &1.0)],
            a.zip(&b).collect::<Vec<_>>()
        );
        let mut it = a.zip(&b);
        assert_eq!(Some((&1.0, &4.0)), it.next());
        assert_eq!(Some((&2.0, &3.0)), it.next());
        assert_eq!(Some((&3.0, &2.0)), it.next());
        assert_eq!(Some((&4.0, &1.0)), it.next());
        assert_eq!(None, it.next());
    }

    #[test]
    fn zip_for_each_collect() {
        let a = Tuple {
            coords: [1.0, 2.0, 3.0, 4.0],
        };
        let b = Tuple {
            coords: [4.0, 3.0, 2.0, 1.0],
        };
        assert_eq!(
            [5.0, 5.0, 5.0, 5.0],
            Tuple::zip_for_each_collect(a, b, |a, b| a + b).get_array()
        );
    }
}
