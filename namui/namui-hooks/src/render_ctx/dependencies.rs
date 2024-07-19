use crate::*;

pub trait Dependencies<T> {
    fn cloned(self) -> T;
}

impl<T, U> Dependencies<U> for &T
where
    T: ToOwned<Owned = U> + ?Sized,
    U: 'static,
{
    fn cloned(self) -> U {
        self.to_owned()
    }
}

impl<T, R> Dependencies<T> for Sig<'_, T, R>
where
    T: 'static + Send + Clone,
    R: std::borrow::Borrow<T>,
{
    fn cloned(self) -> T {
        self.as_ref().clone()
    }
}

macro_rules! dependencies_impl {
    (
        $(
            ($
                ($T:ident, $i:tt, $R: ident),
            *),
        )*
    ) => {
        $(
            impl<
                $($T: Dependencies<$R>,)*
                $($R: 'static,)*
            > Dependencies<($($R,)*)> for ($($T,)*)
            {
                fn cloned(self) -> ($($R,)*) {
                    ($(self.$i.cloned(),)*)
                }
            }
        )*
    };
}

dependencies_impl!(
    (T0, 0, R0),
    (T0, 0, R0, T1, 1, R1),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2, T3, 3, R3),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2, T3, 3, R3, T4, 4, R4),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2, T3, 3, R3, T4, 4, R4, T5, 5, R5),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2, T3, 3, R3, T4, 4, R4, T5, 5, R5, T6, 6, R6),
    (T0, 0, R0, T1, 1, R1, T2, 2, R2, T3, 3, R3, T4, 4, R4, T5, 5, R5, T6, 6, R6, T7, 7, R7),
);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn str_dependencies_should_be_string() {
        let str_ref = "hello";
        let _cloned: String = str_ref.cloned();
    }

    #[test]
    fn sig_dependencies_should_be_inner() {
        use std::sync::Arc;
        struct MockSkCalculate;
        impl SkCalculate for MockSkCalculate {
            fn group_glyph(&self, _font: &Font, _paint: &Paint) -> Arc<dyn GroupGlyph> {
                unimplemented!()
            }

            fn font_metrics(&self, _font: &Font) -> Option<FontMetrics> {
                unimplemented!()
            }

            fn load_typeface(
                &self,
                _typeface_name: String,
                _bytes: Vec<u8>,
            ) -> JoinHandle<Result<()>> {
                unimplemented!()
            }

            fn path_contains_xy(&self, _path: &Path, _paint: Option<&Paint>, _xy: Xy<Px>) -> bool {
                unimplemented!()
            }

            fn path_bounding_box(&self, _path: &Path, _paint: Option<&Paint>) -> Option<Rect<Px>> {
                unimplemented!()
            }

            fn load_image_from_raw(
                &self,
                _image_info: ImageInfo,
                _bitmap: &[u8],
            ) -> JoinHandle<Image> {
                unimplemented!()
            }

            fn load_image_from_encoded(&self, _bytes: &[u8]) -> JoinHandle<Image> {
                todo!()
            }
        }

        let world = World::init(Instant::now, &MockSkCalculate);
        let sig = Sig::new(1, SigId::Atom { index: 0 }, &world);
        let _cloned: i32 = sig.cloned();
    }
}
