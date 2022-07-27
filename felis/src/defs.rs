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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const IRenderable as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const IRenderable as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn display(&self) -> crate::style::Display {
        unsafe {
            let this = self as *const IRenderable as *const *const std::os::raw::c_void;
            ((*self.vtable).display)(this).into()
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
            ((*self.vtable).layout)(
                this,
                pango_context.into(),
                style_computed.into(),
                content_boundary.into(),
            )
            .into()
        }
    }

    pub fn paint(
        &self,
        renderer: &crate::rendering::cairo::CairoRenderer,
        style_computed: &crate::style::Style,
    ) -> crosscom::Void {
        unsafe {
            let this = self as *const IRenderable as *const *const std::os::raw::c_void;
            ((*self.vtable).paint)(this, renderer.into(), style_computed.into()).into()
        }
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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const IDomString as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const IDomString as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn bytes(&self) -> *const std::os::raw::c_uchar {
        unsafe {
            let this = self as *const IDomString as *const *const std::os::raw::c_void;
            ((*self.vtable).bytes)(this).into()
        }
    }

    pub fn str(&self) -> crosscom::StaticStr {
        unsafe {
            let this = self as *const IDomString as *const *const std::os::raw::c_void;
            ((*self.vtable).str)(this).into()
        }
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
macro_rules! ComObject_DomString {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod DomString_crosscom_impl {
            use crate::defs::ICharacterDataImpl;
            use crate::defs::IDomStringImpl;
            use crate::defs::IElementImpl;
            use crate::defs::IHtmlBodyElementImpl;
            use crate::defs::IHtmlElementImpl;
            use crate::defs::IHtmlHeadElementImpl;
            use crate::defs::IHtmlHtmlElementImpl;
            use crate::defs::IHtmlParagraphElementImpl;
            use crate::defs::IHtmlScriptElementImpl;
            use crate::defs::INodeImpl;
            use crate::defs::IRenderableImpl;
            use crate::defs::ITextImpl;
            use crosscom::ComInterface;

            #[repr(C)]
            pub struct DomStringCcw {
                IDomString: crate::defs::IDomString,

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
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IDomString::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    _ => crosscom::ResultCode::ENoInterface as i32,
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
                let object = crosscom::get_object::<DomStringCcw>(this);
                (*object).inner.bytes().into()
            }

            fn str(this: *const *const std::os::raw::c_void) -> crosscom::StaticStr {
                unsafe {
                    let object = crosscom::get_object::<DomStringCcw>(this);
                    (*object).inner.str()
                }
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IDomStringVirtualTable_CCW_FOR_DomString:
                crate::defs::IDomStringVirtualTableCcw = crate::defs::IDomStringVirtualTableCcw {
                offset: 0,
                vtable: crate::defs::IDomStringVirtualTable {
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
                        IDomString: crate::defs::IDomString {
                            vtable: &GLOBAL_IDomStringVirtualTable_CCW_FOR_DomString.vtable
                                as *const crate::defs::IDomStringVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }
            }
        }
    };
}

pub(crate) use ComObject_DomString;

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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<crate::defs::INode> {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            ((*self.vtable).children)(this).into()
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            ((*self.vtable).inner_html)(this).into()
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            ((*self.vtable).outer_html)(this).into()
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> () {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            ((*self.vtable).set_inner_html)(this, html.into()).into()
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement> {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            ((*self.vtable).get_elements_by_tag_name)(this, tag.into()).into()
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>> {
        unsafe {
            let this = self as *const INode as *const *const std::os::raw::c_void;
            ((*self.vtable).get_element_by_id)(this, id.into()).into()
        }
    }
}

pub trait INodeImpl {
    fn children(&self) -> crosscom::ObjectArray<crate::defs::INode>;
    fn inner_html(&self) -> crosscom::ComRc<IDomString>;
    fn outer_html(&self) -> crosscom::ComRc<IDomString>;
    fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> ();
    fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement>;
    fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>>;
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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<crate::defs::INode> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            ((*self.vtable).children)(this).into()
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            ((*self.vtable).inner_html)(this).into()
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            ((*self.vtable).outer_html)(this).into()
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> () {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            ((*self.vtable).set_inner_html)(this, html.into()).into()
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_elements_by_tag_name)(this, tag.into()).into()
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_element_by_id)(this, id.into()).into()
        }
    }

    pub fn id(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            ((*self.vtable).id)(this).into()
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IElement as *const *const std::os::raw::c_void;
            ((*self.vtable).tag)(this).into()
        }
    }
}

pub trait IElementImpl {
    fn id(&self) -> crosscom::ComRc<IDomString>;
    fn tag(&self) -> crosscom::ComRc<IDomString>;
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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<crate::defs::INode> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            ((*self.vtable).children)(this).into()
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            ((*self.vtable).inner_html)(this).into()
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            ((*self.vtable).outer_html)(this).into()
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> () {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            ((*self.vtable).set_inner_html)(this, html.into()).into()
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            ((*self.vtable).get_elements_by_tag_name)(this, tag.into()).into()
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            ((*self.vtable).get_element_by_id)(this, id.into()).into()
        }
    }

    pub fn text(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
            ((*self.vtable).text)(this).into()
        }
    }
}

