// Interface INode

#[repr(C)]
#[allow(non_snake_case)]
pub struct INodeVirtualTable {

    pub query_interface: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, guid: uuid::Uuid, 
retval: &mut *const *const std::os::raw::c_void, 
) -> std::os::raw::c_long
,    pub add_ref: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> std::os::raw::c_long
,    pub release: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> std::os::raw::c_long
,    pub children: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> *const *const std::os::raw::c_void
,}


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


pub fn add_ref (&self, ) -> i32
 {
    unsafe {
        let this = self as *const INode as *const *const std::os::raw::c_void;
        ((*self.vtable).add_ref)(this, ).into()
    }
}


pub fn release (&self, ) -> i32
 {
    unsafe {
        let this = self as *const INode as *const *const std::os::raw::c_void;
        ((*self.vtable).release)(this, ).into()
    }
}


pub fn children (&self, ) -> crosscom::ObjectArray<crate::dom::defs::INode>
 {
    unsafe {
        let this = self as *const INode as *const *const std::os::raw::c_void;
        ((*self.vtable).children)(this, ).into()
    }
}


}

pub trait INodeImpl {
fn children (&self, ) -> crosscom::ObjectArray<crate::dom::defs::INode>
;
}

impl crosscom::ComInterface for INode {
            
    // 518d7182-9244-448e-a439-624115b0be12
    const INTERFACE_ID: [u8; 16] = [81u8,141u8,113u8,130u8,146u8,68u8,68u8,142u8,164u8,57u8,98u8,65u8,21u8,176u8,190u8,18u8];
}


// Class Node

