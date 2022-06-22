#![feature(generic_associated_types)]

use std::marker::PhantomData;

pub trait XcDataType {
    type XcObjectType<T: XcDataType>;
    type PropertiesType;
    type NilType: XcDataType;
    type BaseObjectType: XcDataType;
    type CompleteType;
    type ExtensibleDataType<T: XcDataType>;
    type BuilderType;
    type ConstructorType;

    fn get_constructor(
        base_builder: <<Self as XcDataType>::BaseObjectType as XcDataType>::BuilderType,
    ) -> Self::ConstructorType;
}

#[macro_export]
macro_rules! declare_xcdt {
    ($ty_name: ident, $prop_type: ty, $base_complete_ty: ty) => {
        paste::paste! {
            pub struct [<Xc $ty_name>]<T: xcdt::XcDataType> {
                props: $prop_type,
                ext: T,
            }

            impl<T: xcdt::XcDataType> [<Xc $ty_name>]<T> {
                pub fn new(props: $prop_type, ext: T) -> Self {
                    Self { props, ext }
                }


                pub fn ext(&self) -> &T {
                    &self.ext
                }

                pub fn get_xc_object(object: &[<$ty_name Base>]<T>) -> &[<Xc $ty_name>]<T>{
                    <<<$base_complete_ty as xcdt::XcDataType>
                        ::NilType as xcdt::XcDataType>::BaseObjectType as xcdt::XcDataType>
                            ::XcObjectType::<[<Xc $ty_name>]<T>>::get_xc_object(object).ext()
                }
            }

            pub struct [<Xc $ty_name Builder>]<T: xcdt::XcDataType>{
                base_builder: <<[<Xc $ty_name>]<T> as xcdt::XcDataType>::BaseObjectType as xcdt::XcDataType>::BuilderType,
                props: $prop_type,
            }

            impl<T: xcdt::XcDataType> [<Xc $ty_name Builder>]<T> {
                pub fn build(self, ext: T) -> <[<Xc $ty_name>]<T> as xcdt::XcDataType>::CompleteType {
                    self.base_builder.build([<Xc $ty_name>] { props: self.props, ext })
                }
            }

            pub struct [<$ty_name Constructor>]<T: xcdt::XcDataType> {
                base_builder: <<[<Xc $ty_name>]<T> as xcdt::XcDataType>::BaseObjectType as xcdt::XcDataType>::BuilderType,
            }

            impl<T: xcdt::XcDataType<BaseObjectType = [<Xc $ty_name>]<T>>> [<$ty_name Constructor>]<T> {
                pub fn with(self, props: $prop_type) -> T::ConstructorType {
                    T::get_constructor( [<Xc $ty_name Builder>]::<T> { base_builder: self.base_builder, props })
                }
            }

            impl<T: xcdt::XcDataType> xcdt::XcDataType for [<Xc $ty_name>]<T> {
                type XcObjectType<U: xcdt::XcDataType> = [<Xc $ty_name>]<U>;
                type NilType = T::NilType;
                type PropertiesType = $prop_type;
                type BaseObjectType = <<
                    <$base_complete_ty as xcdt::XcDataType>
                        ::NilType as xcdt::XcDataType>
                            ::BaseObjectType as xcdt::XcDataType>
                                ::XcObjectType<[<Xc $ty_name>]<T>>;

                // type CompleteType = <<
                //     <Self as xcdt::XcDataType>
                //         ::BaseObjectType as xcdt::XcDataType>
                //             ::ExtensibleDataType <[<Xc $ty_name>]<T>> as xcdt::XcDataType>
                //                 ::CompleteType;

                type CompleteType = <Self as xcdt::XcDataType>::ExtensibleDataType<T>;

                type ExtensibleDataType<U: xcdt::XcDataType> = <<
                    <$base_complete_ty as xcdt::XcDataType>
                        ::NilType as xcdt::XcDataType>
                            ::BaseObjectType as xcdt::XcDataType>
                                ::ExtensibleDataType<[<Xc $ty_name>]<U>>;
                type BuilderType = [<Xc $ty_name Builder>]<T>;
                type ConstructorType = [<$ty_name Constructor>]<T>;

                fn get_constructor(base_builder: <<Self as xcdt::XcDataType>::BaseObjectType as xcdt::XcDataType>::BuilderType) -> Self::ConstructorType {
                    Self::ConstructorType { base_builder }
                }
            }

            #[allow(non_snake_case)]
            pub trait [<Is $ty_name>] {
                fn $prop_type(&self) -> &$prop_type;
            }

            impl<T: xcdt::XcDataType> [<Is $ty_name>] for [<$ty_name Base>]<T> {
                fn $prop_type(&self) -> &$prop_type {
                    &<[<Xc $ty_name>]<T>>::get_xc_object(self).props
                }
            }

            pub type $ty_name = <[<Xc $ty_name>]<[<_ $ty_name _nil>]::Nil> as xcdt::XcDataType>::CompleteType;
            // pub type $ty_name = [<$ty_name Base>]<[<_ $ty_name _nil>]::Nil>;
            pub type [<$ty_name Base>]<T> = <[<Xc $ty_name>]<[<_ $ty_name _nil>]::Nil> as xcdt::XcDataType>::ExtensibleDataType<T>;

            pub mod [<_ $ty_name _nil>] {
                pub struct Nil;

                impl xcdt::XcDataType for Nil {
                    type XcObjectType<U: xcdt::XcDataType> = Nil;
                    type NilType = Nil;
                    type PropertiesType = ();
                    type BaseObjectType = super::[<Xc $ty_name>]<Nil>;
                    type CompleteType = <super::[<Xc $ty_name>]<Nil> as xcdt::XcDataType>::CompleteType;
                    type ExtensibleDataType<U: xcdt::XcDataType> = ();
                    type BuilderType = ();
                    type ConstructorType = CompleteObjectBuiler;

                    fn get_constructor(base_builder: <<Self as xcdt::XcDataType>::BaseObjectType as xcdt::XcDataType>::BuilderType) -> Self::ConstructorType {
                        Self::ConstructorType { base_builder }
                    }
                }

                pub struct CompleteObjectBuiler {
                    base_builder: super::[<Xc $ty_name Builder>]<Nil>,
                }
                impl CompleteObjectBuiler {
                    pub fn build(self) -> <Nil as xcdt::XcDataType>::CompleteType {
                        self.base_builder.build(Nil{})
                    }
                }
            }
        }
    };
}