pub trait ICharacterDataImpl {
    fn text(&self) -> crosscom::ComRc<IDomString>;
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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<crate::defs::INode> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            ((*self.vtable).children)(this).into()
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            ((*self.vtable).inner_html)(this).into()
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            ((*self.vtable).outer_html)(this).into()
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> () {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            ((*self.vtable).set_inner_html)(this, html.into()).into()
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            ((*self.vtable).get_elements_by_tag_name)(this, tag.into()).into()
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            ((*self.vtable).get_element_by_id)(this, id.into()).into()
        }
    }

    pub fn text(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IText as *const *const std::os::raw::c_void;
            ((*self.vtable).text)(this).into()
        }
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
macro_rules! ComObject_Text {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod Text_crosscom_impl {
            use crate::defs::ICharacterDataImpl;
            use crate::defs::IDomStringImpl;
            use crate::defs::IElementImpl;
            use crate::defs::IHtmlBodyElementImpl;
            use crate::defs::IHtmlElementImpl;
            use crate::defs::IHtmlHeadElementImpl;
            use crate::defs::IHtmlHtmlElementImpl;
            use crate::defs::IHtmlParagraphElementImpl;
            use crate::defs::IHtmlScriptElementImpl;
            use crate::defs::INodeImpl;
            use crate::defs::IRenderableImpl;
            use crate::defs::ITextImpl;
            use crosscom::ComInterface;

            #[repr(C)]
            pub struct TextCcw {
                IText: crate::defs::IText,
                IRenderable: crate::defs::IRenderable,

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
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::ICharacterData::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IText::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    _ => crosscom::ResultCode::ENoInterface as i32,
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
                    let object = crosscom::get_object::<TextCcw>(this);
                    (*object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let object = crosscom::get_object::<TextCcw>(this);
                    (*object)
                        .inner
                        .0
                        .layout(pango_context, style_computed, content_boundary)
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let object = crosscom::get_object::<TextCcw>(this);
                    (*object).inner.0.paint(renderer, style_computed)
                }
            }

            unsafe extern "system" fn text(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<TextCcw>(this);
                (*object).inner.0.text().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<TextCcw>(this);
                (*object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<TextCcw>(this);
                (*object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<TextCcw>(this);
                (*object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let object = crosscom::get_object::<TextCcw>(this);
                (*object).inner.0.set_inner_html(html.into()).into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<TextCcw>(this);
                (*object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let object = crosscom::get_object::<TextCcw>(this);
                (*object).inner.0.get_element_by_id(id.into()).into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_ITextVirtualTable_CCW_FOR_Text: crate::defs::ITextVirtualTableCcw =
                crate::defs::ITextVirtualTableCcw {
                    offset: 0,
                    vtable: crate::defs::ITextVirtualTable {
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
                crate::defs::IRenderableVirtualTableCcw = crate::defs::IRenderableVirtualTableCcw {
                offset: -1,
                vtable: crate::defs::IRenderableVirtualTable {
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
                        IText: crate::defs::IText {
                            vtable: &GLOBAL_ITextVirtualTable_CCW_FOR_Text.vtable
                                as *const crate::defs::ITextVirtualTable,
                        },

                        IRenderable: crate::defs::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_Text.vtable
                                as *const crate::defs::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }
            }
        }
    };
}

pub(crate) use ComObject_Text;

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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<crate::defs::INode> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).children)(this).into()
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).inner_html)(this).into()
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).outer_html)(this).into()
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).set_inner_html)(this, html.into()).into()
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_elements_by_tag_name)(this, tag.into()).into()
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_element_by_id)(this, id.into()).into()
        }
    }

    pub fn id(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).id)(this).into()
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).tag)(this).into()
        }
    }
}

