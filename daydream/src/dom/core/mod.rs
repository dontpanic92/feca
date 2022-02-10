pub(crate) mod character_data;
pub(crate) mod element;
pub(crate) mod node;
pub(crate) mod text;

pub struct Nil {}

impl XcDataType for Nil {
    type PropertiesType = Nil;
}

pub trait XcDataType {
    type PropertiesType;
}

#[macro_export]
macro_rules! declare_xcdt {
    ($ty_name: ident, $prop_type: ty, $base_ty: ty) => {
        paste::paste! {
            pub struct [<Xc $ty_name>]<T: XcDataType> {
                properties: $prop_type,
                ext: T,
            }

            impl<T: XcDataType> [<Xc $ty_name>]<T> {
                pub fn new(properties: $prop_type, ext: T) -> Self {
                    Self { properties, ext }
                }

                pub fn ext(&self) -> &T {
                    &self.ext
                }

                pub fn properties(&self) -> &$prop_type {
                    &self.properties
                }
            }

            impl<T: XcDataType> XcDataType for [<Xc $ty_name>]<T> {
                type PropertiesType = $prop_type;
            }

            pub(crate) type [<$ty_name Base>]<T> = $base_ty <[<Xc $ty_name>]<T>>;

            impl<T: XcDataType> [<$ty_name Base>]<T> {
                pub fn [<as_ $ty_name Base>](&self) -> &[<Xc $ty_name>]<T> {
                    self. [<as_ $base_ty>] ().ext()
                }
            }

            pub(crate) type $ty_name = $base_ty<[<Xc $ty_name>]<Nil>>;

        }
    };
}

type XcObjectBase<T> = XcObject<T>;

pub(crate) struct XcObject<T: XcDataType> {
    ext: T,
}

impl<T: XcDataType> XcObject<T> {
    pub fn new(ext: T) -> Self {
        Self { ext }
    }

    pub fn ext(&self) -> &T {
        &self.ext
    }

    pub fn as_XcObjectBase(&self) -> &XcObject<T> {
        self
    }
}
