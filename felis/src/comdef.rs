use crate as felis;
// Interface IRenderable

#[repr(C)]
#[allow(non_snake_case)]
pub struct IRenderableVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub display: fn(this: *const *const std::os::raw::c_void) -> crate::style::Display,
    pub layout: fn(
        this: *const *const std::os::raw::c_void,
        pango_context: &pango::Context,
        style_computed: &crate::style::Style,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle,
    pub paint: fn(
        this: *const *const std::os::raw::c_void,
        renderer: &crate::rendering::cairo::CairoRenderer,
        style_computed: &crate::style::Style,
    ) -> crosscom::Void,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IRenderableVirtualTableCcw {
    pub offset: isize,
    pub vtable: IRenderableVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IRenderable {
    pub vtable: *const IRenderableVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IRenderable {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IRenderable as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IRenderable as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IRenderable as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn display(&self) -> crate::style::Display {
        unsafe {
            let this = self as *const IRenderable as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).display)(this);

            ret
        }
    }

    pub fn layout(
        &self,
        pango_context: &pango::Context,
        style_computed: &crate::style::Style,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle {
        unsafe {
            let this = self as *const IRenderable as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).layout)(
                this,
                pango_context.into(),
                style_computed.into(),
                content_boundary.into(),
            );

            ret
        }
    }

    pub fn paint(
        &self,
        renderer: &crate::rendering::cairo::CairoRenderer,
        style_computed: &crate::style::Style,
    ) -> crosscom::Void {
        unsafe {
            let this = self as *const IRenderable as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).paint)(this, renderer.into(), style_computed.into());

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IRenderable::INTERFACE_ID)
    }
}

pub trait IRenderableImpl {
    fn display(&self) -> crate::style::Display;
    fn layout(
        &self,
        pango_context: &pango::Context,
        style_computed: &crate::style::Style,
        content_boundary: crate::common::Rectangle,
    ) -> crate::common::Rectangle;
    fn paint(
        &self,
        renderer: &crate::rendering::cairo::CairoRenderer,
        style_computed: &crate::style::Style,
    ) -> crosscom::Void;
}

impl crosscom::ComInterface for IRenderable {
    // 81bb38bb-da96-4eee-9a80-47bae3c060aa
    const INTERFACE_ID: [u8; 16] = [
        129u8, 187u8, 56u8, 187u8, 218u8, 150u8, 78u8, 238u8, 154u8, 128u8, 71u8, 186u8, 227u8,
        192u8, 96u8, 170u8,
    ];
}

// Interface IDomString

#[repr(C)]
#[allow(non_snake_case)]
pub struct IDomStringVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub bytes: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const std::os::raw::c_uchar,
    pub str: fn(this: *const *const std::os::raw::c_void) -> crosscom::StaticStr,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IDomStringVirtualTableCcw {
    pub offset: isize,
    pub vtable: IDomStringVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IDomString {
    pub vtable: *const IDomStringVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IDomString {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IDomString as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IDomString as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IDomString as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn bytes(&self) -> *const std::os::raw::c_uchar {
        unsafe {
            let this = self as *const IDomString as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).bytes)(this);
            let ret: *const std::os::raw::c_uchar = ret.into();

            ret
        }
    }

    pub fn str(&self) -> crosscom::StaticStr {
        unsafe {
            let this = self as *const IDomString as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).str)(this);

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IDomString::INTERFACE_ID)
    }
}

pub trait IDomStringImpl {
    fn bytes(&self) -> *const std::os::raw::c_uchar;
    fn str(&self) -> crosscom::StaticStr;
}

impl crosscom::ComInterface for IDomString {
    // 6c636789-fd7e-4411-b2e8-6606538e9c8d
    const INTERFACE_ID: [u8; 16] = [
        108u8, 99u8, 103u8, 137u8, 253u8, 126u8, 68u8, 17u8, 178u8, 232u8, 102u8, 6u8, 83u8, 142u8,
        156u8, 141u8,
    ];
}

// Class DomString

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_DomString {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod DomString_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct DomStringCcw {
                IDomString: felis::comdef::IDomString,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<DomStringCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IDomString::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<DomStringCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<DomStringCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut DomStringCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn bytes(
                this: *const *const std::os::raw::c_void,
            ) -> *const std::os::raw::c_uchar {
                let __crosscom_object = crosscom::get_object::<DomStringCcw>(this);
                (*__crosscom_object).inner.bytes().into()
            }

            fn str(this: *const *const std::os::raw::c_void) -> crosscom::StaticStr {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<DomStringCcw>(this);
                    (*__crosscom_object).inner.str()
                }
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IDomStringVirtualTable_CCW_FOR_DomString:
                felis::comdef::IDomStringVirtualTableCcw =
                felis::comdef::IDomStringVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IDomStringVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        bytes,
                        str,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = DomStringCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IDomString: felis::comdef::IDomString {
                            vtable: &GLOBAL_IDomStringVirtualTable_CCW_FOR_DomString.vtable
                                as *const felis::comdef::IDomStringVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this =
                            this.offset(-(crosscom::offset_of!(DomStringCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_DomString;

// Interface INode

#[repr(C)]
#[allow(non_snake_case)]
pub struct INodeVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
}

#[repr(C)]
#[allow(dead_code)]
pub struct INodeVirtualTableCcw {
    pub offset: isize,
    pub vtable: INodeVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct INode {
    pub vtable: *const INodeVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl INode {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const INode as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(INode::INTERFACE_ID)
    }
}

pub trait INodeImpl {
    fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode>;
    fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString>;
    fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString>;
    fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> ();
    fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement>;
    fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>>;
}

impl crosscom::ComInterface for INode {
    // 518d7182-9244-448e-a439-624115b0be12
    const INTERFACE_ID: [u8; 16] = [
        81u8, 141u8, 113u8, 130u8, 146u8, 68u8, 68u8, 142u8, 164u8, 57u8, 98u8, 65u8, 21u8, 176u8,
        190u8, 18u8,
    ];
}

// Interface IElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IElement {
    pub vtable: *const IElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IElement::INTERFACE_ID)
    }
}

pub trait IElementImpl {
    fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString>;
    fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString>;
}

impl crosscom::ComInterface for IElement {
    // c8c9b586-0569-4af4-b619-6491c61dc94a
    const INTERFACE_ID: [u8; 16] = [
        200u8, 201u8, 181u8, 134u8, 5u8, 105u8, 74u8, 244u8, 182u8, 25u8, 100u8, 145u8, 198u8,
        29u8, 201u8, 74u8,
    ];
}

// Interface ICharacterData

#[repr(C)]
#[allow(non_snake_case)]
pub struct ICharacterDataVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub text: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
}

#[repr(C)]
#[allow(dead_code)]
pub struct ICharacterDataVirtualTableCcw {
    pub offset: isize,
    pub vtable: ICharacterDataVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct ICharacterData {
    pub vtable: *const ICharacterDataVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl ICharacterData {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn text(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).text)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(ICharacterData::INTERFACE_ID)
    }
}

pub trait ICharacterDataImpl {
    fn text(&self) -> crosscom::ComRc<felis::comdef::IDomString>;
}

impl crosscom::ComInterface for ICharacterData {
    // a8b5f552-0f22-4c4d-930c-432312b16a6c
    const INTERFACE_ID: [u8; 16] = [
        168u8, 181u8, 245u8, 82u8, 15u8, 34u8, 76u8, 77u8, 147u8, 12u8, 67u8, 35u8, 18u8, 177u8,
        106u8, 108u8,
    ];
}

// Interface IText

#[repr(C)]
#[allow(non_snake_case)]
pub struct ITextVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub text: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
}

#[repr(C)]
#[allow(dead_code)]
pub struct ITextVirtualTableCcw {
    pub offset: isize,
    pub vtable: ITextVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IText {
    pub vtable: *const ITextVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IText {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IText as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn text(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).text)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IText::INTERFACE_ID)
    }
}