pub trait IHtmlElementImpl {}

impl crosscom::ComInterface for IHtmlElement {
    // 2be9cc09-3c60-45b9-9084-e4e50ab94ad2
    const INTERFACE_ID: [u8; 16] = [
        43u8, 233u8, 204u8, 9u8, 60u8, 96u8, 69u8, 185u8, 144u8, 132u8, 228u8, 229u8, 10u8, 185u8,
        74u8, 210u8,
    ];
}

// Class HtmlElement

#[allow(unused)]
macro_rules! ComObject_HtmlElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlElement_crosscom_impl {
            use crate::defs::ICharacterDataImpl;
            use crate::defs::IDomStringImpl;
            use crate::defs::IElementImpl;
            use crate::defs::IHtmlBodyElementImpl;
            use crate::defs::IHtmlElementImpl;
            use crate::defs::IHtmlHeadElementImpl;
            use crate::defs::IHtmlHtmlElementImpl;
            use crate::defs::IHtmlParagraphElementImpl;
            use crate::defs::IHtmlScriptElementImpl;
            use crate::defs::INodeImpl;
            use crate::defs::IRenderableImpl;
            use crate::defs::ITextImpl;
            use crosscom::ComInterface;

            #[repr(C)]
            pub struct HtmlElementCcw {
                IHtmlElement: crate::defs::IHtmlElement,
                IRenderable: crate::defs::IRenderable,

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
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    _ => crosscom::ResultCode::ENoInterface as i32,
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

            fn display(this: *const *const std::os::raw::c_void) -> crate::style::Display {
                unsafe {
                    let object = crosscom::get_object::<HtmlElementCcw>(this);
                    (*object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let object = crosscom::get_object::<HtmlElementCcw>(this);
                    (*object)
                        .inner
                        .0
                        .layout(pango_context, style_computed, content_boundary)
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let object = crosscom::get_object::<HtmlElementCcw>(this);
                    (*object).inner.0.paint(renderer, style_computed)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlElementCcw>(this);
                (*object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlElementCcw>(this);
                (*object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlElementCcw>(this);
                (*object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlElementCcw>(this);
                (*object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlElementCcw>(this);
                (*object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let object = crosscom::get_object::<HtmlElementCcw>(this);
                (*object).inner.0.set_inner_html(html.into()).into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlElementCcw>(this);
                (*object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let object = crosscom::get_object::<HtmlElementCcw>(this);
                (*object).inner.0.get_element_by_id(id.into()).into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlElementVirtualTable_CCW_FOR_HtmlElement:
                crate::defs::IHtmlElementVirtualTableCcw =
                crate::defs::IHtmlElementVirtualTableCcw {
                    offset: 0,
                    vtable: crate::defs::IHtmlElementVirtualTable {
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
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlElement:
                crate::defs::IRenderableVirtualTableCcw = crate::defs::IRenderableVirtualTableCcw {
                offset: -1,
                vtable: crate::defs::IRenderableVirtualTable {
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
                        IHtmlElement: crate::defs::IHtmlElement {
                            vtable: &GLOBAL_IHtmlElementVirtualTable_CCW_FOR_HtmlElement.vtable
                                as *const crate::defs::IHtmlElementVirtualTable,
                        },

                        IRenderable: crate::defs::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlElement.vtable
                                as *const crate::defs::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }
            }
        }
    };
}

pub(crate) use ComObject_HtmlElement;

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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<crate::defs::INode> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).children)(this).into()
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).inner_html)(this).into()
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).outer_html)(this).into()
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).set_inner_html)(this, html.into()).into()
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_elements_by_tag_name)(this, tag.into()).into()
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_element_by_id)(this, id.into()).into()
        }
    }

    pub fn id(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).id)(this).into()
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlHtmlElement as *const *const std::os::raw::c_void;
            ((*self.vtable).tag)(this).into()
        }
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
macro_rules! ComObject_HtmlHtmlElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlHtmlElement_crosscom_impl {
            use crate::defs::ICharacterDataImpl;
            use crate::defs::IDomStringImpl;
            use crate::defs::IElementImpl;
            use crate::defs::IHtmlBodyElementImpl;
            use crate::defs::IHtmlElementImpl;
            use crate::defs::IHtmlHeadElementImpl;
            use crate::defs::IHtmlHtmlElementImpl;
            use crate::defs::IHtmlParagraphElementImpl;
            use crate::defs::IHtmlScriptElementImpl;
            use crate::defs::INodeImpl;
            use crate::defs::IRenderableImpl;
            use crate::defs::ITextImpl;
            use crosscom::ComInterface;

            #[repr(C)]
            pub struct HtmlHtmlElementCcw {
                IHtmlHtmlElement: crate::defs::IHtmlHtmlElement,
                IRenderable: crate::defs::IRenderable,

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
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    _ => crosscom::ResultCode::ENoInterface as i32,
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
                    let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                    (*object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                    (*object)
                        .inner
                        .0
                        .layout(pango_context, style_computed, content_boundary)
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                    (*object).inner.0.paint(renderer, style_computed)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*object).inner.0.set_inner_html(html.into()).into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let object = crosscom::get_object::<HtmlHtmlElementCcw>(this);
                (*object).inner.0.get_element_by_id(id.into()).into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlHtmlElementVirtualTable_CCW_FOR_HtmlHtmlElement:
                crate::defs::IHtmlHtmlElementVirtualTableCcw =
                crate::defs::IHtmlHtmlElementVirtualTableCcw {
                    offset: 0,
                    vtable: crate::defs::IHtmlHtmlElementVirtualTable {
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
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlHtmlElement:
                crate::defs::IRenderableVirtualTableCcw = crate::defs::IRenderableVirtualTableCcw {
                offset: -1,
                vtable: crate::defs::IRenderableVirtualTable {
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
                        IHtmlHtmlElement: crate::defs::IHtmlHtmlElement {
                            vtable: &GLOBAL_IHtmlHtmlElementVirtualTable_CCW_FOR_HtmlHtmlElement
                                .vtable
                                as *const crate::defs::IHtmlHtmlElementVirtualTable,
                        },

                        IRenderable: crate::defs::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlHtmlElement.vtable
                                as *const crate::defs::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }
            }
        }
    };
}

pub(crate) use ComObject_HtmlHtmlElement;

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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<crate::defs::INode> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).children)(this).into()
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).inner_html)(this).into()
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).outer_html)(this).into()
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).set_inner_html)(this, html.into()).into()
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_elements_by_tag_name)(this, tag.into()).into()
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_element_by_id)(this, id.into()).into()
        }
    }

    pub fn id(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).id)(this).into()
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).tag)(this).into()
        }
    }

    pub fn text(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlScriptElement as *const *const std::os::raw::c_void;
            ((*self.vtable).text)(this).into()
        }
    }
}

