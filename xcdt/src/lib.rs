#![feature(generic_associated_types)]

use std::marker::PhantomData;

pub trait XcDataType {
    type XcObjectType<T: XcDataType>;
    type PropertiesType;
    type PreviousDataType: XcDataType;
    type CompleteType;
    type ExtensibleDataType<T: XcDataType>;
    type BuilderType;
    type ConstructorType;

    fn get_constructor(
        base_builder: <<Self as XcDataType>::PreviousDataType as XcDataType>::BuilderType,
    ) -> Self::ConstructorType;
}

#[macro_export]
macro_rules! declare_xcdt {
    ($ty_name: ident, $prop_type: ty, $test_ty: ty, $base_ty2: ty) => {
        paste::paste! {
            pub struct [<Xc $ty_name>]<T: xcdt::XcDataType> {
                properties: $prop_type,
                ext: T,
            }

            impl<T: xcdt::XcDataType> [<Xc $ty_name>]<T> {
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

            pub struct [<Xc $ty_name Builder>]<T: xcdt::XcDataType>{
                base_builder: <<[<Xc $ty_name>]<T> as xcdt::XcDataType>::PreviousDataType as xcdt::XcDataType>::BuilderType,
                properties: $prop_type,
            }

            impl<T: xcdt::XcDataType> [<Xc $ty_name Builder>]<T> {
                pub fn build(self, ext: T) -> <[<Xc $ty_name>]<T> as xcdt::XcDataType>::CompleteType {
                    self.base_builder.build([<Xc $ty_name>] { properties: self.properties, ext })
                }
            }

            pub struct [<$ty_name Constructor>]<T: xcdt::XcDataType> {
                base_builder: <<[<Xc $ty_name>]<T> as xcdt::XcDataType>::PreviousDataType as xcdt::XcDataType>::BuilderType,
            }

            impl<T: xcdt::XcDataType<PreviousDataType = [<Xc $ty_name>]<T>>> [<$ty_name Constructor>]<T> {
                pub fn with(self, properties: $prop_type) -> T::ConstructorType {
                    T::get_constructor( [<Xc $ty_name Builder>]::<T> { base_builder: self.base_builder, properties })
                }
            }

            impl<T: xcdt::XcDataType> xcdt::XcDataType for [<Xc $ty_name>]<T> {
                type XcObjectType<U: xcdt::XcDataType> = [<Xc $ty_name>]<U>;
                type PropertiesType = $prop_type;
                type PreviousDataType = <$test_ty as xcdt::XcDataType>::ExtensibleDataType<[<Xc $ty_name>]<T>>;
                type CompleteType = <<$test_ty as xcdt::XcDataType>::ExtensibleDataType <[<Xc $ty_name>]<T>> as xcdt::XcDataType>::CompleteType;
                type ExtensibleDataType<U: xcdt::XcDataType> = <<$test_ty as xcdt::XcDataType>::ExtensibleDataType <[<Xc $ty_name>]<T>> as xcdt::XcDataType>::ExtensibleDataType<[<Xc $ty_name>]<U>>;
                type BuilderType = [<Xc $ty_name Builder>]<T>;
                type ConstructorType = [<$ty_name Constructor>]<T>;

                fn get_constructor(base_builder: <<Self as xcdt::XcDataType>::PreviousDataType as xcdt::XcDataType>::BuilderType) -> Self::ConstructorType {
                    Self::ConstructorType { base_builder }
                }
            }

            pub type $ty_name = <[<Xc $ty_name>]< [<_ $ty_name _nil>]::Nil> as xcdt::XcDataType>::CompleteType;
            pub type [<$ty_name Base>]<T> = $base_ty2<[<Xc $ty_name>]<T>>;

            mod [<_ $ty_name _nil>] {
                pub struct Nil;

                impl xcdt::XcDataType for Nil {
                    type XcObjectType<U: xcdt::XcDataType> = Nil;
                    type PropertiesType = Nil;
                    type PreviousDataType = super::[<Xc $ty_name>]<Nil>;
                    type CompleteType = <super::[<Xc $ty_name>]<Nil> as xcdt::XcDataType>::CompleteType;
                    type ExtensibleDataType<U: xcdt::XcDataType> = Nil;
                    type BuilderType = NilConstructor;
                    type ConstructorType = NilConstructor;

                    fn get_constructor(base_builder: <<Self as xcdt::XcDataType>::PreviousDataType as xcdt::XcDataType>::BuilderType) -> Self::ConstructorType {
                        Self::ConstructorType { base_builder }
                    }
                }

                pub struct NilConstructor {
                    base_builder: super::[<Xc $ty_name Builder>]<Nil>,
                }
                impl NilConstructor {
                    pub fn build(self) -> <Nil as xcdt::XcDataType>::CompleteType {
                        self.base_builder.build(Nil{})
                    }
                }
            }
        }
    };
}

pub type XcObjectBase<T> = XcObject<T>;
pub type Object = XcObject<Nil>;

pub struct Nil;
impl XcDataType for Nil {
    type XcObjectType<U: XcDataType> = Nil;
    type PropertiesType = Nil;
    type PreviousDataType = XcObject<Nil>;
    type CompleteType = <XcObject<Nil> as XcDataType>::CompleteType;
    type ExtensibleDataType<U: XcDataType> = Nil;
    type BuilderType = Nil;
    type ConstructorType = Nil;

    fn get_constructor(
        _: <<Self as XcDataType>::PreviousDataType as XcDataType>::BuilderType,
    ) -> Self::ConstructorType {
        Nil
    }
}

pub struct XcObject<T: XcDataType> {
    ext: T,
}

impl<T: XcDataType> XcObject<T> {
    pub fn new(ext: T) -> Self {
        Self { ext }
    }

    pub fn ext(&self) -> &T {
        &self.ext
    }
}

pub struct XcObjectBuilder<T: XcDataType> {
    _pd: PhantomData<T>,
}

impl<T: XcDataType> XcObjectBuilder<T> {
    pub fn build(self, ext: T) -> XcObject<T> {
        XcObject { ext }
    }
}

pub struct XcObjectConstructor<T: XcDataType> {
    _pd: PhantomData<T>,
}

impl<T: XcDataType<PreviousDataType = XcObject<T>>> XcObjectConstructor<T> {
    pub fn with(self) -> T::ConstructorType {
        T::get_constructor(XcObjectBuilder::<T> { _pd: PhantomData })
    }
}

impl<T: XcDataType> XcDataType for XcObject<T> {
    type XcObjectType<U: XcDataType> = XcObject<U>;
    type PropertiesType = ();
    type PreviousDataType = XcObject<T>;
    type CompleteType = XcObject<T>;
    type ExtensibleDataType<U: XcDataType> = XcObject<U>;
    type BuilderType = XcObjectBuilder<T>;
    type ConstructorType = XcObjectConstructor<T>;

    fn get_constructor(
        _: <<Self as XcDataType>::PreviousDataType as XcDataType>::BuilderType,
    ) -> Self::ConstructorType {
        XcObjectConstructor::<T> { _pd: PhantomData }
    }
}