pub trait ITextImpl {}

impl crosscom::ComInterface for IText {
    // 2d54063c-b56b-44ec-8930-2ca618be2ecf
    const INTERFACE_ID: [u8; 16] = [
        45u8, 84u8, 6u8, 60u8, 181u8, 107u8, 68u8, 236u8, 137u8, 48u8, 44u8, 166u8, 24u8, 190u8,
        46u8, 207u8,
    ];
}

// Class Text

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_Text {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod Text_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct TextCcw {
                IText: felis::comdef::IText,
                IRenderable: felis::comdef::IRenderable,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<TextCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::ICharacterData::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IText::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<TextCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<TextCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut TextCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            fn display(this: *const *const std::os::raw::c_void) -> crate::style::Display {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<TextCcw>(this);
                    (*__crosscom_object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<TextCcw>(this);
                    (*__crosscom_object).inner.0.layout(
                        pango_context,
                        style_computed,
                        content_boundary,
                    )
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<TextCcw>(this);
                    (*__crosscom_object).inner.0.paint(renderer, style_computed)
                }
            }

            unsafe extern "system" fn text(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<TextCcw>(this);
                (*__crosscom_object).inner.0.text().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<TextCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<TextCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<TextCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<TextCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<TextCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<TextCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_ITextVirtualTable_CCW_FOR_Text: felis::comdef::ITextVirtualTableCcw =
                felis::comdef::ITextVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::ITextVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        text,
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_Text:
                felis::comdef::IRenderableVirtualTableCcw =
                felis::comdef::IRenderableVirtualTableCcw {
                    offset: -1,
                    vtable: felis::comdef::IRenderableVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        display,
                        layout,
                        paint,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = TextCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IText: felis::comdef::IText {
                            vtable: &GLOBAL_ITextVirtualTable_CCW_FOR_Text.vtable
                                as *const felis::comdef::ITextVirtualTable,
                        },

                        IRenderable: felis::comdef::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_Text.vtable
                                as *const felis::comdef::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this = this.offset(-(crosscom::offset_of!(TextCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_Text;

// Interface IHtmlElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IHtmlElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub on_mouse_move: fn(
        this: *const *const std::os::raw::c_void,
        x: f64,
        y: f64,
        window: &winit::window::Window,
    ) -> crosscom::Void,
    pub on_mouse_click: fn(this: *const *const std::os::raw::c_void) -> crate::page::FelisAction,
    pub merge_style:
        fn(this: *const *const std::os::raw::c_void, style: &crate::style::Style) -> crosscom::Void,
    pub get_attribute:
        fn(this: *const *const std::os::raw::c_void, key: &str) -> Option<Option<String>>,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IHtmlElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlElement {
    pub vtable: *const IHtmlElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IHtmlElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_move)(this, x.into(), y.into(), window.into());

            ret
        }
    }

    pub fn on_mouse_click(&self) -> crate::page::FelisAction {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_click)(this);

            ret
        }
    }

    pub fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).merge_style)(this, style.into());

            ret
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<Option<String>> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_attribute)(this, key.into());

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IHtmlElement::INTERFACE_ID)
    }
}

pub trait IHtmlElementImpl {
    fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void;
    fn on_mouse_click(&self) -> crate::page::FelisAction;
    fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void;
    fn get_attribute(&self, key: &str) -> Option<Option<String>>;
}

impl crosscom::ComInterface for IHtmlElement {
    // 2be9cc09-3c60-45b9-9084-e4e50ab94ad2
    const INTERFACE_ID: [u8; 16] = [
        43u8, 233u8, 204u8, 9u8, 60u8, 96u8, 69u8, 185u8, 144u8, 132u8, 228u8, 229u8, 10u8, 185u8,
        74u8, 210u8,
    ];
}

// Class HtmlElement

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_HtmlElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlElement_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct HtmlElementCcw {
                IHtmlElement: felis::comdef::IHtmlElement,
                IRenderable: felis::comdef::IRenderable,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlElementCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlElementCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlElementCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut HtmlElementCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            fn on_mouse_move(
                this: *const *const std::os::raw::c_void,
                x: f64,
                y: f64,
                window: &winit::window::Window,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_move(x, y, window)
                }
            }

            fn on_mouse_click(
                this: *const *const std::os::raw::c_void,
            ) -> crate::page::FelisAction {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_click()
                }
            }

            fn merge_style(
                this: *const *const std::os::raw::c_void,
                style: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.merge_style(style)
                }
            }

            fn get_attribute(
                this: *const *const std::os::raw::c_void,
                key: &str,
            ) -> Option<Option<String>> {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.get_attribute(key)
                }
            }

            fn display(this: *const *const std::os::raw::c_void) -> crate::style::Display {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.layout(
                        pango_context,
                        style_computed,
                        content_boundary,
                    )
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.paint(renderer, style_computed)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                (*__crosscom_object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                (*__crosscom_object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<HtmlElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlElementVirtualTable_CCW_FOR_HtmlElement:
                felis::comdef::IHtmlElementVirtualTableCcw =
                felis::comdef::IHtmlElementVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IHtmlElementVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        id,
                        tag,
                        on_mouse_move,
                        on_mouse_click,
                        merge_style,
                        get_attribute,
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlElement:
                felis::comdef::IRenderableVirtualTableCcw =
                felis::comdef::IRenderableVirtualTableCcw {
                    offset: -1,
                    vtable: felis::comdef::IRenderableVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        display,
                        layout,
                        paint,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IHtmlElement: felis::comdef::IHtmlElement {
                            vtable: &GLOBAL_IHtmlElementVirtualTable_CCW_FOR_HtmlElement.vtable
                                as *const felis::comdef::IHtmlElementVirtualTable,
                        },

                        IRenderable: felis::comdef::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlElement.vtable
                                as *const felis::comdef::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this =
                            this.offset(-(crosscom::offset_of!(HtmlElementCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_HtmlElement;

// Interface IHtmlHtmlElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IHtmlHtmlElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub on_mouse_move: fn(
        this: *const *const std::os::raw::c_void,
        x: f64,
        y: f64,
        window: &winit::window::Window,
    ) -> crosscom::Void,
    pub on_mouse_click: fn(this: *const *const std::os::raw::c_void) -> crate::page::FelisAction,
    pub merge_style:
        fn(this: *const *const std::os::raw::c_void, style: &crate::style::Style) -> crosscom::Void,
    pub get_attribute:
        fn(this: *const *const std::os::raw::c_void, key: &str) -> Option<Option<String>>,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlHtmlElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IHtmlHtmlElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlHtmlElement {
    pub vtable: *const IHtmlHtmlElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IHtmlHtmlElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_move)(this, x.into(), y.into(), window.into());

            ret
        }
    }

    pub fn on_mouse_click(&self) -> crate::page::FelisAction {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_click)(this);

            ret
        }
    }

    pub fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).merge_style)(this, style.into());

            ret
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<Option<String>> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_attribute)(this, key.into());

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IHtmlHtmlElement::INTERFACE_ID)
    }
}