pub trait IHtmlScriptElementImpl {
    fn text(&self) -> crosscom::ComRc<IDomString>;
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
macro_rules! ComObject_HtmlScriptElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlScriptElement_crosscom_impl {
            use crate::defs::ICharacterDataImpl;
            use crate::defs::IDomStringImpl;
            use crate::defs::IElementImpl;
            use crate::defs::IHtmlBodyElementImpl;
            use crate::defs::IHtmlElementImpl;
            use crate::defs::IHtmlHeadElementImpl;
            use crate::defs::IHtmlHtmlElementImpl;
            use crate::defs::IHtmlParagraphElementImpl;
            use crate::defs::IHtmlScriptElementImpl;
            use crate::defs::INodeImpl;
            use crate::defs::IRenderableImpl;
            use crate::defs::ITextImpl;
            use crosscom::ComInterface;

            #[repr(C)]
            pub struct HtmlScriptElementCcw {
                IHtmlScriptElement: crate::defs::IHtmlScriptElement,

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
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlScriptElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    _ => crosscom::ResultCode::ENoInterface as i32,
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
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*object).inner.0.text().into()
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*object).inner.0.set_inner_html(html.into()).into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let object = crosscom::get_object::<HtmlScriptElementCcw>(this);
                (*object).inner.0.get_element_by_id(id.into()).into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlScriptElementVirtualTable_CCW_FOR_HtmlScriptElement:
                crate::defs::IHtmlScriptElementVirtualTableCcw =
                crate::defs::IHtmlScriptElementVirtualTableCcw {
                    offset: 0,
                    vtable: crate::defs::IHtmlScriptElementVirtualTable {
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
                        text,
                    },
                };