#[allow(unused)]
macro_rules! ComObject_Node {
    ($impl_type: ty) => {

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
mod Node_crosscom_impl {
    use crosscom::ComInterface;
use crate::dom::defs::INodeImpl;
use crate::dom::defs::IElementImpl;
use crate::dom::defs::ICharacterDataImpl;
use crate::dom::defs::ITextImpl;


    #[repr(C)]
    pub struct NodeCcw {
        INode: crate::dom::defs::INode,

        ref_count: std::sync::atomic::AtomicU32,
        inner: $impl_type,
    }

    unsafe extern "system" fn query_interface(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long {
        let object = crosscom::get_object::<NodeCcw>(this);
        match guid.as_bytes() {
            
&crosscom::IUnknown::INTERFACE_ID => {
    *retval = (object as *const *const std::os::raw::c_void).offset(0);
    add_ref(object as *const *const std::os::raw::c_void);
    crosscom::ResultCode::Ok as i32
}


&crate::dom::defs::INode::INTERFACE_ID => {
    *retval = (object as *const *const std::os::raw::c_void).offset(0);
    add_ref(object as *const *const std::os::raw::c_void);
    crosscom::ResultCode::Ok as i32
}


            _ => crosscom::ResultCode::ENoInterface as i32,
        }
    }

    unsafe extern "system" fn add_ref(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long {
        let object = crosscom::get_object::<NodeCcw>(this);
        let previous = (*object).ref_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        (previous + 1) as std::os::raw::c_long
    }

    unsafe extern "system" fn release(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long {
        let object = crosscom::get_object::<NodeCcw>(this);

        let previous = (*object).ref_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        if previous - 1 == 0 {
            Box::from_raw(object as *mut NodeCcw);
        }

        (previous - 1) as std::os::raw::c_long
    }


    
unsafe extern "system" fn children (this: *const *const std::os::raw::c_void, ) -> *const *const std::os::raw::c_void {
    let object = crosscom::get_object::<NodeCcw>(this);
    (*object).inner.0.children().into()
}





    
#[allow(non_upper_case_globals)]
pub const GLOBAL_INodeVirtualTable_CCW_FOR_Node: crate::dom::defs::INodeVirtualTableCcw 
    = crate::dom::defs::INodeVirtualTableCcw {
    offset: 0,
    vtable: crate::dom::defs::INodeVirtualTable {
        query_interface,
add_ref,
release,
children,

    },
};




    impl crosscom::ComObject for $impl_type {
        type CcwType = NodeCcw;

        fn create_ccw(self) -> Self::CcwType {
            Self::CcwType {
                
INode: crate::dom::defs::INode {
    vtable: &GLOBAL_INodeVirtualTable_CCW_FOR_Node.vtable
        as *const crate::dom::defs::INodeVirtualTable,
},

                ref_count: std::sync::atomic::AtomicU32::new(0),
                inner: self,
            }
        }
    }
}
    }
}

pub(crate) use ComObject_Node;

// Interface IElement

#[repr(C)]
#[allow(non_snake_case)]
pub struct IElementVirtualTable {

    pub query_interface: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, guid: uuid::Uuid, 
retval: &mut *const *const std::os::raw::c_void, 
) -> std::os::raw::c_long
,    pub add_ref: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> std::os::raw::c_long
,    pub release: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> std::os::raw::c_long
,    pub children: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> *const *const std::os::raw::c_void
,    pub id: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> ()
,}


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


pub fn add_ref (&self, ) -> i32
 {
    unsafe {
        let this = self as *const IElement as *const *const std::os::raw::c_void;
        ((*self.vtable).add_ref)(this, ).into()
    }
}


pub fn release (&self, ) -> i32
 {
    unsafe {
        let this = self as *const IElement as *const *const std::os::raw::c_void;
        ((*self.vtable).release)(this, ).into()
    }
}


pub fn children (&self, ) -> crosscom::ObjectArray<crate::dom::defs::INode>
 {
    unsafe {
        let this = self as *const IElement as *const *const std::os::raw::c_void;
        ((*self.vtable).children)(this, ).into()
    }
}


pub fn id (&self, ) -> ()
 {
    unsafe {
        let this = self as *const IElement as *const *const std::os::raw::c_void;
        ((*self.vtable).id)(this, ).into()
    }
}


}

pub trait IElementImpl {
fn id (&self, ) -> ()
;
}

impl crosscom::ComInterface for IElement {
            
    // c8c9b586-0569-4af4-b619-6491c61dc94a
    const INTERFACE_ID: [u8; 16] = [200u8,201u8,181u8,134u8,5u8,105u8,74u8,244u8,182u8,25u8,100u8,145u8,198u8,29u8,201u8,74u8];
}


// Class Element

#[allow(unused)]
macro_rules! ComObject_Element {
    ($impl_type: ty) => {

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
mod Element_crosscom_impl {
    use crosscom::ComInterface;
use crate::dom::defs::INodeImpl;
use crate::dom::defs::IElementImpl;
use crate::dom::defs::ICharacterDataImpl;
use crate::dom::defs::ITextImpl;


    #[repr(C)]
    pub struct ElementCcw {
        IElement: crate::dom::defs::IElement,

        ref_count: std::sync::atomic::AtomicU32,
        inner: $impl_type,
    }

    unsafe extern "system" fn query_interface(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long {
        let object = crosscom::get_object::<ElementCcw>(this);
        match guid.as_bytes() {
            
&crosscom::IUnknown::INTERFACE_ID => {
    *retval = (object as *const *const std::os::raw::c_void).offset(0);
    add_ref(object as *const *const std::os::raw::c_void);
    crosscom::ResultCode::Ok as i32
}


&crate::dom::defs::INode::INTERFACE_ID => {
    *retval = (object as *const *const std::os::raw::c_void).offset(0);
    add_ref(object as *const *const std::os::raw::c_void);
    crosscom::ResultCode::Ok as i32
}


&crate::dom::defs::IElement::INTERFACE_ID => {
    *retval = (object as *const *const std::os::raw::c_void).offset(0);
    add_ref(object as *const *const std::os::raw::c_void);
    crosscom::ResultCode::Ok as i32
}


            _ => crosscom::ResultCode::ENoInterface as i32,
        }
    }

    unsafe extern "system" fn add_ref(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long {
        let object = crosscom::get_object::<ElementCcw>(this);
        let previous = (*object).ref_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        (previous + 1) as std::os::raw::c_long
    }

    unsafe extern "system" fn release(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long {
        let object = crosscom::get_object::<ElementCcw>(this);

        let previous = (*object).ref_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        if previous - 1 == 0 {
            Box::from_raw(object as *mut ElementCcw);
        }

        (previous - 1) as std::os::raw::c_long
    }


    
unsafe extern "system" fn id (this: *const *const std::os::raw::c_void, ) -> () {
    let object = crosscom::get_object::<ElementCcw>(this);
    (*object).inner.0.id().into()
}



unsafe extern "system" fn children (this: *const *const std::os::raw::c_void, ) -> *const *const std::os::raw::c_void {
    let object = crosscom::get_object::<ElementCcw>(this);
    (*object).inner.0.children().into()
}





    
#[allow(non_upper_case_globals)]
pub const GLOBAL_IElementVirtualTable_CCW_FOR_Element: crate::dom::defs::IElementVirtualTableCcw 
    = crate::dom::defs::IElementVirtualTableCcw {
    offset: 0,
    vtable: crate::dom::defs::IElementVirtualTable {
        query_interface,
add_ref,
release,
children,
id,

    },
};




    impl crosscom::ComObject for $impl_type {
        type CcwType = ElementCcw;

        fn create_ccw(self) -> Self::CcwType {
            Self::CcwType {
                
IElement: crate::dom::defs::IElement {
    vtable: &GLOBAL_IElementVirtualTable_CCW_FOR_Element.vtable
        as *const crate::dom::defs::IElementVirtualTable,
},

                ref_count: std::sync::atomic::AtomicU32::new(0),
                inner: self,
            }
        }
    }
}
    }
}

pub(crate) use ComObject_Element;

// Interface ICharacterData

#[repr(C)]
#[allow(non_snake_case)]
pub struct ICharacterDataVirtualTable {

    pub query_interface: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, guid: uuid::Uuid, 
retval: &mut *const *const std::os::raw::c_void, 
) -> std::os::raw::c_long
,    pub add_ref: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> std::os::raw::c_long
,    pub release: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> std::os::raw::c_long
,    pub children: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> *const *const std::os::raw::c_void
,    pub text: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> ()
,}


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


pub fn add_ref (&self, ) -> i32
 {
    unsafe {
        let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
        ((*self.vtable).add_ref)(this, ).into()
    }
}


pub fn release (&self, ) -> i32
 {
    unsafe {
        let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
        ((*self.vtable).release)(this, ).into()
    }
}


pub fn children (&self, ) -> crosscom::ObjectArray<crate::dom::defs::INode>
 {
    unsafe {
        let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
        ((*self.vtable).children)(this, ).into()
    }
}


pub fn text (&self, ) -> ()
 {
    unsafe {
        let this = self as *const ICharacterData as *const *const std::os::raw::c_void;
        ((*self.vtable).text)(this, ).into()
    }
}


}

pub trait ICharacterDataImpl {
fn text (&self, ) -> ()
;
}

impl crosscom::ComInterface for ICharacterData {
            
    // a8b5f552-0f22-4c4d-930c-432312b16a6c
    const INTERFACE_ID: [u8; 16] = [168u8,181u8,245u8,82u8,15u8,34u8,76u8,77u8,147u8,12u8,67u8,35u8,18u8,177u8,106u8,108u8];
}

// Interface IText

#[repr(C)]
#[allow(non_snake_case)]
pub struct ITextVirtualTable {

    pub query_interface: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, guid: uuid::Uuid, 
retval: &mut *const *const std::os::raw::c_void, 
) -> std::os::raw::c_long
,    pub add_ref: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> std::os::raw::c_long
,    pub release: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> std::os::raw::c_long
,    pub children: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> *const *const std::os::raw::c_void
,    pub text: unsafe extern "system" fn (this: *const *const std::os::raw::c_void, ) -> ()
,}


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


pub fn add_ref (&self, ) -> i32
 {
    unsafe {
        let this = self as *const IText as *const *const std::os::raw::c_void;
        ((*self.vtable).add_ref)(this, ).into()
    }
}


pub fn release (&self, ) -> i32
 {
    unsafe {
        let this = self as *const IText as *const *const std::os::raw::c_void;
        ((*self.vtable).release)(this, ).into()
    }
}


pub fn children (&self, ) -> crosscom::ObjectArray<crate::dom::defs::INode>
 {
    unsafe {
        let this = self as *const IText as *const *const std::os::raw::c_void;
        ((*self.vtable).children)(this, ).into()
    }
}


pub fn text (&self, ) -> ()
 {
    unsafe {
        let this = self as *const IText as *const *const std::os::raw::c_void;
        ((*self.vtable).text)(this, ).into()
    }
}


}