pub trait IHtmlHtmlElementImpl {}

impl crosscom::ComInterface for IHtmlHtmlElement {
    // 98dc32d3-7a60-4892-84ea-037b64b352ea
    const INTERFACE_ID: [u8; 16] = [
        152u8, 220u8, 50u8, 211u8, 122u8, 96u8, 72u8, 146u8, 132u8, 234u8, 3u8, 123u8, 100u8,
        179u8, 82u8, 234u8,
    ];
}

// Class HtmlHtmlElement

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_HtmlHtmlElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlHtmlElement_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct HtmlHtmlElementCcw {
                IHtmlHtmlElement: felis::comdef::IHtmlHtmlElement,
                IRenderable: felis::comdef::IRenderable,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut HtmlHtmlElementCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            fn display(this: *const *const std::os::raw::c_void) -> crate::style::Display {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.layout(
                        pango_context,
                        style_computed,
                        content_boundary,
                    )
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.paint(renderer, style_computed)
                }
            }

            fn on_mouse_move(
                this: *const *const std::os::raw::c_void,
                x: f64,
                y: f64,
                window: &winit::window::Window,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_move(x, y, window)
                }
            }

            fn on_mouse_click(
                this: *const *const std::os::raw::c_void,
            ) -> crate::page::FelisAction {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_click()
                }
            }

            fn merge_style(
                this: *const *const std::os::raw::c_void,
                style: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.merge_style(style)
                }
            }

            fn get_attribute(
                this: *const *const std::os::raw::c_void,
                key: &str,
            ) -> Option<Option<String>> {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                    (*__crosscom_object).inner.0.get_attribute(key)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*__crosscom_object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*__crosscom_object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlHtmlElementVirtualTable_CCW_FOR_HtmlHtmlElement:
                felis::comdef::IHtmlHtmlElementVirtualTableCcw =
                felis::comdef::IHtmlHtmlElementVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IHtmlHtmlElementVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        id,
                        tag,
                        on_mouse_move,
                        on_mouse_click,
                        merge_style,
                        get_attribute,
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlHtmlElement:
                felis::comdef::IRenderableVirtualTableCcw =
                felis::comdef::IRenderableVirtualTableCcw {
                    offset: -1,
                    vtable: felis::comdef::IRenderableVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        display,
                        layout,
                        paint,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlHtmlElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IHtmlHtmlElement: felis::comdef::IHtmlHtmlElement {
                            vtable: &GLOBAL_IHtmlHtmlElementVirtualTable_CCW_FOR_HtmlHtmlElement
                                .vtable
                                as *const felis::comdef::IHtmlHtmlElementVirtualTable,
                        },

                        IRenderable: felis::comdef::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlHtmlElement.vtable
                                as *const felis::comdef::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this = this
                            .offset(-(crosscom::offset_of!(HtmlHtmlElementCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_HtmlHtmlElement;

// Interface IHtmlScriptElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IHtmlScriptElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub on_mouse_move: fn(
        this: *const *const std::os::raw::c_void,
        x: f64,
        y: f64,
        window: &winit::window::Window,
    ) -> crosscom::Void,
    pub on_mouse_click: fn(this: *const *const std::os::raw::c_void) -> crate::page::FelisAction,
    pub merge_style:
        fn(this: *const *const std::os::raw::c_void, style: &crate::style::Style) -> crosscom::Void,
    pub get_attribute:
        fn(this: *const *const std::os::raw::c_void, key: &str) -> Option<Option<String>>,
    pub text: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlScriptElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IHtmlScriptElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlScriptElement {
    pub vtable: *const IHtmlScriptElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IHtmlScriptElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_move)(this, x.into(), y.into(), window.into());

            ret
        }
    }

    pub fn on_mouse_click(&self) -> crate::page::FelisAction {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_click)(this);

            ret
        }
    }

    pub fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).merge_style)(this, style.into());

            ret
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<Option<String>> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_attribute)(this, key.into());

            ret
        }
    }

    pub fn text(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).text)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IHtmlScriptElement::INTERFACE_ID)
    }
}

pub trait IHtmlScriptElementImpl {
    fn text(&self) -> crosscom::ComRc<felis::comdef::IDomString>;
}

impl crosscom::ComInterface for IHtmlScriptElement {
    // 0f722326-a6fb-40c0-b66e-a88d50459859
    const INTERFACE_ID: [u8; 16] = [
        15u8, 114u8, 35u8, 38u8, 166u8, 251u8, 64u8, 192u8, 182u8, 110u8, 168u8, 141u8, 80u8, 69u8,
        152u8, 89u8,
    ];
}