pub type ObjectBase<T> = XcObject<T>;
pub type Object = XcObject<nil::Nil>;

pub struct XcObject<T: XcDataType> {
    ext: T,
}

impl<T: XcDataType<BaseObjectType = XcObject<T>>> XcObject<T> {
    pub fn new(ext: T) -> Self {
        Self { ext }
    }

    pub fn ext(&self) -> &T {
        &self.ext
    }

    pub fn builder() -> T::ConstructorType {
        XcObjectConstructor::<T> { _pd: PhantomData }.with()
    }

    pub fn get_xc_object(object: &ObjectBase<T>) -> &XcObject<T> {
        object
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

impl<T: XcDataType<BaseObjectType = XcObject<T>>> XcObjectConstructor<T> {
    pub fn with(self) -> T::ConstructorType {
        T::get_constructor(XcObjectBuilder::<T> { _pd: PhantomData })
    }
}

impl<T: XcDataType> std::fmt::Debug for XcObject<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "XcObject")
    }
}

impl<T: XcDataType> XcDataType for XcObject<T> {
    type XcObjectType<U: XcDataType> = XcObject<U>;
    type PropertiesType = ();
    type BaseObjectType = XcObject<T>;
    type NilType = T::NilType;
    type CompleteType = XcObject<T>;
    type ExtensibleDataType<U: XcDataType> = XcObject<U>;
    type BuilderType = XcObjectBuilder<T>;
    type ConstructorType = XcObjectConstructor<T>;

    fn get_constructor(
        _: <<Self as XcDataType>::BaseObjectType as XcDataType>::BuilderType,
    ) -> Self::ConstructorType {
        XcObjectConstructor::<T> { _pd: PhantomData }
    }
}

mod nil {
    use crate::{XcDataType, XcObject};

    pub struct Nil;
    impl XcDataType for Nil {
        type XcObjectType<U: XcDataType> = Nil;
        type PropertiesType = ();
        type BaseObjectType = XcObject<Nil>;
        type NilType = Nil;
        type CompleteType = XcObject<Nil>;
        type ExtensibleDataType<U: XcDataType> = Nil;
        type BuilderType = Nil;
        type ConstructorType = Nil;

        fn get_constructor(
            _: <<Self as XcDataType>::BaseObjectType as XcDataType>::BuilderType,
        ) -> Self::ConstructorType {
            Nil
        }
    }
}
