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
    (*object).inner.children().into()
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