// Class HtmlScriptElement

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_HtmlScriptElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlScriptElement_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct HtmlScriptElementCcw {
                IHtmlScriptElement: felis::comdef::IHtmlScriptElement,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlScriptElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut HtmlScriptElementCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn text(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*__crosscom_object).inner.0.text().into()
            }

            fn on_mouse_move(
                this: *const *const std::os::raw::c_void,
                x: f64,
                y: f64,
                window: &winit::window::Window,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_move(x, y, window)
                }
            }

            fn on_mouse_click(
                this: *const *const std::os::raw::c_void,
            ) -> crate::page::FelisAction {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_click()
                }
            }

            fn merge_style(
                this: *const *const std::os::raw::c_void,
                style: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                    (*__crosscom_object).inner.0.merge_style(style)
                }
            }

            fn get_attribute(
                this: *const *const std::os::raw::c_void,
                key: &str,
            ) -> Option<Option<String>> {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                    (*__crosscom_object).inner.0.get_attribute(key)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*__crosscom_object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*__crosscom_object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlScriptElementVirtualTable_CCW_FOR_HtmlScriptElement:
                felis::comdef::IHtmlScriptElementVirtualTableCcw =
                felis::comdef::IHtmlScriptElementVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IHtmlScriptElementVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        id,
                        tag,
                        on_mouse_move,
                        on_mouse_click,
                        merge_style,
                        get_attribute,
                        text,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlScriptElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IHtmlScriptElement: felis::comdef::IHtmlScriptElement {
                            vtable: &GLOBAL_IHtmlScriptElementVirtualTable_CCW_FOR_HtmlScriptElement
                                .vtable
                                as *const felis::comdef::IHtmlScriptElementVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this = this
                            .offset(-(crosscom::offset_of!(HtmlScriptElementCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_HtmlScriptElement;

// Interface IHtmlHeadElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IHtmlHeadElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub on_mouse_move: fn(
        this: *const *const std::os::raw::c_void,
        x: f64,
        y: f64,
        window: &winit::window::Window,
    ) -> crosscom::Void,
    pub on_mouse_click: fn(this: *const *const std::os::raw::c_void) -> crate::page::FelisAction,
    pub merge_style:
        fn(this: *const *const std::os::raw::c_void, style: &crate::style::Style) -> crosscom::Void,
    pub get_attribute:
        fn(this: *const *const std::os::raw::c_void, key: &str) -> Option<Option<String>>,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlHeadElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IHtmlHeadElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlHeadElement {
    pub vtable: *const IHtmlHeadElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IHtmlHeadElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_move)(this, x.into(), y.into(), window.into());

            ret
        }
    }

    pub fn on_mouse_click(&self) -> crate::page::FelisAction {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_click)(this);

            ret
        }
    }

    pub fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).merge_style)(this, style.into());

            ret
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<Option<String>> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_attribute)(this, key.into());

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IHtmlHeadElement::INTERFACE_ID)
    }
}

pub trait IHtmlHeadElementImpl {}

impl crosscom::ComInterface for IHtmlHeadElement {
    // 7c37faa1-4c84-49b4-85d2-0d48ef94f669
    const INTERFACE_ID: [u8; 16] = [
        124u8, 55u8, 250u8, 161u8, 76u8, 132u8, 73u8, 180u8, 133u8, 210u8, 13u8, 72u8, 239u8,
        148u8, 246u8, 105u8,
    ];
}

// Class HtmlHeadElement

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_HtmlHeadElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlHeadElement_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct HtmlHeadElementCcw {
                IHtmlHeadElement: felis::comdef::IHtmlHeadElement,
                IRenderable: felis::comdef::IRenderable,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlHeadElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut HtmlHeadElementCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            fn display(this: *const *const std::os::raw::c_void) -> crate::style::Display {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                    (*__crosscom_object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                    (*__crosscom_object).inner.0.layout(
                        pango_context,
                        style_computed,
                        content_boundary,
                    )
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                    (*__crosscom_object).inner.0.paint(renderer, style_computed)
                }
            }

            fn on_mouse_move(
                this: *const *const std::os::raw::c_void,
                x: f64,
                y: f64,
                window: &winit::window::Window,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_move(x, y, window)
                }
            }

            fn on_mouse_click(
                this: *const *const std::os::raw::c_void,
            ) -> crate::page::FelisAction {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_click()
                }
            }

            fn merge_style(
                this: *const *const std::os::raw::c_void,
                style: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                    (*__crosscom_object).inner.0.merge_style(style)
                }
            }

            fn get_attribute(
                this: *const *const std::os::raw::c_void,
                key: &str,
            ) -> Option<Option<String>> {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                    (*__crosscom_object).inner.0.get_attribute(key)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*__crosscom_object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*__crosscom_object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlHeadElementVirtualTable_CCW_FOR_HtmlHeadElement:
                felis::comdef::IHtmlHeadElementVirtualTableCcw =
                felis::comdef::IHtmlHeadElementVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IHtmlHeadElementVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        id,
                        tag,
                        on_mouse_move,
                        on_mouse_click,
                        merge_style,
                        get_attribute,
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlHeadElement:
                felis::comdef::IRenderableVirtualTableCcw =
                felis::comdef::IRenderableVirtualTableCcw {
                    offset: -1,
                    vtable: felis::comdef::IRenderableVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        display,
                        layout,
                        paint,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlHeadElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IHtmlHeadElement: felis::comdef::IHtmlHeadElement {
                            vtable: &GLOBAL_IHtmlHeadElementVirtualTable_CCW_FOR_HtmlHeadElement
                                .vtable
                                as *const felis::comdef::IHtmlHeadElementVirtualTable,
                        },

                        IRenderable: felis::comdef::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlHeadElement.vtable
                                as *const felis::comdef::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this = this
                            .offset(-(crosscom::offset_of!(HtmlHeadElementCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_HtmlHeadElement;

// Interface IHtmlLinkElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IHtmlLinkElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub on_mouse_move: fn(
        this: *const *const std::os::raw::c_void,
        x: f64,
        y: f64,
        window: &winit::window::Window,
    ) -> crosscom::Void,
    pub on_mouse_click: fn(this: *const *const std::os::raw::c_void) -> crate::page::FelisAction,
    pub merge_style:
        fn(this: *const *const std::os::raw::c_void, style: &crate::style::Style) -> crosscom::Void,
    pub get_attribute:
        fn(this: *const *const std::os::raw::c_void, key: &str) -> Option<Option<String>>,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlLinkElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IHtmlLinkElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlLinkElement {
    pub vtable: *const IHtmlLinkElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IHtmlLinkElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_move)(this, x.into(), y.into(), window.into());

            ret
        }
    }

    pub fn on_mouse_click(&self) -> crate::page::FelisAction {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_click)(this);

            ret
        }
    }

    pub fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).merge_style)(this, style.into());

            ret
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<Option<String>> {
        unsafe {
            let this = self as *const IHtmlLinkElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_attribute)(this, key.into());

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IHtmlLinkElement::INTERFACE_ID)
    }
}

pub trait IHtmlLinkElementImpl {}

impl crosscom::ComInterface for IHtmlLinkElement {
    // 850ed2e0-99a4-4e2c-b9ea-86259daba13c
    const INTERFACE_ID: [u8; 16] = [
        133u8, 14u8, 210u8, 224u8, 153u8, 164u8, 78u8, 44u8, 185u8, 234u8, 134u8, 37u8, 157u8,
        171u8, 161u8, 60u8,
    ];
}

