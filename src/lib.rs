//! Low-level bindings to the [Wren language](http://munificent.github.io/wren)
//!
//! It is recommended that you use the higher-level [wren](https://crates.io/crates/wren) crate
//! instead of this one.
//!
//! Details about embedding wren can be found at http://munificent.github.io/wren/embedding-api.html

#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
extern crate libc;

pub type size_t = ::std::os::raw::c_ulong;

pub enum WrenVM { }
pub enum WrenValue { }
pub type WrenReallocateFn =
    ::std::option::Option<unsafe extern "C" fn(memory:
                                                   *mut ::std::os::raw::c_void,
                                               newSize: size_t)
                              -> *mut ::std::os::raw::c_void>;
pub type WrenForeignMethodFn =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut WrenVM)>;
pub type WrenFinalizerFn =
    ::std::option::Option<unsafe extern "C" fn(data:
                                                   *mut ::std::os::raw::c_void)>;
pub type WrenLoadModuleFn =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut WrenVM,
                                               name:
                                                   *const ::std::os::raw::c_char)
                              -> *mut ::std::os::raw::c_char>;
pub type WrenBindForeignMethodFn =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut WrenVM,
                                               module:
                                                   *const ::std::os::raw::c_char,
                                               className:
                                                   *const ::std::os::raw::c_char,
                                               isStatic: u8,
                                               signature:
                                                   *const ::std::os::raw::c_char)
                              -> WrenForeignMethodFn>;
pub type WrenWriteFn =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut WrenVM,
                                               text:
                                                   *const ::std::os::raw::c_char)>;
#[derive(Clone, Copy)]
#[repr(u32)]
pub enum WrenErrorType {
    WREN_ERROR_COMPILE = 0,
    WREN_ERROR_RUNTIME = 1,
    WREN_ERROR_STACK_TRACE = 2,
}
pub type WrenErrorFn =
    ::std::option::Option<unsafe extern "C" fn(_type: WrenErrorType,
                                               module:
                                                   *const ::std::os::raw::c_char,
                                               line: ::std::os::raw::c_int,
                                               message:
                                                   *const ::std::os::raw::c_char)>;
