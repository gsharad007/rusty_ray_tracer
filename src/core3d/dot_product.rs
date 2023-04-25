use super::array_base::ArrayBase;

pub trait DotProduct: ArrayBase<Item = f32> {
    /// Calculate Dot Product on two `ArrayBasedStructs`
    ///
    /// # Example
    ///
    /// ```
    /// # use crate::rusty_ray_tracer::core3d::array_base::ArrayBase;
    /// # use crate::rusty_ray_tracer::core3d::array_base::*;
    /// # use crate::rusty_ray_tracer::core3d::dot_product::DotProduct;
    /// # #[derive(Clone)]
    /// # struct ArrayBasedStruct([f32; 4]);
    /// # impl ArrayBase for ArrayBasedStruct {
    /// #     type Item = f32;
    /// #     fn get_array    (self     ) ->      [f32; 4] { self.0      }
    /// #     fn get_array_ref(&self    ) -> &    [f32; 4] { &self.0     }
    /// #     fn get_array_mut(&mut self) -> &mut [f32; 4] { &mut self.0 }
    /// # }
    /// # impl DotProduct for ArrayBasedStruct {}
    /// assert_eq!(0.0, ArrayBasedStruct([0.0, 0.0, 0.0, 0.0]).dot(ArrayBasedStruct([0.0, 0.0, 0.0, 0.0])));
    /// assert_eq!(1.0, ArrayBasedStruct([1.0, 0.0, 0.0, 0.0]).dot(ArrayBasedStruct([1.0, 0.0, 0.0, 0.0])));
    /// assert_eq!(1.0, ArrayBasedStruct([0.0, 1.0, 0.0, 0.0]).dot(ArrayBasedStruct([0.0, 1.0, 0.0, 0.0])));
    /// assert_eq!(1.0, ArrayBasedStruct([0.0, 0.0, 1.0, 0.0]).dot(ArrayBasedStruct([0.0, 0.0, 1.0, 0.0])));
    /// assert_eq!(1.0, ArrayBasedStruct([0.0, 0.0, 0.0, 1.0]).dot(ArrayBasedStruct([0.0, 0.0, 0.0, 1.0])));
    /// assert_eq!(1.0, ArrayBasedStruct([0.5, 0.5, 0.5, 0.5]).dot(ArrayBasedStruct([0.5, 0.5, 0.5, 0.5])));
    /// ```
    #[must_use]
    #[allow(clippy::suboptimal_flops)]
    fn dot(self, other: Self) -> f32 {
        Self::into_iter(self)
            .zip(other.into_iter())
            .fold(0.0, |acc, v| acc + (v.0 * v.1))
    }
}

#[cfg(test)]
mod tests_dot_product {
    use super::*;

    #[derive(Clone)]
    struct ArrayBasedStruct([f32; 4]);
    impl ArrayBase for ArrayBasedStruct {
        type Item = f32;
        fn get_array(self) -> [f32; 4] {
            self.0
        }
        fn get_array_ref(&self) -> &[f32; 4] {
            &self.0
        }
        fn get_array_mut(&mut self) -> &mut [f32; 4] {
            &mut self.0
        }
    }
    impl DotProduct for ArrayBasedStruct {}

    #[test]
    fn test() {
        assert_eq!(
            0.0,
            ArrayBasedStruct([0.0, 0.0, 0.0, 0.0]).dot(ArrayBasedStruct([0.0, 0.0, 0.0, 0.0]))
        );
        assert_eq!(
            1.0,
            ArrayBasedStruct([1.0, 0.0, 0.0, 0.0]).dot(ArrayBasedStruct([1.0, 0.0, 0.0, 0.0]))
        );
        assert_eq!(
            1.0,
            ArrayBasedStruct([0.0, 1.0, 0.0, 0.0]).dot(ArrayBasedStruct([0.0, 1.0, 0.0, 0.0]))
        );
        assert_eq!(
            1.0,
            ArrayBasedStruct([0.0, 0.0, 1.0, 0.0]).dot(ArrayBasedStruct([0.0, 0.0, 1.0, 0.0]))
        );
        assert_eq!(
            1.0,
            ArrayBasedStruct([0.0, 0.0, 0.0, 1.0]).dot(ArrayBasedStruct([0.0, 0.0, 0.0, 1.0]))
        );
        assert_eq!(
            1.0,
            ArrayBasedStruct([0.5, 0.5, 0.5, 0.5]).dot(ArrayBasedStruct([0.5, 0.5, 0.5, 0.5]))
        );

        assert_eq!(
            14.0,
            ArrayBasedStruct::dot(
                ArrayBasedStruct([1.0, 2.0, 3.0, 0.0]),
                ArrayBasedStruct([1.0, 2.0, 3.0, 0.0])
            )
        );
        assert_eq!(
            14.0,
            ArrayBasedStruct::dot(
                ArrayBasedStruct([-1.0, -2.0, -3.0, 0.0]),
                ArrayBasedStruct([-1.0, -2.0, -3.0, 0.0])
            )
        );
        assert_eq!(
            -14.0,
            ArrayBasedStruct::dot(
                ArrayBasedStruct([1.0, 2.0, 3.0, 0.0]),
                ArrayBasedStruct([-1.0, -2.0, -3.0, 0.0])
            )
        );

        assert_eq!(
            32.0,
            ArrayBasedStruct::dot(
                ArrayBasedStruct([1.0, 2.0, 3.0, 0.0]),
                ArrayBasedStruct([4.0, 5.0, 6.0, 0.0])
            )
        );
    }
}