// Class HtmlLinkElement

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_HtmlLinkElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlLinkElement_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct HtmlLinkElementCcw {
                IHtmlLinkElement: felis::comdef::IHtmlLinkElement,
                IRenderable: felis::comdef::IRenderable,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlLinkElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlLinkElementCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut HtmlLinkElementCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            fn display(this: *const *const std::os::raw::c_void) -> crate::style::Display {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                    (*__crosscom_object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                    (*__crosscom_object).inner.0.layout(
                        pango_context,
                        style_computed,
                        content_boundary,
                    )
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                    (*__crosscom_object).inner.0.paint(renderer, style_computed)
                }
            }

            fn on_mouse_move(
                this: *const *const std::os::raw::c_void,
                x: f64,
                y: f64,
                window: &winit::window::Window,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_move(x, y, window)
                }
            }

            fn on_mouse_click(
                this: *const *const std::os::raw::c_void,
            ) -> crate::page::FelisAction {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_click()
                }
            }

            fn merge_style(
                this: *const *const std::os::raw::c_void,
                style: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                    (*__crosscom_object).inner.0.merge_style(style)
                }
            }

            fn get_attribute(
                this: *const *const std::os::raw::c_void,
                key: &str,
            ) -> Option<Option<String>> {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                    (*__crosscom_object).inner.0.get_attribute(key)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                (*__crosscom_object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                (*__crosscom_object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<HtmlLinkElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlLinkElementVirtualTable_CCW_FOR_HtmlLinkElement:
                felis::comdef::IHtmlLinkElementVirtualTableCcw =
                felis::comdef::IHtmlLinkElementVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IHtmlLinkElementVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        id,
                        tag,
                        on_mouse_move,
                        on_mouse_click,
                        merge_style,
                        get_attribute,
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlLinkElement:
                felis::comdef::IRenderableVirtualTableCcw =
                felis::comdef::IRenderableVirtualTableCcw {
                    offset: -1,
                    vtable: felis::comdef::IRenderableVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        display,
                        layout,
                        paint,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlLinkElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IHtmlLinkElement: felis::comdef::IHtmlLinkElement {
                            vtable: &GLOBAL_IHtmlLinkElementVirtualTable_CCW_FOR_HtmlLinkElement
                                .vtable
                                as *const felis::comdef::IHtmlLinkElementVirtualTable,
                        },

                        IRenderable: felis::comdef::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlLinkElement.vtable
                                as *const felis::comdef::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this = this
                            .offset(-(crosscom::offset_of!(HtmlLinkElementCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_HtmlLinkElement;

// Interface IHtmlBodyElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IHtmlBodyElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub on_mouse_move: fn(
        this: *const *const std::os::raw::c_void,
        x: f64,
        y: f64,
        window: &winit::window::Window,
    ) -> crosscom::Void,
    pub on_mouse_click: fn(this: *const *const std::os::raw::c_void) -> crate::page::FelisAction,
    pub merge_style:
        fn(this: *const *const std::os::raw::c_void, style: &crate::style::Style) -> crosscom::Void,
    pub get_attribute:
        fn(this: *const *const std::os::raw::c_void, key: &str) -> Option<Option<String>>,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlBodyElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IHtmlBodyElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlBodyElement {
    pub vtable: *const IHtmlBodyElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IHtmlBodyElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_move)(this, x.into(), y.into(), window.into());

            ret
        }
    }

    pub fn on_mouse_click(&self) -> crate::page::FelisAction {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_click)(this);

            ret
        }
    }

    pub fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).merge_style)(this, style.into());

            ret
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<Option<String>> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_attribute)(this, key.into());

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IHtmlBodyElement::INTERFACE_ID)
    }
}

pub trait IHtmlBodyElementImpl {}

impl crosscom::ComInterface for IHtmlBodyElement {
    // 622e1b69-cb5a-442a-afd3-e8fa7d43cfeb
    const INTERFACE_ID: [u8; 16] = [
        98u8, 46u8, 27u8, 105u8, 203u8, 90u8, 68u8, 42u8, 175u8, 211u8, 232u8, 250u8, 125u8, 67u8,
        207u8, 235u8,
    ];
}

// Class HtmlBodyElement

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_HtmlBodyElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlBodyElement_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct HtmlBodyElementCcw {
                IHtmlBodyElement: felis::comdef::IHtmlBodyElement,
                IRenderable: felis::comdef::IRenderable,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlBodyElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut HtmlBodyElementCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            fn display(this: *const *const std::os::raw::c_void) -> crate::style::Display {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                    (*__crosscom_object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                    (*__crosscom_object).inner.0.layout(
                        pango_context,
                        style_computed,
                        content_boundary,
                    )
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                    (*__crosscom_object).inner.0.paint(renderer, style_computed)
                }
            }

            fn on_mouse_move(
                this: *const *const std::os::raw::c_void,
                x: f64,
                y: f64,
                window: &winit::window::Window,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_move(x, y, window)
                }
            }

            fn on_mouse_click(
                this: *const *const std::os::raw::c_void,
            ) -> crate::page::FelisAction {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_click()
                }
            }

            fn merge_style(
                this: *const *const std::os::raw::c_void,
                style: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                    (*__crosscom_object).inner.0.merge_style(style)
                }
            }

            fn get_attribute(
                this: *const *const std::os::raw::c_void,
                key: &str,
            ) -> Option<Option<String>> {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                    (*__crosscom_object).inner.0.get_attribute(key)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*__crosscom_object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*__crosscom_object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlBodyElementVirtualTable_CCW_FOR_HtmlBodyElement:
                felis::comdef::IHtmlBodyElementVirtualTableCcw =
                felis::comdef::IHtmlBodyElementVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IHtmlBodyElementVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        id,
                        tag,
                        on_mouse_move,
                        on_mouse_click,
                        merge_style,
                        get_attribute,
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlBodyElement:
                felis::comdef::IRenderableVirtualTableCcw =
                felis::comdef::IRenderableVirtualTableCcw {
                    offset: -1,
                    vtable: felis::comdef::IRenderableVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        display,
                        layout,
                        paint,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlBodyElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IHtmlBodyElement: felis::comdef::IHtmlBodyElement {
                            vtable: &GLOBAL_IHtmlBodyElementVirtualTable_CCW_FOR_HtmlBodyElement
                                .vtable
                                as *const felis::comdef::IHtmlBodyElementVirtualTable,
                        },

                        IRenderable: felis::comdef::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlBodyElement.vtable
                                as *const felis::comdef::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this = this
                            .offset(-(crosscom::offset_of!(HtmlBodyElementCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_HtmlBodyElement;

// Interface IHtmlParagraphElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IHtmlParagraphElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub on_mouse_move: fn(
        this: *const *const std::os::raw::c_void,
        x: f64,
        y: f64,
        window: &winit::window::Window,
    ) -> crosscom::Void,
    pub on_mouse_click: fn(this: *const *const std::os::raw::c_void) -> crate::page::FelisAction,
    pub merge_style:
        fn(this: *const *const std::os::raw::c_void, style: &crate::style::Style) -> crosscom::Void,
    pub get_attribute:
        fn(this: *const *const std::os::raw::c_void, key: &str) -> Option<Option<String>>,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlParagraphElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IHtmlParagraphElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlParagraphElement {
    pub vtable: *const IHtmlParagraphElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IHtmlParagraphElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_move)(this, x.into(), y.into(), window.into());

            ret
        }
    }

    pub fn on_mouse_click(&self) -> crate::page::FelisAction {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_click)(this);

            ret
        }
    }

    pub fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).merge_style)(this, style.into());

            ret
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<Option<String>> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_attribute)(this, key.into());

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IHtmlParagraphElement::INTERFACE_ID)
    }
}

