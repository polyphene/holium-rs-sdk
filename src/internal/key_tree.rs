//! Key trees are responsible for recursively holding structure keys used as I/O paramters in transformations

#[derive(Default, Debug, Eq, PartialEq)]
pub struct Node {
    pub value: Option<&'static str>,
    pub children: Vec<Node>,
}

/// Trait meant to generate key node for supported native rust types
pub trait GenerateNode {
    fn generate_node() -> Node;
}

macro_rules! primitives_impl {
    ($ty:ident) => {
        impl GenerateNode for $ty {
            #[inline]
            fn generate_node() -> Node {
                Node::default()
            }
        }
    };
}

primitives_impl!(bool);
primitives_impl!(isize);
primitives_impl!(i8);
primitives_impl!(i16);
primitives_impl!(i32);
primitives_impl!(i64);
primitives_impl!(i128);
primitives_impl!(usize);
primitives_impl!(u8);
primitives_impl!(u16);
primitives_impl!(u32);
primitives_impl!(u64);
primitives_impl!(u128);
primitives_impl!(f32);
primitives_impl!(f64);
primitives_impl!(char);
primitives_impl!(str);
primitives_impl!(String);

impl<T> GenerateNode for [T] {
    #[inline]
    fn generate_node() -> Node {
        Node::default()
    }
}

impl<T> GenerateNode for Vec<T> {
    #[inline]
    fn generate_node() -> Node {
        Node::default()
    }
}

macro_rules! array_impls {
    ($($len:tt)+) => {
        $(
            impl<T> GenerateNode for [T; $len] {
                #[inline]
                fn generate_node() -> Node {
                    Node::default()
                }
            }
        )+
    }
}

array_impls! {
    0  01 02 03 04 05 06 07 08 09
    10 11 12 13 14 15 16 17 18 19
    20 21 22 23 24 25 26 27 28 29
    30 31 32
}

impl<T> GenerateNode for Option<T>
where
    T: GenerateNode,
{
    #[inline]
    fn generate_node() -> Node {
        T::generate_node()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! declare_primitive_tests {
        (
            $readable:tt
            $($name:ident { $($ty:ident,)+ })+
        ) => {
            $(
                #[test]
                fn $name() {
                    $(
                        assert_eq!(Node { value: None, children: Vec::new() }, $ty::generate_node());
                    )+
                }
            )+
        };

        ($($name:ident { $($ty:ident,)+ })+) => {
            $(
                #[test]
                fn $name() {
                    $(
                        assert_eq!(Node { value: None, children: Vec::new() }, $ty::generate_node());
                    )+
                }
            )+
        }
    }

    declare_primitive_tests! {
        test_primitives {
            bool,
            isize,
            i8,
            i16,
            i32,
            i64,
            usize,
            u8,
            u16,
            u32,
            u64,
            f32,
            f64,
            char,
            i128,
            u128,
            str,
            String,
        }
    }

    #[test]
    fn test_array() {
        assert_eq!(Node::default(), <[u8; 32]>::generate_node());
        assert_eq!(Node::default(), <[u8]>::generate_node());
    }

    #[test]
    fn test_vec() {
        assert_eq!(Node::default(), Vec::<u8>::generate_node());
    }

    #[test]
    fn test_option() {
        assert_eq!(Node::default(), Option::<u8>::generate_node());
    }
}
