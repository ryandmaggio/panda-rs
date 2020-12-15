use crate::prelude::*;
use crate::enums::MemRWStatus;
use std::os::raw::c_char;

// Public API ----------------------------------------------------------------------------------------------------------

/// Read from guest virtual memory
pub fn virtual_memory_read(cpu: &mut CPUState, addr: target_ulong, len: usize) -> Result<Vec<u8>, MemRWStatus> {
    let mut buf: Vec<c_char> = Vec::with_capacity(len);

    unsafe {
        let res = panda_sys::panda_virtual_memory_read_external(
            &mut *cpu,
            addr,
            buf.as_mut_ptr(),
            len as i32,
        ).into();

        match res {
            MemRWStatus::MemTxOk => {
                buf.set_len(len);
                Ok(vec_i8_into_u8(buf))
            },
            _ => Err(res)
        }
    }
}

/// Read from guest physical memory
pub fn physical_memory_read(addr: target_ulong, len: usize) -> Result<Vec<u8>, MemRWStatus> {
    let mut buf: Vec<u8> = Vec::with_capacity(len);

    unsafe {
        let res = panda_sys::panda_physical_memory_read_external(
            addr as u64,
            buf.as_mut_ptr(),
            len as i32,
        ).into();

        match res {
            MemRWStatus::MemTxOk => {
                buf.set_len(len);
                Ok(buf)
            },
            _ => Err(res)
        }
    }
}

/// Write to guest virtual memory
pub fn virtual_memory_write(cpu: &mut CPUState, addr: target_ulong, data: &[u8]) -> MemRWStatus {
    let mut c_data = vec_u8_into_i8(data.to_vec()); // Alloc b/c C API wants mut
    unsafe {
        panda_sys::panda_virtual_memory_write_external(
            &mut *cpu,
            addr,
            c_data.as_mut_ptr(),
            c_data.len() as i32,
        ).into()
    }
}

/// Write to guest physical memory
pub fn physical_memory_write(addr: target_ulong, data: &[u8]) -> MemRWStatus {
    let mut c_data = data.to_vec(); // Alloc b/c C API wants mut
    unsafe {
        panda_sys::panda_physical_memory_write_external(
            addr,
            c_data.as_mut_ptr(),
            c_data.len() as i32,
        ).into()
    }
}

/// Translate guest virtual address to physical address
pub fn virt_to_phys(cpu: &mut CPUState, addr: target_ulong) -> target_ulong {
    unsafe {
        panda_sys::panda_virt_to_phys_external(
            &mut *cpu,
            addr,
        )
    }
}

// Private API ---------------------------------------------------------------------------------------------------------

// https://stackoverflow.com/questions/59707349/cast-vector-of-i8-to-vector-of-u8-in-rust/59707887#59707887
// TODO: replace with https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_raw_parts, once on stable
fn vec_i8_into_u8(v: Vec<i8>) -> Vec<u8> {
    // Make sure v's destructor doesn't free the data it thinks it owns when it goes out of scope
    let mut v = std::mem::ManuallyDrop::new(v);

    // Pick apart the existing Vec
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    // Adopt the data into a new Vec
    unsafe { Vec::from_raw_parts(p as *mut u8, len, cap) }
}

// https://stackoverflow.com/questions/59707349/cast-vector-of-i8-to-vector-of-u8-in-rust/59707887#59707887
// TODO: replace with https://doc.rust-lang.org/std/vec/struct.Vec.html#method.into_raw_parts, once on stable
fn vec_u8_into_i8(v: Vec<u8>) -> Vec<i8> {
    // Make sure v's destructor doesn't free the data it thinks it owns when it goes out of scope
    let mut v = std::mem::ManuallyDrop::new(v);

    // Pick apart the existing Vec
    let p = v.as_mut_ptr();
    let len = v.len();
    let cap = v.capacity();

    // Adopt the data into a new Vec
    unsafe { Vec::from_raw_parts(p as *mut i8, len, cap) }
}