pub trait IHtmlParagraphElementImpl {}

impl crosscom::ComInterface for IHtmlParagraphElement {
    // ebf22804-343a-431b-9719-f44396a5d0a8
    const INTERFACE_ID: [u8; 16] = [
        235u8, 242u8, 40u8, 4u8, 52u8, 58u8, 67u8, 27u8, 151u8, 25u8, 244u8, 67u8, 150u8, 165u8,
        208u8, 168u8,
    ];
}

// Class HtmlParagraphElement

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_HtmlParagraphElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlParagraphElement_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct HtmlParagraphElementCcw {
                IHtmlParagraphElement: felis::comdef::IHtmlParagraphElement,
                IRenderable: felis::comdef::IRenderable,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlParagraphElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut HtmlParagraphElementCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            fn display(this: *const *const std::os::raw::c_void) -> crate::style::Display {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                    (*__crosscom_object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                    (*__crosscom_object).inner.0.layout(
                        pango_context,
                        style_computed,
                        content_boundary,
                    )
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                    (*__crosscom_object).inner.0.paint(renderer, style_computed)
                }
            }

            fn on_mouse_move(
                this: *const *const std::os::raw::c_void,
                x: f64,
                y: f64,
                window: &winit::window::Window,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_move(x, y, window)
                }
            }

            fn on_mouse_click(
                this: *const *const std::os::raw::c_void,
            ) -> crate::page::FelisAction {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_click()
                }
            }

            fn merge_style(
                this: *const *const std::os::raw::c_void,
                style: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                    (*__crosscom_object).inner.0.merge_style(style)
                }
            }

            fn get_attribute(
                this: *const *const std::os::raw::c_void,
                key: &str,
            ) -> Option<Option<String>> {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                    (*__crosscom_object).inner.0.get_attribute(key)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*__crosscom_object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*__crosscom_object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlParagraphElementVirtualTable_CCW_FOR_HtmlParagraphElement:
                felis::comdef::IHtmlParagraphElementVirtualTableCcw =
                felis::comdef::IHtmlParagraphElementVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IHtmlParagraphElementVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        id,
                        tag,
                        on_mouse_move,
                        on_mouse_click,
                        merge_style,
                        get_attribute,
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlParagraphElement:
                felis::comdef::IRenderableVirtualTableCcw =
                felis::comdef::IRenderableVirtualTableCcw {
                    offset: -1,
                    vtable: felis::comdef::IRenderableVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        display,
                        layout,
                        paint,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlParagraphElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {

        IHtmlParagraphElement: felis::comdef::IHtmlParagraphElement {
            vtable: &GLOBAL_IHtmlParagraphElementVirtualTable_CCW_FOR_HtmlParagraphElement.vtable
                as *const felis::comdef::IHtmlParagraphElementVirtualTable,
        },

        IRenderable: felis::comdef::IRenderable {
            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlParagraphElement.vtable
                as *const felis::comdef::IRenderableVirtualTable,
        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this = this.offset(
                            -(crosscom::offset_of!(HtmlParagraphElementCcw, inner) as isize),
                        );
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_HtmlParagraphElement;

// Interface IHtmlDivElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IHtmlDivElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub on_mouse_move: fn(
        this: *const *const std::os::raw::c_void,
        x: f64,
        y: f64,
        window: &winit::window::Window,
    ) -> crosscom::Void,
    pub on_mouse_click: fn(this: *const *const std::os::raw::c_void) -> crate::page::FelisAction,
    pub merge_style:
        fn(this: *const *const std::os::raw::c_void, style: &crate::style::Style) -> crosscom::Void,
    pub get_attribute:
        fn(this: *const *const std::os::raw::c_void, key: &str) -> Option<Option<String>>,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlDivElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IHtmlDivElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlDivElement {
    pub vtable: *const IHtmlDivElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IHtmlDivElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_move)(this, x.into(), y.into(), window.into());

            ret
        }
    }

    pub fn on_mouse_click(&self) -> crate::page::FelisAction {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_click)(this);

            ret
        }
    }

    pub fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).merge_style)(this, style.into());

            ret
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<Option<String>> {
        unsafe {
            let this = self as *const IHtmlDivElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_attribute)(this, key.into());

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IHtmlDivElement::INTERFACE_ID)
    }
}

pub trait IHtmlDivElementImpl {}

impl crosscom::ComInterface for IHtmlDivElement {
    // 0ccec5a8-23af-4b91-b4f9-27ebe63dcd6f
    const INTERFACE_ID: [u8; 16] = [
        12u8, 206u8, 197u8, 168u8, 35u8, 175u8, 75u8, 145u8, 180u8, 249u8, 39u8, 235u8, 230u8,
        61u8, 205u8, 111u8,
    ];
}

// Class HtmlDivElement

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_HtmlDivElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlDivElement_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct HtmlDivElementCcw {
                IHtmlDivElement: felis::comdef::IHtmlDivElement,
                IRenderable: felis::comdef::IRenderable,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlDivElementCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlDivElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlDivElementCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlDivElementCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut HtmlDivElementCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            fn display(this: *const *const std::os::raw::c_void) -> crate::style::Display {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                    (*__crosscom_object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                    (*__crosscom_object).inner.0.layout(
                        pango_context,
                        style_computed,
                        content_boundary,
                    )
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                    (*__crosscom_object).inner.0.paint(renderer, style_computed)
                }
            }

            fn on_mouse_move(
                this: *const *const std::os::raw::c_void,
                x: f64,
                y: f64,
                window: &winit::window::Window,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_move(x, y, window)
                }
            }

            fn on_mouse_click(
                this: *const *const std::os::raw::c_void,
            ) -> crate::page::FelisAction {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_click()
                }
            }

            fn merge_style(
                this: *const *const std::os::raw::c_void,
                style: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                    (*__crosscom_object).inner.0.merge_style(style)
                }
            }

            fn get_attribute(
                this: *const *const std::os::raw::c_void,
                key: &str,
            ) -> Option<Option<String>> {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                    (*__crosscom_object).inner.0.get_attribute(key)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                (*__crosscom_object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                (*__crosscom_object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<HtmlDivElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlDivElementVirtualTable_CCW_FOR_HtmlDivElement:
                felis::comdef::IHtmlDivElementVirtualTableCcw =
                felis::comdef::IHtmlDivElementVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IHtmlDivElementVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        id,
                        tag,
                        on_mouse_move,
                        on_mouse_click,
                        merge_style,
                        get_attribute,
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlDivElement:
                felis::comdef::IRenderableVirtualTableCcw =
                felis::comdef::IRenderableVirtualTableCcw {
                    offset: -1,
                    vtable: felis::comdef::IRenderableVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        display,
                        layout,
                        paint,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlDivElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IHtmlDivElement: felis::comdef::IHtmlDivElement {
                            vtable: &GLOBAL_IHtmlDivElementVirtualTable_CCW_FOR_HtmlDivElement
                                .vtable
                                as *const felis::comdef::IHtmlDivElementVirtualTable,
                        },

                        IRenderable: felis::comdef::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlDivElement.vtable
                                as *const felis::comdef::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this =
                            this.offset(-(crosscom::offset_of!(HtmlDivElementCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_HtmlDivElement;

// Interface IHtmlImageElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IHtmlImageElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub on_mouse_move: fn(
        this: *const *const std::os::raw::c_void,
        x: f64,
        y: f64,
        window: &winit::window::Window,
    ) -> crosscom::Void,
    pub on_mouse_click: fn(this: *const *const std::os::raw::c_void) -> crate::page::FelisAction,
    pub merge_style:
        fn(this: *const *const std::os::raw::c_void, style: &crate::style::Style) -> crosscom::Void,
    pub get_attribute:
        fn(this: *const *const std::os::raw::c_void, key: &str) -> Option<Option<String>>,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlImageElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IHtmlImageElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlImageElement {
    pub vtable: *const IHtmlImageElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IHtmlImageElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_move)(this, x.into(), y.into(), window.into());

            ret
        }
    }

    pub fn on_mouse_click(&self) -> crate::page::FelisAction {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_click)(this);

            ret
        }
    }

    pub fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).merge_style)(this, style.into());

            ret
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<Option<String>> {
        unsafe {
            let this = self as *const IHtmlImageElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_attribute)(this, key.into());

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IHtmlImageElement::INTERFACE_ID)
    }
}