#[repr(C)]
#[derive(Copy)]
pub struct WrenForeignClassMethods {
    pub allocate: WrenForeignMethodFn,
    pub finalize: WrenFinalizerFn,
}
impl ::std::clone::Clone for WrenForeignClassMethods {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for WrenForeignClassMethods {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
pub type WrenBindForeignClassFn =
    ::std::option::Option<unsafe extern "C" fn(vm: *mut WrenVM,
                                               module:
                                                   *const ::std::os::raw::c_char,
                                               className:
                                                   *const ::std::os::raw::c_char)
                              -> WrenForeignClassMethods>;
#[repr(C)]
#[derive(Copy)]
pub struct WrenConfiguration {
    pub reallocateFn: WrenReallocateFn,
    pub loadModuleFn: WrenLoadModuleFn,
    pub bindForeignMethodFn: WrenBindForeignMethodFn,
    pub bindForeignClassFn: WrenBindForeignClassFn,
    pub writeFn: WrenWriteFn,
    pub errorFn: WrenErrorFn,
    pub initialHeapSize: size_t,
    pub minHeapSize: size_t,
    pub heapGrowthPercent: ::std::os::raw::c_int,
}
impl ::std::clone::Clone for WrenConfiguration {
    fn clone(&self) -> Self { *self }
}
impl ::std::default::Default for WrenConfiguration {
    fn default() -> Self { unsafe { ::std::mem::zeroed() } }
}
#[derive(Clone, Copy, Debug, PartialEq)]
#[repr(u32)]
pub enum WrenInterpretResult {
    WREN_RESULT_SUCCESS = 0,
    WREN_RESULT_COMPILE_ERROR = 1,
    WREN_RESULT_RUNTIME_ERROR = 2,
}

#[derive(Clone, Copy)]
#[repr(u32)]
pub enum WrenType {
    WREN_TYPE_BOOL = 0,
    WREN_TYPE_NUM = 1,
    WREN_TYPE_FOREIGN = 2,
    WREN_TYPE_LIST = 3,
    WREN_TYPE_NULL = 4,
    WREN_TYPE_STRING = 5,
    WREN_TYPE_UNKNOWN = 6,
}
extern "C" {
    pub fn wrenInitConfiguration(configuration: *mut WrenConfiguration);
    pub fn wrenNewVM(configuration: *mut WrenConfiguration) -> *mut WrenVM;
    pub fn wrenFreeVM(vm: *mut WrenVM);
    pub fn wrenCollectGarbage(vm: *mut WrenVM);
    pub fn wrenInterpret(vm: *mut WrenVM,
                         source: *const ::std::os::raw::c_char)
     -> WrenInterpretResult;
    pub fn wrenMakeCallHandle(vm: *mut WrenVM,
                              signature: *const ::std::os::raw::c_char)
     -> *mut WrenValue;
    pub fn wrenCall(vm: *mut WrenVM, method: *mut WrenValue)
     -> WrenInterpretResult;
    pub fn wrenReleaseValue(vm: *mut WrenVM, value: *mut WrenValue);
    pub fn wrenGetSlotCount(vm: *mut WrenVM) -> ::std::os::raw::c_int;
    pub fn wrenEnsureSlots(vm: *mut WrenVM, numSlots: ::std::os::raw::c_int);
    pub fn wrenGetSlotType(vm: *mut WrenVM, slot: ::std::os::raw::c_int)
     -> WrenType;
    pub fn wrenGetSlotBool(vm: *mut WrenVM, slot: ::std::os::raw::c_int)
     -> u8;
    pub fn wrenGetSlotBytes(vm: *mut WrenVM, slot: ::std::os::raw::c_int,
                            length: *mut ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_char;
    pub fn wrenGetSlotDouble(vm: *mut WrenVM, slot: ::std::os::raw::c_int)
     -> ::std::os::raw::c_double;
    pub fn wrenGetSlotForeign(vm: *mut WrenVM, slot: ::std::os::raw::c_int)
     -> *mut ::std::os::raw::c_void;
    pub fn wrenGetSlotString(vm: *mut WrenVM, slot: ::std::os::raw::c_int)
     -> *const ::std::os::raw::c_char;
    pub fn wrenGetSlotValue(vm: *mut WrenVM, slot: ::std::os::raw::c_int)
     -> *mut WrenValue;
    pub fn wrenSetSlotBool(vm: *mut WrenVM, slot: ::std::os::raw::c_int,
                           value: u8);
    pub fn wrenSetSlotBytes(vm: *mut WrenVM, slot: ::std::os::raw::c_int,
                            bytes: *const ::std::os::raw::c_char,
                            length: size_t);
    pub fn wrenSetSlotDouble(vm: *mut WrenVM, slot: ::std::os::raw::c_int,
                             value: ::std::os::raw::c_double);
    pub fn wrenSetSlotNewForeign(vm: *mut WrenVM, slot: ::std::os::raw::c_int,
                                 classSlot: ::std::os::raw::c_int,
                                 size: size_t) -> *mut ::std::os::raw::c_void;
    pub fn wrenSetSlotNewList(vm: *mut WrenVM, slot: ::std::os::raw::c_int);
    pub fn wrenSetSlotNull(vm: *mut WrenVM, slot: ::std::os::raw::c_int);
    pub fn wrenSetSlotString(vm: *mut WrenVM, slot: ::std::os::raw::c_int,
                             text: *const ::std::os::raw::c_char);
    pub fn wrenSetSlotValue(vm: *mut WrenVM, slot: ::std::os::raw::c_int,
                            value: *mut WrenValue);
    pub fn wrenInsertInList(vm: *mut WrenVM, listSlot: ::std::os::raw::c_int,
                            index: ::std::os::raw::c_int,
                            elementSlot: ::std::os::raw::c_int);
    pub fn wrenGetVariable(vm: *mut WrenVM,
                           module: *const ::std::os::raw::c_char,
                           name: *const ::std::os::raw::c_char,
                           slot: ::std::os::raw::c_int);
}

#[cfg(test)] mod tests {
    use std::default::Default;
    use super::{WrenVM, wrenInitConfiguration, wrenNewVM, WrenConfiguration, wrenInterpret, wrenFreeVM, WrenInterpretResult, size_t};
    use std::ffi::{CString, CStr};
    use libc;

    #[test]
    fn test_interpret_code() {
        unsafe {
            let mut config: WrenConfiguration = Default::default();
            wrenInitConfiguration(&mut config);
            config.reallocateFn = Some(allocate);
            config.writeFn = Some(write);
            let vm = wrenNewVM(&mut config);
            let source = CString::new(r#" System.print("hello, world") "#)
                             .unwrap()
                             .as_ptr();
            let result = wrenInterpret(vm, source as *const _);
            assert_eq!(result, WrenInterpretResult::WREN_RESULT_SUCCESS);
            wrenFreeVM(vm);

            /* These two functions are for the Wren VM to use to write output, and manage memory, respectively */

            unsafe extern "C" fn write(_: *mut WrenVM, text: *const ::std::os::raw::c_char) {
                println!("{}", CStr::from_ptr(text).to_string_lossy().into_owned());
            }

            unsafe extern "C" fn allocate(memory: *mut ::std::os::raw::c_void, new_size: size_t) -> *mut ::std::os::raw::c_void {
                if new_size == 0 {
                    libc::free(::std::mem::transmute(memory));
                    memory
                } else {
                    ::std::mem::transmute(libc::realloc(::std::mem::transmute(memory), ::std::mem::transmute(new_size)))
                }
            }
        }
    }
}
