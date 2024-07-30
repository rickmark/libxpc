#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

use libc::{size_t, uuid_t};

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

type xpc_msg_id_t = u64;
type xpc_size_t = u64;
type xpc_length_t = u32;
type xpc_count_t = u32;

const XPC_1_MAGIC: u32 = 0x58504321;
const XPC_4_MAGIC: u32 = 0x40585043;
const XPC_5_MAGIC: u32 = 0x29B00B92;

#[repr(u32)]
enum xpc_flags {
    REMOTE = 1 << 0,
    DATA_PRESENT = 1 << 8,
    HEARTBEAT_REQUEST = 1 << 16,
    HEARTBEAT_REPLY = 1 << 17,
    FILE_TX_OPEN = 1 << 20,
    FILE_TX_REPLY = 1 << 21,
    SYSDIAGNOSE_INIT = 1 << 22
}

#[repr(u32)]
enum xpc_magic {
    MAGIC_1 = XPC_1_MAGIC,
    MAGIC_4 = XPC_4_MAGIC,
    MAGIC_5 = XPC_5_MAGIC
}

#[repr(u32)]
enum xpc_type_t {
    XPC_NULL              = 0x00001000,
    XPC_BOOL              = 0x00002000,
    XPC_INT64             = 0x00003000,
    XPC_UINT64            = 0x00004000,
    XPC_DOUBLE            = 0x00005000,
    XPC_POINTER           = 0x00006000,
    XPC_DATE              = 0x00007000,
    XPC_DATA              = 0x00008000,
    XPC_STRING            = 0x00009000,
    XPC_UUID              = 0x0000a000,
    XPC_FD                = 0x0000b000,
    XPC_SHMEM             = 0x0000c000,
    XPC_MACH_SEND         = 0x0000d000,
    XPC_ARRAY             = 0x0000e000,
    XPC_DICTIONARY        = 0x0000f000,
    XPC_ERROR             = 0x00010000,
    XPC_CONNECTION        = 0x00011000,
    XPC_ENDPOINT          = 0x00012000,
    XPC_SERIALIZER        = 0x00013000,
    XPC_PIPE              = 0x00014000,
    XPC_MACH_RECV         = 0x00015000,
    XPC_BUNDLE            = 0x00016000,
    XPC_SERVICE           = 0x00017000,
    XPC_SERVICE_INSTANCE  = 0x00018000,
    XPC_ACTIVITY          = 0x00019000,
    XPC_FILE_TRANSFER     = 0x0001a000
}

struct xpc_uint64_t {
    xpc_type: xpc_type_t,
    value: u64
}

struct xpc_uint32_t {
    xpc_type: xpc_type_t,
    value: u32
}

struct xpc_bool_t {
    xpc_type:xpc_type_t,
    value: bool
}

struct xpc_int64_t {
    xpc_type: xpc_type_t,
    value: i64
}

struct xpc_int32_t {
    xpc_type: xpc_type_t,
    value: i32
}

struct xpc_uuid_t {
    xpc_type: xpc_type_t,
    value: uuid_t
}

struct xpc_shmem_t {
    xpc_type: xpc_type_t,
    data: *mut u8,
    length: size_t
}

struct xpc_tlv_t {
     xpc_type: xpc_type_t,
     length: xpc_length_t,
     data: *const u8
}

struct xpc_dictionary_entry_t {
    key: *const str,
    xpc_type: xpc_type_t,
    value: xpc_object_t
}

struct xpc_array_entry_t {
    xpc_type: xpc_type_t,
    value: xpc_object_t
}

struct xpc_compound_t {
    xpc_type: xpc_type_t,
    length: xpc_length_t,
    count: xpc_count_t
}

struct xpc_wrapper_t
{
    magic: xpc_magic,
    flags: xpc_flags,
    body_len: xpc_size_t,
    msg_id: xpc_msg_id_t
}

struct xpc_file_transfer_t {
    xpc_type: xpc_type_t,
    msg_id: xpc_msg_id_t,
    data: xpc_compound_t
}

union xpc_object_t {
    uint64: xpc_uint64_t,
    uint32: xpc_uint32_t,
    int32: xpc_int32_t,
    int64: xpc_int64_t,
    dictionary: xpc_compound_t,
    file_transfer: xpc_file_transfer_t,
    uuid: xpc_uuid_t,
    shmem: xpc_shmem_t,
    array: xpc_compound_t
}