pub trait IHtmlImageElementImpl {}

impl crosscom::ComInterface for IHtmlImageElement {
    // 066ad66a-267b-46e7-87ed-4d094635ab91
    const INTERFACE_ID: [u8; 16] = [
        6u8, 106u8, 214u8, 106u8, 38u8, 123u8, 70u8, 231u8, 135u8, 237u8, 77u8, 9u8, 70u8, 53u8,
        171u8, 145u8,
    ];
}

// Class HtmlImageElement

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_HtmlImageElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlImageElement_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct HtmlImageElementCcw {
                IHtmlImageElement: felis::comdef::IHtmlImageElement,
                IRenderable: felis::comdef::IRenderable,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlImageElementCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlImageElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlImageElementCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlImageElementCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut HtmlImageElementCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            fn display(this: *const *const std::os::raw::c_void) -> crate::style::Display {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                    (*__crosscom_object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                    (*__crosscom_object).inner.0.layout(
                        pango_context,
                        style_computed,
                        content_boundary,
                    )
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                    (*__crosscom_object).inner.0.paint(renderer, style_computed)
                }
            }

            fn on_mouse_move(
                this: *const *const std::os::raw::c_void,
                x: f64,
                y: f64,
                window: &winit::window::Window,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_move(x, y, window)
                }
            }

            fn on_mouse_click(
                this: *const *const std::os::raw::c_void,
            ) -> crate::page::FelisAction {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_click()
                }
            }

            fn merge_style(
                this: *const *const std::os::raw::c_void,
                style: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                    (*__crosscom_object).inner.0.merge_style(style)
                }
            }

            fn get_attribute(
                this: *const *const std::os::raw::c_void,
                key: &str,
            ) -> Option<Option<String>> {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                    (*__crosscom_object).inner.0.get_attribute(key)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                (*__crosscom_object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                (*__crosscom_object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<HtmlImageElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlImageElementVirtualTable_CCW_FOR_HtmlImageElement:
                felis::comdef::IHtmlImageElementVirtualTableCcw =
                felis::comdef::IHtmlImageElementVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IHtmlImageElementVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        id,
                        tag,
                        on_mouse_move,
                        on_mouse_click,
                        merge_style,
                        get_attribute,
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlImageElement:
                felis::comdef::IRenderableVirtualTableCcw =
                felis::comdef::IRenderableVirtualTableCcw {
                    offset: -1,
                    vtable: felis::comdef::IRenderableVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        display,
                        layout,
                        paint,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlImageElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IHtmlImageElement: felis::comdef::IHtmlImageElement {
                            vtable: &GLOBAL_IHtmlImageElementVirtualTable_CCW_FOR_HtmlImageElement
                                .vtable
                                as *const felis::comdef::IHtmlImageElementVirtualTable,
                        },

                        IRenderable: felis::comdef::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlImageElement.vtable
                                as *const felis::comdef::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this = this
                            .offset(-(crosscom::offset_of!(HtmlImageElementCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_HtmlImageElement;

// Interface IHtmlStyleElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IHtmlStyleElementVirtualTable {
    pub query_interface: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long,
    pub add_ref:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub release:
        unsafe extern "system" fn(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long,
    pub children: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub outer_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub set_inner_html: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        html: *const *const std::os::raw::c_void,
    ) -> (),
    pub get_elements_by_tag_name: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        tag: *const *const std::os::raw::c_void,
    )
        -> *const *const std::os::raw::c_void,
    pub get_element_by_id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
        id: *const *const std::os::raw::c_void,
    ) -> crosscom::RawPointer,
    pub id: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub tag: unsafe extern "system" fn(
        this: *const *const std::os::raw::c_void,
    ) -> *const *const std::os::raw::c_void,
    pub on_mouse_move: fn(
        this: *const *const std::os::raw::c_void,
        x: f64,
        y: f64,
        window: &winit::window::Window,
    ) -> crosscom::Void,
    pub on_mouse_click: fn(this: *const *const std::os::raw::c_void) -> crate::page::FelisAction,
    pub merge_style:
        fn(this: *const *const std::os::raw::c_void, style: &crate::style::Style) -> crosscom::Void,
    pub get_attribute:
        fn(this: *const *const std::os::raw::c_void, key: &str) -> Option<Option<String>>,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlStyleElementVirtualTableCcw {
    pub offset: isize,
    pub vtable: IHtmlStyleElementVirtualTable,
}

#[repr(C)]
#[allow(dead_code)]
pub struct IHtmlStyleElement {
    pub vtable: *const IHtmlStyleElementVirtualTable,
}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl IHtmlStyleElement {
    pub fn query_interface<T: crosscom::ComInterface>(&self) -> Option<crosscom::ComRc<T>> {
        let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
        let mut raw = 0 as *const *const std::os::raw::c_void;
        let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
        let ret_val = unsafe { ((*self.vtable).query_interface)(this, guid, &mut raw) };
        if ret_val != 0 {
            None
        } else {
            Some(unsafe { crosscom::ComRc::<T>::from_raw_pointer(raw) })
        }
    }

    pub fn add_ref(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).add_ref)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn release(&self) -> std::os::raw::c_long {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).release)(this);
            let ret: std::os::raw::c_long = ret.into();

            ret
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<felis::comdef::INode> {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).children)(this);
            let ret: crosscom::ObjectArray<felis::comdef::INode> = ret.into();

            ret
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).inner_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).outer_html)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<felis::comdef::IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).set_inner_html)(this, html.into());
            let ret: () = ret.into();

            ret
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> crosscom::ObjectArray<felis::comdef::IElement> {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_elements_by_tag_name)(this, tag.into());
            let ret: crosscom::ObjectArray<felis::comdef::IElement> = ret.into();

            ret
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<felis::comdef::IDomString>,
    ) -> Option<crosscom::ComRc<felis::comdef::IElement>> {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_element_by_id)(this, id.into());
            let ret: Option<crosscom::ComRc<felis::comdef::IElement>> = ret.into();

            ret
        }
    }

    pub fn id(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).id)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<felis::comdef::IDomString> {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).tag)(this);
            let ret: crosscom::ComRc<IDomString> = ret.into();

            ret
        }
    }

    pub fn on_mouse_move(&self, x: f64, y: f64, window: &winit::window::Window) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_move)(this, x.into(), y.into(), window.into());

            ret
        }
    }

    pub fn on_mouse_click(&self) -> crate::page::FelisAction {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).on_mouse_click)(this);

            ret
        }
    }

    pub fn merge_style(&self, style: &crate::style::Style) -> crosscom::Void {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).merge_style)(this, style.into());

            ret
        }
    }

    pub fn get_attribute(&self, key: &str) -> Option<Option<String>> {
        unsafe {
            let this = self as *const IHtmlStyleElement as *const *const std::os::raw::c_void;
            let ret = ((*self.vtable).get_attribute)(this, key.into());

            ret
        }
    }

    pub fn uuid() -> uuid::Uuid {
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes(IHtmlStyleElement::INTERFACE_ID)
    }
}