pub trait ITextImpl {
}

impl crosscom::ComInterface for IText {
            
    // 2d54063c-b56b-44ec-8930-2ca618be2ecf
    const INTERFACE_ID: [u8; 16] = [45u8,84u8,6u8,60u8,181u8,107u8,68u8,236u8,137u8,48u8,44u8,166u8,24u8,190u8,46u8,207u8];
}


// Class Text

#[allow(unused)]
macro_rules! ComObject_Text {
    ($impl_type: ty) => {

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
mod Text_crosscom_impl {
    use crosscom::ComInterface;
use crate::dom::defs::INodeImpl;
use crate::dom::defs::IElementImpl;
use crate::dom::defs::ICharacterDataImpl;
use crate::dom::defs::ITextImpl;


    #[repr(C)]
    pub struct TextCcw {
        IText: crate::dom::defs::IText,

        ref_count: std::sync::atomic::AtomicU32,
        inner: $impl_type,
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


&crate::dom::defs::INode::INTERFACE_ID => {
    *retval = (object as *const *const std::os::raw::c_void).offset(0);
    add_ref(object as *const *const std::os::raw::c_void);
    crosscom::ResultCode::Ok as i32
}


&crate::dom::defs::ICharacterData::INTERFACE_ID => {
    *retval = (object as *const *const std::os::raw::c_void).offset(0);
    add_ref(object as *const *const std::os::raw::c_void);
    crosscom::ResultCode::Ok as i32
}


&crate::dom::defs::IText::INTERFACE_ID => {
    *retval = (object as *const *const std::os::raw::c_void).offset(0);
    add_ref(object as *const *const std::os::raw::c_void);
    crosscom::ResultCode::Ok as i32
}


            _ => crosscom::ResultCode::ENoInterface as i32,
        }
    }

    unsafe extern "system" fn add_ref(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long {
        let object = crosscom::get_object::<TextCcw>(this);
        let previous = (*object).ref_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        (previous + 1) as std::os::raw::c_long
    }

    unsafe extern "system" fn release(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long {
        let object = crosscom::get_object::<TextCcw>(this);

        let previous = (*object).ref_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        if previous - 1 == 0 {
            Box::from_raw(object as *mut TextCcw);
        }

        (previous - 1) as std::os::raw::c_long
    }


    
unsafe extern "system" fn text (this: *const *const std::os::raw::c_void, ) -> () {
    let object = crosscom::get_object::<TextCcw>(this);
    (*object).inner.0.text().into()
}



unsafe extern "system" fn children (this: *const *const std::os::raw::c_void, ) -> *const *const std::os::raw::c_void {
    let object = crosscom::get_object::<TextCcw>(this);
    (*object).inner.0.children().into()
}





    
#[allow(non_upper_case_globals)]
pub const GLOBAL_ITextVirtualTable_CCW_FOR_Text: crate::dom::defs::ITextVirtualTableCcw 
    = crate::dom::defs::ITextVirtualTableCcw {
    offset: 0,
    vtable: crate::dom::defs::ITextVirtualTable {
        query_interface,
add_ref,
release,
children,
text,

    },
};




    impl crosscom::ComObject for $impl_type {
        type CcwType = TextCcw;

        fn create_ccw(self) -> Self::CcwType {
            Self::CcwType {
                
IText: crate::dom::defs::IText {
    vtable: &GLOBAL_ITextVirtualTable_CCW_FOR_Text.vtable
        as *const crate::dom::defs::ITextVirtualTable,
},

                ref_count: std::sync::atomic::AtomicU32::new(0),
                inner: self,
            }
        }
    }
}
    }
}

pub(crate) use ComObject_Text;

