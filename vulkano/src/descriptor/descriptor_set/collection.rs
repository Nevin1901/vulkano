// Copyright (c) 2016 The vulkano developers
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>,
// at your option. All files in the project carrying such
// notice may not be copied, modified, or distributed except
// according to those terms.

use descriptor::descriptor::DescriptorDesc;
use descriptor::descriptor_set::DescriptorSet;
use descriptor::descriptor_set::DescriptorSetDesc;
use descriptor::descriptor_set::UnsafeDescriptorSet;

/// A collection of descriptor set objects.
pub unsafe trait DescriptorSetsCollection {
    /// Returns the number of sets in the collection. Includes possibly empty sets.
    ///
    /// In other words, this should be equal to the highest set number plus one.
    fn num_sets(&self) -> usize;

    /// Returns the descriptor set with the given id. Returns `None` if the set is empty.
    fn descriptor_set(&self, set: usize) -> Option<&UnsafeDescriptorSet>;

    /// Returns the number of descriptors in the set. Includes possibly empty descriptors.
    ///
    /// Returns `None` if the set is out of range.
    fn num_bindings_in_set(&self, set: usize) -> Option<usize>;

    /// Returns the descriptor for the given binding of the given set.
    ///
    /// Returns `None` if out of range.
    fn descriptor(&self, set: usize, binding: usize) -> Option<DescriptorDesc>;
}

unsafe impl DescriptorSetsCollection for () {
    #[inline]
    fn num_sets(&self) -> usize {
        0
    }

    #[inline]
    fn descriptor_set(&self, set: usize) -> Option<&UnsafeDescriptorSet> {
        None
    }

    #[inline]
    fn num_bindings_in_set(&self, set: usize) -> Option<usize> {
        None
    }

    #[inline]
    fn descriptor(&self, set: usize, binding: usize) -> Option<DescriptorDesc> {
        None
    }
}

unsafe impl<T> DescriptorSetsCollection for T
    where T: DescriptorSet
{
    #[inline]
    fn num_sets(&self) -> usize {
        1
    }

    #[inline]
    fn descriptor_set(&self, set: usize) -> Option<&UnsafeDescriptorSet> {
        match set {
            0 => Some(self.inner()),
            _ => None
        }
    }

    #[inline]
    fn num_bindings_in_set(&self, set: usize) -> Option<usize> {
        match set {
            0 => Some(self.num_bindings()),
            _ => None
        }
    }

    #[inline]
    fn descriptor(&self, set: usize, binding: usize) -> Option<DescriptorDesc> {
        match set {
            0 => self.descriptor(binding),
            _ => None
        }
    }
}

macro_rules! impl_collection {
    ($first:ident $(, $others:ident)*) => (
        unsafe impl<$first$(, $others)*> DescriptorSetsCollection for ($first, $($others),*)
            where $first: DescriptorSet + DescriptorSetDesc
                  $(, $others: DescriptorSet + DescriptorSetDesc)*
        {
            #[inline]
            fn num_sets(&self) -> usize {
                #![allow(non_snake_case)]
                1 $( + {let $others=0;1})*
            }

            #[inline]
            fn descriptor_set(&self, mut set: usize) -> Option<&UnsafeDescriptorSet> {
                #![allow(non_snake_case)]
                #![allow(unused_mut)]       // For the `set` parameter.

                if set == 0 {
                    return Some(self.0.inner());
                }

                let &(_, $(ref $others,)*) = self;

                $(
                    set -= 1;
                    if set == 0 {
                        return Some($others.inner());
                    }
                )*

                None
            }

            #[inline]
            fn num_bindings_in_set(&self, mut set: usize) -> Option<usize> {
                #![allow(non_snake_case)]
                #![allow(unused_mut)]       // For the `set` parameter.

                if set == 0 {
                    return Some(self.0.num_bindings());
                }

                let &(_, $(ref $others,)*) = self;

                $(
                    set -= 1;
                    if set == 0 {
                        return Some($others.num_bindings());
                    }
                )*

                None
            }

            #[inline]
            fn descriptor(&self, mut set: usize, binding: usize) -> Option<DescriptorDesc> {
                #![allow(non_snake_case)]
                #![allow(unused_mut)]       // For the `set` parameter.

                if set == 0 {
                    return self.0.descriptor(binding);
                }

                let &(_, $(ref $others,)*) = self;

                $(
                    set -= 1;
                    if set == 0 {
                        return $others.descriptor(binding);
                    }
                )*

                None
            }
        }

        impl_collection!($($others),*);
    );

    () => ();
}

impl_collection!(Z, Y, X, W, V, U, T, S, R, Q, P, O, N, M, L, K, J, I, H, G, F, E, D, C, B, A);