pub trait IHtmlStyleElementImpl {}

impl crosscom::ComInterface for IHtmlStyleElement {
    // e6665467-847f-4aef-91f7-cfdc5e59ff8f
    const INTERFACE_ID: [u8; 16] = [
        230u8, 102u8, 84u8, 103u8, 132u8, 127u8, 74u8, 239u8, 145u8, 247u8, 207u8, 220u8, 94u8,
        89u8, 255u8, 143u8,
    ];
}

// Class HtmlStyleElement

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_HtmlStyleElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlStyleElement_crosscom_impl {
            use crate as felis;
            use crosscom::ComInterface;
            use crosscom::IObjectArrayImpl;
            use crosscom::IUnknownImpl;
            use felis::comdef::ICharacterDataImpl;
            use felis::comdef::IDomStringImpl;
            use felis::comdef::IElementImpl;
            use felis::comdef::IHtmlBodyElementImpl;
            use felis::comdef::IHtmlDivElementImpl;
            use felis::comdef::IHtmlElementImpl;
            use felis::comdef::IHtmlHeadElementImpl;
            use felis::comdef::IHtmlHtmlElementImpl;
            use felis::comdef::IHtmlImageElementImpl;
            use felis::comdef::IHtmlLinkElementImpl;
            use felis::comdef::IHtmlParagraphElementImpl;
            use felis::comdef::IHtmlScriptElementImpl;
            use felis::comdef::IHtmlStyleElementImpl;
            use felis::comdef::INodeImpl;
            use felis::comdef::IRenderableImpl;
            use felis::comdef::ITextImpl;

            #[repr(C)]
            pub struct HtmlStyleElementCcw {
                IHtmlStyleElement: felis::comdef::IHtmlStyleElement,

                ref_count: std::sync::atomic::AtomicU32,
                pub inner: $impl_type,
            }

            unsafe extern "system" fn query_interface(
                this: *const *const std::os::raw::c_void,
                guid: uuid::Uuid,
                retval: &mut *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                match guid.as_bytes() {
                    &crosscom::IUnknown::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    &felis::comdef::IHtmlStyleElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as std::os::raw::c_long
                    }

                    _ => crosscom::ResultCode::ENoInterface as std::os::raw::c_long,
                }
            }

            unsafe extern "system" fn add_ref(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                let previous = (*object)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                (previous + 1) as std::os::raw::c_long
            }

            unsafe extern "system" fn release(
                this: *const *const std::os::raw::c_void,
            ) -> std::os::raw::c_long {
                let object = crosscom::get_object::<HtmlStyleElementCcw>(this);

                let previous = (*object)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
                if previous - 1 == 0 {
                    Box::from_raw(object as *mut HtmlStyleElementCcw);
                }

                (previous - 1) as std::os::raw::c_long
            }

            fn on_mouse_move(
                this: *const *const std::os::raw::c_void,
                x: f64,
                y: f64,
                window: &winit::window::Window,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_move(x, y, window)
                }
            }

            fn on_mouse_click(
                this: *const *const std::os::raw::c_void,
            ) -> crate::page::FelisAction {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                    (*__crosscom_object).inner.0.on_mouse_click()
                }
            }

            fn merge_style(
                this: *const *const std::os::raw::c_void,
                style: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                    (*__crosscom_object).inner.0.merge_style(style)
                }
            }

            fn get_attribute(
                this: *const *const std::os::raw::c_void,
                key: &str,
            ) -> Option<Option<String>> {
                unsafe {
                    let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                    (*__crosscom_object).inner.0.get_attribute(key)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                (*__crosscom_object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                (*__crosscom_object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                (*__crosscom_object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                (*__crosscom_object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                (*__crosscom_object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let html: crosscom::ComRc<felis::comdef::IDomString> = html.into();

                let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .set_inner_html(html.into())
                    .into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let tag: crosscom::ComRc<felis::comdef::IDomString> = tag.into();

                let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let id: crosscom::ComRc<felis::comdef::IDomString> = id.into();

                let __crosscom_object = crosscom::get_object::<HtmlStyleElementCcw>(this);
                (*__crosscom_object)
                    .inner
                    .0
                    .get_element_by_id(id.into())
                    .into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlStyleElementVirtualTable_CCW_FOR_HtmlStyleElement:
                felis::comdef::IHtmlStyleElementVirtualTableCcw =
                felis::comdef::IHtmlStyleElementVirtualTableCcw {
                    offset: 0,
                    vtable: felis::comdef::IHtmlStyleElementVirtualTable {
                        query_interface,
                        add_ref,
                        release,
                        children,
                        inner_html,
                        outer_html,
                        set_inner_html,
                        get_elements_by_tag_name,
                        get_element_by_id,
                        id,
                        tag,
                        on_mouse_move,
                        on_mouse_click,
                        merge_style,
                        get_attribute,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlStyleElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IHtmlStyleElement: felis::comdef::IHtmlStyleElement {
                            vtable: &GLOBAL_IHtmlStyleElementVirtualTable_CCW_FOR_HtmlStyleElement
                                .vtable
                                as *const felis::comdef::IHtmlStyleElementVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }

                fn get_ccw(&self) -> &Self::CcwType {
                    unsafe {
                        let this = self as *const _ as *const u8;
                        let this = this
                            .offset(-(crosscom::offset_of!(HtmlStyleElementCcw, inner) as isize));
                        &*(this as *const Self::CcwType)
                    }
                }
            }
        }
    };
}

// pub use ComObject_HtmlStyleElement;
