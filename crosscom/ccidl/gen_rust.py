from audioop import cross
from email import parser
import io
import uuid
from parser import Interface, Class, CrossComIdl, Method, MethodParameter, Module, parse


class Writer:
    def __init__(self):
        self.writer = io.StringIO()

    def ln(self, text="", ident=0):
        prefix = "    " * ident
        return self.writer.write(prefix + text + '\n')

    def w(self, text):
        return self.writer.write(text)

    def get_value(self):
        return self.writer.getvalue()


type_map = {
    'long': ('std::os::raw::c_long', 'std::os::raw::c_long'),
    'longlong': ('std::os::raw::c_longlong', 'std::os::raw::c_longlong'),
    'int': ('std::os::raw::c_int', 'std::os::raw::c_int'),
    'float': ('std::os::raw::c_float', 'f32'),
    'byte': ('std::os::raw::c_uchar', 'std::os::raw::c_uchar'),
    'byte*': ('*const std::os::raw::c_uchar', '*const std::os::raw::c_uchar'),
    'UUID': ('uuid::Uuid', 'uuid::Uuid'),
    'bool': ('std::os::raw::c_int', 'bool', ' != 0'),
    'void': ('()', '()'),
}


class RustGen:

    def __init__(self, unit: CrossComIdl, idl_file_path: str, crosscom_module_name: str = "crosscom"):
        self.unit = unit
        self.crosscom_module_name = crosscom_module_name

        self.process_imports(idl_file_path, unit)
        self.symbols = self.__collect_symbols(unit)

    def get_rust_module(self, unit):
        return next(filter(lambda m: m.module_lang == "rust", unit.modules))

    def get_rust_crate(self):
        return self.get_rust_module(self.unit).module_name.split("::")[0]

    def process_imports(self, idl_file_path: str, unit: CrossComIdl):
        import os
        source_dir = os.path.dirname(idl_file_path)

        for imp in unit.imports:
            imp_file_path = os.path.join(source_dir, imp.file_name)
            imp_content = open(imp_file_path, encoding="utf-8").read()
            imp_unit = parse(imp_content)
            imp_module = self.get_rust_module(imp_unit)

            # self.process_imports(imp_file_path, imp_unit)

            for imp_item in imp_unit.items:
                if isinstance(imp_item, Interface):

                    if imp_item.attrs is None:
                        imp_item.attrs = {}

                    imp_item.attrs["codegen"] = "ignore"
                    imp_item.module = imp_module
                    self.unit.items.append(imp_item)


    def gen(self) -> str:

        w = Writer()
        w.ln(f"use crate as {self.get_rust_crate()};")

        for i in self.unit.items:
            if isinstance(i, Class):
                w.w(self.__gen_class(i))
            else:
                self.__gen_interface(i, w)

        return w.get_value()

    def __gen_method_raw_param_list(self, method: Method):
        w = Writer()

        for p in method.params:
            if method.attrs is not None and 'internal' in method.attrs:
                w.ln(f'{p.name}: {self.__map_rust_internal_type(p.ty, method.interface.module)}, ')
            else:
                w.ln(f'{p.name}: {self.__map_raw_type(p.ty, p.attrs)}, ')

        return w.get_value()

    def __gen_method_param_mapping(self, method: Method):
        w = Writer()

        for p in method.params:
            if method.attrs is not None and 'internal' in method.attrs:
                w.ln(f'{p.name}: {self.__map_rust_internal_type(p.ty, method.interface.module)}, ')
            else:
                w.ln(f'let {p.name}: {self.__map_type(p.ty, True)} = {self.__gen_param_ty_convert(p)};')

        return w.get_value()

    def __gen_method_ret_mapping(self, method: Method):
        w = Writer()

        if method.attrs is not None and 'internal' in method.attrs:
            w.ln(f'');
        else:
            w.ln(f'let ret: {self.__map_type(method.ret_ty, False)} = {self.__gen_ret_ty_convert(method)};')

        return w.get_value()

    def __gen_param_ty_convert(self, p: MethodParameter):
        w = Writer()
        if p.ty in type_map and len(type_map[p.ty]) > 2:
            w.ln(f'{p.name}{type_map[p.ty][2]}')
        else:
            w.ln(f'{p.name}.into()')

        return w.get_value()

    def __gen_ret_ty_convert(self, p: Method):
        w = Writer()
        if p.ret_ty in type_map and len(type_map[p.ret_ty]) > 2:
            w.ln(f'ret{type_map[p.ret_ty][2]}')
        else:
            w.ln(f'ret.into()')

        return w.get_value()

    def __gen_method_raw_signature(self, method: Method, w: Writer):
        if method.attrs is not None and 'internal' in method.attrs:
            w.ln(
                f'fn (this: *const *const std::os::raw::c_void, {self.__gen_method_raw_param_list(method)}) -> {method.ret_ty}')
        else:
            w.ln(
                f'unsafe extern "system" fn (this: *const *const std::os::raw::c_void, {self.__gen_method_raw_param_list(method)}) -> {self.__map_raw_type(method.ret_ty)}')

    def __gen_method_signature2(self, method: Method) -> str:
        w = Writer()
        w.w(f'fn {method.name} (&self, ')

        for p in method.params:
            if method.attrs is not None and 'internal' in method.attrs:
                w.ln(f'{p.name}: {p.ty}, ')
            else:
                w.ln(f'{p.name}: {self.__map_type(p.ty, True)}, ')

        if method.attrs is not None and 'internal' in method.attrs:
            w.ln(f') -> {method.ret_ty}')
        else:
            w.ln(f') -> {self.__map_type(method.ret_ty, True)}')

        return w.get_value()

    def __gen_trait_use(self) -> str:
        w = Writer()
        w.ln(f'use {self.crosscom_module_name}::ComInterface;')
        for item in self.unit.items:
            if isinstance(item, Interface):
                w.ln(f'use {item.module.module_name}::{item.name}Impl;')
        return w.get_value()

    def __gen_klass_base_field(self, klass: Class) -> str:
        w = Writer()
        for b in klass.bases:
            symbol = self.symbols[b]
            w.ln(f'{b}: {symbol.module.module_name}::{b},')
        return w.get_value()

    def __gen_raw_method_impl(self, klass: Class, method: Method) -> str:
        w = Writer()

        field_name = "" if 'rust_inner_field' not in klass.attrs else f".{klass.attrs['rust_inner_field']}"

        if method.attrs is not None and 'internal' in method.attrs:
            w.ln(f"""
    fn {method.name} (this: *const *const std::os::raw::c_void, {self.__gen_method_raw_param_list(method)}) -> {method.ret_ty} {{
        unsafe {{
            let __crosscom_object = {self.crosscom_module_name}::get_object::<{klass.name}Ccw>(this);
            (*__crosscom_object).inner{field_name}.{ method.name }({','.join([p.name for p in method.params])})
        }}
    }}
    """)
        else:
            w.ln(f"""
    unsafe extern "system" fn {method.name} (this: *const *const std::os::raw::c_void, {self.__gen_method_raw_param_list(method)}) -> {self.__map_raw_type(method.ret_ty)} {{
        {self.__gen_method_param_mapping(method)}
        let __crosscom_object = {self.crosscom_module_name}::get_object::<{klass.name}Ccw>(this);
        (*__crosscom_object).inner{field_name}.{ method.name }({','.join([f'{p.name}.into()' for p in method.params])}).into()
    }}
    """)

        return w.get_value()

    def __gen_raw_method_impl_for_class(self, klass: Class) -> str:
        w = Writer()

        for m in klass.methods:
            w.ln(self.__gen_raw_method_impl(klass, m))

        visited = set()
        ancestors = [b for b in klass.bases]
        while len(ancestors) > 0:
            a = ancestors.pop(0)
            if a in visited:
                continue

            visited.add(a)

            interface = self.unit.find(a)
            if interface is None:
                raise f'Cannot find base type: {a}'

            if isinstance(interface, Class):
                raise f'Class type cannot be used as base: {a}'

            if interface.name == "IUnknown":
                continue

            if interface.bases is not None:
                ancestors.extend(interface.bases)

            for m in interface.methods:
                w.ln(self.__gen_raw_method_impl(klass, m))

        return w.get_value()

    def __collect_all_methods2(self, iname: str, only_public: bool = False) -> list[Method]:
        interface = self.unit.find(iname)
        if interface is None:
            raise f'Cannot find base type: {iname}'

        if isinstance(interface, Class):
            raise f'Class type cannot be used as base: {iname}'

        methods = []
        if interface.bases is not None:
            if len(interface.bases) == 1:
                methods = self.__collect_all_methods2(
                    interface.bases[0], only_public)
            elif len(interface.bases) > 1:
                raise f'Cannot have more than 1 parent for interface: {interface.name}'

        if not only_public:
            methods.extend(interface.methods)
        else:
            methods.extend(interface.public_methods())

        return methods

    def __collect_inherit_chain(self, iname: str) -> list[Method]:
        interface = self.unit.find(iname)
        if interface is None:
            raise Exception(f'Cannot find base type: {iname}')

        if isinstance(interface, Class):
            raise Exception(f'Class type cannot be used as base: {iname}')

        ifaces = []
        if interface.bases is not None:
            if len(interface.bases) == 1:
                ifaces = self.__collect_inherit_chain(interface.bases[0])
            elif len(interface.bases) > 1:
                raise Exception(f'Cannot have more than 1 parent for interface: {interface.name}')

        ifaces.append(interface)

        return ifaces

    def __gen_interface_vtbl_methods(self, iname: str) -> str:
        w = Writer()

        methods = self.__collect_all_methods2(iname)
        for m in methods:
            w.ln(m.name + ',')

        return w.get_value()

    def __gen_base_struct(self, klass: Class) -> str:
        w = Writer()
        for b in klass.bases:
            symbol = self.symbols[b]
            w.ln(f"""
{b}: {symbol.module.module_name}::{b} {{
    vtable: &GLOBAL_{b}VirtualTable_CCW_FOR_{klass.name}.vtable
        as *const {symbol.module.module_name}::{b}VirtualTable,
}},""")

        return w.get_value()

    def __gen_class_ccw_vtbl(self, klass: Class) -> str:
        w = Writer()
        offset = 1
        for b in klass.bases:
            offset -= 1
            symbol = self.symbols[b]
            w.ln(f"""
#[allow(non_upper_case_globals)]
pub const GLOBAL_{b}VirtualTable_CCW_FOR_{ klass.name }: {symbol.module.module_name}::{b}VirtualTableCcw 
    = {symbol.module.module_name}::{b}VirtualTableCcw {{
    offset: {offset},
    vtable: {symbol.module.module_name}::{b}VirtualTable {{
        {self.__gen_interface_vtbl_methods(b)}
    }},
}};

""")
        return w.get_value()

    def __gen_query_interface_branches(self, klass: Class) -> str:
        w = Writer()

        visited = set()
        offset = -1
        for i in klass.bases:
            offset += 1
            ifaces = self.__collect_inherit_chain(i)
            for interface in ifaces:
                if interface.name in visited:
                    continue

                visited.add(interface.name)
                mod = interface.module.module_name
                w.ln(f"""
&{mod}::{interface.name}::INTERFACE_ID => {{
    *retval = (object as *const *const std::os::raw::c_void).offset({offset});
    add_ref(object as *const *const std::os::raw::c_void);
    {self.crosscom_module_name}::ResultCode::Ok as std::os::raw::c_long
}}
""")

        return w.get_value()

    def __gen_class(self, klass: Class) -> str:
        w = Writer()
        w.ln(f"""
// Class {klass.name}

#[allow(unused)]
#[macro_export]
macro_rules! ComObject_{klass.name} {{
    ($impl_type: ty) => {{

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
mod {klass.name}_crosscom_impl {{
    use crate as {self.get_rust_crate()};
    {self.__gen_trait_use()}

    #[repr(C)]
    pub struct { klass.name }Ccw {{
        {self.__gen_klass_base_field(klass)}
        ref_count: std::sync::atomic::AtomicU32,
        pub inner: $impl_type,
    }}

    unsafe extern "system" fn query_interface(
        this: *const *const std::os::raw::c_void,
        guid: uuid::Uuid,
        retval: &mut *const *const std::os::raw::c_void,
    ) -> std::os::raw::c_long {{
        let object = {self.crosscom_module_name}::get_object::<{klass.name}Ccw>(this);
        match guid.as_bytes() {{
            {self.__gen_query_interface_branches(klass)}
            _ => {self.crosscom_module_name}::ResultCode::ENoInterface as std::os::raw::c_long,
        }}
    }}

    unsafe extern "system" fn add_ref(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long {{
        let object = {self.crosscom_module_name}::get_object::<{klass.name}Ccw>(this);
        let previous = (*object).ref_count.fetch_add(1, std::sync::atomic::Ordering::SeqCst);
        (previous + 1) as std::os::raw::c_long
    }}

    unsafe extern "system" fn release(this: *const *const std::os::raw::c_void) -> std::os::raw::c_long {{
        let object = {self.crosscom_module_name}::get_object::<{klass.name}Ccw>(this);

        let previous = (*object).ref_count.fetch_sub(1, std::sync::atomic::Ordering::SeqCst);
        if previous - 1 == 0 {{
            Box::from_raw(object as *mut {klass.name}Ccw);
        }}

        (previous - 1) as std::os::raw::c_long
    }}


    {self.__gen_raw_method_impl_for_class(klass)}


    {self.__gen_class_ccw_vtbl(klass)}

    impl {self.crosscom_module_name}::ComObject for $impl_type {{
        type CcwType = {klass.name}Ccw;

        fn create_ccw(self) -> Self::CcwType {{
            Self::CcwType {{
                {self.__gen_base_struct(klass)}
                ref_count: std::sync::atomic::AtomicU32::new(0),
                inner: self,
            }}
        }}

        fn get_ccw(&self) -> &Self::CcwType {{
            unsafe {{
                let this = self as *const _ as *const u8;
                let this = this.offset(-(crosscom::offset_of!({klass.name}Ccw, inner) as isize));
                &*(this as *const Self::CcwType)
            }}
        }}
    }}
}}
    }}
}}

// pub use ComObject_{klass.name};
""")

        return w.get_value()

    def __gen_interface_method_safe_wrapper(self, i: Interface) -> str:
        w = Writer()

        for method in self.__collect_all_methods2(i.name):
            if method.name != 'query_interface':
                w.ln(f"""
pub {self.__gen_method_signature2(method)} {{
    unsafe {{
        let this = self as *const {i.name} as *const *const std::os::raw::c_void;
        let ret = ((*self.vtable).{method.name})(this, {','.join([f'{p.name}.into()' for p in method.params])});
        {self.__gen_method_ret_mapping(method)}
        ret
    }}
}}
""")
            else:
                w.ln(f"""
pub fn query_interface<T: {self.crosscom_module_name}::ComInterface>(&self) -> Option<{self.crosscom_module_name}::ComRc<T>> {{
    let this = self as *const {i.name} as *const *const std::os::raw::c_void;
    let mut raw = 0 as *const *const std::os::raw::c_void;
    let guid = uuid::Uuid::from_bytes(T::INTERFACE_ID);
    let ret_val = unsafe {{ ((*self.vtable).query_interface)(this, guid, &mut raw) }};
    if ret_val != 0 {{
        None
    }} else {{
        Some(unsafe {{ {self.crosscom_module_name}::ComRc::<T>::from_raw_pointer(raw) }})
    }}
}}
""")

        return w.get_value()

    def __gen_interface(self, i: Interface, w: Writer):
        if i.codegen_ignore():
            return

        w.ln(f'// Interface {i.name}')

        # Virtual Table
        w.ln(f"""
#[repr(C)]
#[allow(non_snake_case)]
pub struct { i.name }VirtualTable {{
""")

        for method in self.__collect_all_methods2(i.name):
            w.w(f'    pub { method.name }: ')
            self.__gen_method_raw_signature(method, w)
            w.w(',')

        w.ln('}')
        w.ln()

        # Virtual table Ccw
        w.ln(f"""
#[repr(C)]
#[allow(dead_code)]
pub struct { i.name }VirtualTableCcw {{
    pub offset: isize,
    pub vtable: { i.name }VirtualTable,
}}

""")
        # Interface implementation
        w.ln(f"""
#[repr(C)]
#[allow(dead_code)]
pub struct { i.name } {{
    pub vtable: *const { i.name }VirtualTable,
}}

#[allow(dead_code)]
#[allow(non_snake_case)]
#[allow(unused)]
impl { i.name } {{
    {self.__gen_interface_method_safe_wrapper(i)}

    pub fn uuid() -> uuid::Uuid {{
        use crosscom::ComInterface;
        uuid::Uuid::from_bytes({ i.name }::INTERFACE_ID)
    }}
}}
""")

        # Trait

        w.ln(f'pub trait {i.name}Impl {{')

        for method in i.methods:
            w.ln(f'{self.__gen_method_signature2(method)};')

        w.ln('}')
        w.ln(f"""
impl {self.crosscom_module_name}::ComInterface for {i.name} {{
            
    // {i.attrs["uuid"]}
    const INTERFACE_ID: [u8; 16] = {RustGen.__uuid_to_hex_array(i.attrs["uuid"])};
}}""")
        w.ln()

    def __get_interface_symbol(self, idl_ty):
        ty = self.symbols.get(idl_ty)
        if ty != None:
            if isinstance(ty, Class):
                raise f'Cannot use class type here: {ty}'
            else:
                return ty

        return None

    def __map_rust_internal_type(self, rust_ty: str, method_iface_module: Module) -> str:
        if "crate::" in rust_ty:
            if method_iface_module.module_name.split("::")[0] != self.get_rust_module(self.unit).module_name.split("::")[0]:
                return rust_ty.replace("crate::", method_iface_module.module_name.split("::")[0] + "::")
        return rust_ty

    def __map_raw_type(self, idl_ty: str, attrs: list[str] = None) -> str:
        is_out = attrs is not None and 'out' in attrs

        if idl_ty.endswith('[]'):
            # TODO
            inner_ty = self.__map_raw_type(idl_ty[0:-2])
            return '*const *const std::os::raw::c_void'
        elif idl_ty.endswith('?'):
            return 'crosscom::RawPointer'
        elif idl_ty in type_map:
            return type_map[idl_ty][0]
        else:
            ty = self.__get_interface_symbol(idl_ty)
            if ty != None:
                if is_out:
                    return '&mut *const *const std::os::raw::c_void'
                else:
                    return '*const *const std::os::raw::c_void'
            else:
                raise Exception('cannot find type: ' + idl_ty)

    def __map_type(self, idl_ty: str, mod_prefix=True) -> str:
        if idl_ty.endswith('[]'):
            inner_idl_ty = idl_ty[0:-2]
            inner_ty = self.__get_interface_symbol(inner_idl_ty)
            if inner_ty != None:
                mod = inner_ty.module.module_name
                return f'{self.crosscom_module_name}::ObjectArray<{mod}::{inner_ty.name}>'

        elif idl_ty.endswith('?'):
            inner_idl_ty = idl_ty[0:-1]
            inner_ty = self.__get_interface_symbol(inner_idl_ty)
            if inner_ty != None:
                mod = inner_ty.module.module_name
                return f'Option<crosscom::ComRc<{mod}::{inner_ty.name}>>'
        elif idl_ty in type_map:
            return type_map[idl_ty][1]
        else:
            ty = self.__get_interface_symbol(idl_ty)
            if ty != None:
                if mod_prefix:
                    mod = ty.module.module_name
                    return f'{self.crosscom_module_name}::ComRc<{mod}::{ty.name}>'
                else:
                    return f'{self.crosscom_module_name}::ComRc<{ty.name}>'

    def __collect_symbols(self, unit: CrossComIdl):
        symbols = {}
        for i in unit.items:
            symbols[i.name] = i
            if not hasattr(i, 'module'):
                i.module = self.get_rust_module(unit)

        return symbols

    def __uuid_to_hex_array(id: str) -> str:
        guid = uuid.UUID(id)
        return '[' + ','.join([str(b) + 'u8' for b in guid.bytes]) + ']'
