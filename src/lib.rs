#![allow(non_snake_case)]

extern crate libc;

#[derive(Copy)]
pub enum StructWrenVM { }
pub type WrenVM = StructWrenVM;
pub type WrenReallocateFn =
    ::std::option::Option<extern "C" fn
                              (memory: *mut ::libc::c_void, oldSize: ::libc::size_t,
                               newSize: ::libc::size_t) -> *mut ::libc::c_void>;
pub type WrenForeignMethodFn =
    ::std::option::Option<extern "C" fn(vm: *mut WrenVM)>;
#[repr(C)]
#[derive(Copy)]
pub struct Struct_Unnamed1 {
    pub reallocateFn: WrenReallocateFn,
    pub initialHeapSize: ::libc::size_t,
    pub minHeapSize: ::libc::size_t,
    pub heapGrowthPercent: ::libc::c_int,
}
impl ::std::default::Default for Struct_Unnamed1 {
    fn default() -> Struct_Unnamed1 { unsafe { ::std::mem::zeroed() } }
}
pub type WrenConfiguration = Struct_Unnamed1;
pub type EnumUnnamed2 = ::libc::c_uint;
pub const WREN_RESULT_SUCCESS: ::libc::c_uint = 0;
pub const WREN_RESULT_COMPILE_ERROR: ::libc::c_uint = 1;
pub const WREN_RESULT_RUNTIME_ERROR: ::libc::c_uint = 2;
pub type WrenInterpretResult = EnumUnnamed2;
extern "C" {
    pub fn wrenNewVM(configuration: *mut WrenConfiguration) -> *mut WrenVM;
    pub fn wrenFreeVM(vm: *mut WrenVM);
    pub fn wrenInterpret(vm: *mut WrenVM, sourcePath: *const ::libc::c_char,
                         source: *const ::libc::c_char)
     -> WrenInterpretResult;
    pub fn wrenDefineMethod(vm: *mut WrenVM, className: *const ::libc::c_char,
                            methodName: *const ::libc::c_char,
                            numParams: ::libc::c_int,
                            method: WrenForeignMethodFn);
    pub fn wrenDefineStaticMethod(vm: *mut WrenVM,
                                  className: *const ::libc::c_char,
                                  methodName: *const ::libc::c_char,
                                  numParams: ::libc::c_int,
                                  method: WrenForeignMethodFn);
    pub fn wrenGetArgumentDouble(vm: *mut WrenVM, index: ::libc::c_int)
     -> ::libc::c_double;
    pub fn wrenGetArgumentString(vm: *mut WrenVM, index: ::libc::c_int)
     -> *const ::libc::c_char;
    pub fn wrenReturnDouble(vm: *mut WrenVM, value: ::libc::c_double);
    pub fn wrenReturnNull(vm: *mut WrenVM);
    pub fn wrenReturnString(vm: *mut WrenVM, text: *const ::libc::c_char,
                            length: ::libc::c_int);
}

#[cfg(test)]
mod test {
    use std::default::Default;
    use super::{wrenNewVM, WrenConfiguration, wrenInterpret,
                wrenFreeVM, WREN_RESULT_SUCCESS, };
    use std::c_str::ToCStr;

    #[test]
    fn test_new_vm() {
        unsafe {
            let mut config: WrenConfiguration = Default::default();
            let mut vm = wrenNewVM(&mut config);
            let source_path = "".to_c_str().as_ptr();
            let source = r#"
class Unicorn {
    hasHorn {
        return true
    }
}
            "#.to_c_str().as_ptr();
            let result = wrenInterpret(vm, source_path, source);
            assert_eq!(result, WREN_RESULT_SUCCESS);
            wrenFreeVM(vm);
        }
    }
}