            impl crosscom::ComObject for $impl_type {
                type CcwType = HtmlScriptElementCcw;

                fn create_ccw(self) -> Self::CcwType {
                    Self::CcwType {
                        IHtmlScriptElement: crate::defs::IHtmlScriptElement {
                            vtable: &GLOBAL_IHtmlScriptElementVirtualTable_CCW_FOR_HtmlScriptElement
                                .vtable
                                as *const crate::defs::IHtmlScriptElementVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }
            }
        }
    };
}

pub(crate) use ComObject_HtmlScriptElement;

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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<crate::defs::INode> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            ((*self.vtable).children)(this).into()
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            ((*self.vtable).inner_html)(this).into()
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            ((*self.vtable).outer_html)(this).into()
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            ((*self.vtable).set_inner_html)(this, html.into()).into()
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_elements_by_tag_name)(this, tag.into()).into()
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_element_by_id)(this, id.into()).into()
        }
    }

    pub fn id(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            ((*self.vtable).id)(this).into()
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlHeadElement as *const *const std::os::raw::c_void;
            ((*self.vtable).tag)(this).into()
        }
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
macro_rules! ComObject_HtmlHeadElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlHeadElement_crosscom_impl {
            use crate::defs::ICharacterDataImpl;
            use crate::defs::IDomStringImpl;
            use crate::defs::IElementImpl;
            use crate::defs::IHtmlBodyElementImpl;
            use crate::defs::IHtmlElementImpl;
            use crate::defs::IHtmlHeadElementImpl;
            use crate::defs::IHtmlHtmlElementImpl;
            use crate::defs::IHtmlParagraphElementImpl;
            use crate::defs::IHtmlScriptElementImpl;
            use crate::defs::INodeImpl;
            use crate::defs::IRenderableImpl;
            use crate::defs::ITextImpl;
            use crosscom::ComInterface;

            #[repr(C)]
            pub struct HtmlHeadElementCcw {
                IHtmlHeadElement: crate::defs::IHtmlHeadElement,
                IRenderable: crate::defs::IRenderable,

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
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlHeadElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    _ => crosscom::ResultCode::ENoInterface as i32,
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
                    let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                    (*object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                    (*object)
                        .inner
                        .0
                        .layout(pango_context, style_computed, content_boundary)
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                    (*object).inner.0.paint(renderer, style_computed)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*object).inner.0.set_inner_html(html.into()).into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let object = crosscom::get_object::<HtmlHeadElementCcw>(this);
                (*object).inner.0.get_element_by_id(id.into()).into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlHeadElementVirtualTable_CCW_FOR_HtmlHeadElement:
                crate::defs::IHtmlHeadElementVirtualTableCcw =
                crate::defs::IHtmlHeadElementVirtualTableCcw {
                    offset: 0,
                    vtable: crate::defs::IHtmlHeadElementVirtualTable {
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
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlHeadElement:
                crate::defs::IRenderableVirtualTableCcw = crate::defs::IRenderableVirtualTableCcw {
                offset: -1,
                vtable: crate::defs::IRenderableVirtualTable {
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
                        IHtmlHeadElement: crate::defs::IHtmlHeadElement {
                            vtable: &GLOBAL_IHtmlHeadElementVirtualTable_CCW_FOR_HtmlHeadElement
                                .vtable
                                as *const crate::defs::IHtmlHeadElementVirtualTable,
                        },

                        IRenderable: crate::defs::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlHeadElement.vtable
                                as *const crate::defs::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }
            }
        }
    };
}

pub(crate) use ComObject_HtmlHeadElement;

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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<crate::defs::INode> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            ((*self.vtable).children)(this).into()
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            ((*self.vtable).inner_html)(this).into()
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            ((*self.vtable).outer_html)(this).into()
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            ((*self.vtable).set_inner_html)(this, html.into()).into()
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_elements_by_tag_name)(this, tag.into()).into()
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_element_by_id)(this, id.into()).into()
        }
    }

    pub fn id(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            ((*self.vtable).id)(this).into()
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlBodyElement as *const *const std::os::raw::c_void;
            ((*self.vtable).tag)(this).into()
        }
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
macro_rules! ComObject_HtmlBodyElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlBodyElement_crosscom_impl {
            use crate::defs::ICharacterDataImpl;
            use crate::defs::IDomStringImpl;
            use crate::defs::IElementImpl;
            use crate::defs::IHtmlBodyElementImpl;
            use crate::defs::IHtmlElementImpl;
            use crate::defs::IHtmlHeadElementImpl;
            use crate::defs::IHtmlHtmlElementImpl;
            use crate::defs::IHtmlParagraphElementImpl;
            use crate::defs::IHtmlScriptElementImpl;
            use crate::defs::INodeImpl;
            use crate::defs::IRenderableImpl;
            use crate::defs::ITextImpl;
            use crosscom::ComInterface;

            #[repr(C)]
            pub struct HtmlBodyElementCcw {
                IHtmlBodyElement: crate::defs::IHtmlBodyElement,
                IRenderable: crate::defs::IRenderable,

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
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlBodyElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    _ => crosscom::ResultCode::ENoInterface as i32,
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
                    let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                    (*object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                    (*object)
                        .inner
                        .0
                        .layout(pango_context, style_computed, content_boundary)
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                    (*object).inner.0.paint(renderer, style_computed)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*object).inner.0.set_inner_html(html.into()).into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let object = crosscom::get_object::<HtmlBodyElementCcw>(this);
                (*object).inner.0.get_element_by_id(id.into()).into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlBodyElementVirtualTable_CCW_FOR_HtmlBodyElement:
                crate::defs::IHtmlBodyElementVirtualTableCcw =
                crate::defs::IHtmlBodyElementVirtualTableCcw {
                    offset: 0,
                    vtable: crate::defs::IHtmlBodyElementVirtualTable {
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
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlBodyElement:
                crate::defs::IRenderableVirtualTableCcw = crate::defs::IRenderableVirtualTableCcw {
                offset: -1,
                vtable: crate::defs::IRenderableVirtualTable {
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
                        IHtmlBodyElement: crate::defs::IHtmlBodyElement {
                            vtable: &GLOBAL_IHtmlBodyElementVirtualTable_CCW_FOR_HtmlBodyElement
                                .vtable
                                as *const crate::defs::IHtmlBodyElementVirtualTable,
                        },

                        IRenderable: crate::defs::IRenderable {
                            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlBodyElement.vtable
                                as *const crate::defs::IRenderableVirtualTable,
                        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }
            }
        }
    };
}

pub(crate) use ComObject_HtmlBodyElement;

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

    pub fn add_ref(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            ((*self.vtable).add_ref)(this).into()
        }
    }

    pub fn release(&self) -> i32 {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            ((*self.vtable).release)(this).into()
        }
    }

    pub fn children(&self) -> crosscom::ObjectArray<crate::defs::INode> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            ((*self.vtable).children)(this).into()
        }
    }

    pub fn inner_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            ((*self.vtable).inner_html)(this).into()
        }
    }

    pub fn outer_html(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            ((*self.vtable).outer_html)(this).into()
        }
    }

    pub fn set_inner_html(&self, html: crosscom::ComRc<IDomString>) -> () {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            ((*self.vtable).set_inner_html)(this, html.into()).into()
        }
    }

    pub fn get_elements_by_tag_name(
        &self,
        tag: crosscom::ComRc<IDomString>,
    ) -> crosscom::ObjectArray<crate::defs::IElement> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_elements_by_tag_name)(this, tag.into()).into()
        }
    }

    pub fn get_element_by_id(
        &self,
        id: crosscom::ComRc<IDomString>,
    ) -> Option<crosscom::ComRc<crate::defs::IElement>> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            ((*self.vtable).get_element_by_id)(this, id.into()).into()
        }
    }

    pub fn id(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            ((*self.vtable).id)(this).into()
        }
    }

    pub fn tag(&self) -> crosscom::ComRc<IDomString> {
        unsafe {
            let this = self as *const IHtmlParagraphElement as *const *const std::os::raw::c_void;
            ((*self.vtable).tag)(this).into()
        }
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
macro_rules! ComObject_HtmlParagraphElement {
    ($impl_type: ty) => {
        #[allow(dead_code)]
        #[allow(non_snake_case)]
        #[allow(unused)]
        mod HtmlParagraphElement_crosscom_impl {
            use crate::defs::ICharacterDataImpl;
            use crate::defs::IDomStringImpl;
            use crate::defs::IElementImpl;
            use crate::defs::IHtmlBodyElementImpl;
            use crate::defs::IHtmlElementImpl;
            use crate::defs::IHtmlHeadElementImpl;
            use crate::defs::IHtmlHtmlElementImpl;
            use crate::defs::IHtmlParagraphElementImpl;
            use crate::defs::IHtmlScriptElementImpl;
            use crate::defs::INodeImpl;
            use crate::defs::IRenderableImpl;
            use crate::defs::ITextImpl;
            use crosscom::ComInterface;

            #[repr(C)]
            pub struct HtmlParagraphElementCcw {
                IHtmlParagraphElement: crate::defs::IHtmlParagraphElement,
                IRenderable: crate::defs::IRenderable,

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
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::INode::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IHtmlParagraphElement::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(0);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    &crate::defs::IRenderable::INTERFACE_ID => {
                        *retval = (object as *const *const std::os::raw::c_void).offset(1);
                        add_ref(object as *const *const std::os::raw::c_void);
                        crosscom::ResultCode::Ok as i32
                    }

                    _ => crosscom::ResultCode::ENoInterface as i32,
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
                    let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                    (*object).inner.0.display()
                }
            }

            fn layout(
                this: *const *const std::os::raw::c_void,
                pango_context: &pango::Context,
                style_computed: &crate::style::Style,
                content_boundary: crate::common::Rectangle,
            ) -> crate::common::Rectangle {
                unsafe {
                    let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                    (*object)
                        .inner
                        .0
                        .layout(pango_context, style_computed, content_boundary)
                }
            }

            fn paint(
                this: *const *const std::os::raw::c_void,
                renderer: &crate::rendering::cairo::CairoRenderer,
                style_computed: &crate::style::Style,
            ) -> crosscom::Void {
                unsafe {
                    let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                    (*object).inner.0.paint(renderer, style_computed)
                }
            }

            unsafe extern "system" fn id(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*object).inner.0.id().into()
            }

            unsafe extern "system" fn tag(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*object).inner.0.tag().into()
            }

            unsafe extern "system" fn children(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*object).inner.0.children().into()
            }

            unsafe extern "system" fn inner_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*object).inner.0.inner_html().into()
            }

            unsafe extern "system" fn outer_html(
                this: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*object).inner.0.outer_html().into()
            }

            unsafe extern "system" fn set_inner_html(
                this: *const *const std::os::raw::c_void,
                html: *const *const std::os::raw::c_void,
            ) -> () {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*object).inner.0.set_inner_html(html.into()).into()
            }

            unsafe extern "system" fn get_elements_by_tag_name(
                this: *const *const std::os::raw::c_void,
                tag: *const *const std::os::raw::c_void,
            ) -> *const *const std::os::raw::c_void {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*object)
                    .inner
                    .0
                    .get_elements_by_tag_name(tag.into())
                    .into()
            }

            unsafe extern "system" fn get_element_by_id(
                this: *const *const std::os::raw::c_void,
                id: *const *const std::os::raw::c_void,
            ) -> crosscom::RawPointer {
                let object = crosscom::get_object::<HtmlParagraphElementCcw>(this);
                (*object).inner.0.get_element_by_id(id.into()).into()
            }

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IHtmlParagraphElementVirtualTable_CCW_FOR_HtmlParagraphElement:
                crate::defs::IHtmlParagraphElementVirtualTableCcw =
                crate::defs::IHtmlParagraphElementVirtualTableCcw {
                    offset: 0,
                    vtable: crate::defs::IHtmlParagraphElementVirtualTable {
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
                    },
                };

            #[allow(non_upper_case_globals)]
            pub const GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlParagraphElement:
                crate::defs::IRenderableVirtualTableCcw = crate::defs::IRenderableVirtualTableCcw {
                offset: -1,
                vtable: crate::defs::IRenderableVirtualTable {
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

        IHtmlParagraphElement: crate::defs::IHtmlParagraphElement {
            vtable: &GLOBAL_IHtmlParagraphElementVirtualTable_CCW_FOR_HtmlParagraphElement.vtable
                as *const crate::defs::IHtmlParagraphElementVirtualTable,
        },

        IRenderable: crate::defs::IRenderable {
            vtable: &GLOBAL_IRenderableVirtualTable_CCW_FOR_HtmlParagraphElement.vtable
                as *const crate::defs::IRenderableVirtualTable,
        },

                        ref_count: std::sync::atomic::AtomicU32::new(0),
                        inner: self,
                    }
                }
            }
        }
    };
}

pub(crate) use ComObject_HtmlParagraphElement;
