// Note that nothing within this module is allowed to allocate memory. The data structures must
// live in reserved blocks.
// This makes it dangerous to use, for example, println or anything that will invoke string
// formatting machinery.
#![no_std]
#![feature(format_args_nl)]
#![feature(cstr_from_bytes_until_nul)]
#![feature(default_alloc_error_handler)]

extern crate ffi_bindings;

use core::ffi::CStr;
use core::usize::MAX;
use heapless::spsc::Queue;
use heapless::Vec;
use spin::Mutex;

use ffi_bindings::cstr_core::CString;
use ffi_bindings::{
    assert, boot_info_get, println, BootInfo, PhysicalMemoryRegionType, _panic,
    phys_addr_to_virt_ram_remap, printf, PhysicalAddr,
};

// PT: Must match the definitions in kernel/ap_bootstrap.h
const AP_BOOTSTRAP_CODE_PAGE: usize = 0x8000;
const AP_BOOTSTRAP_DATA_PAGE: usize = 0x9000;

static FRAMES_TO_HIDE_FROM_PMM: &'static [usize] = &[
    // Used for AP bootstrap code + data
    AP_BOOTSTRAP_CODE_PAGE,
    AP_BOOTSTRAP_DATA_PAGE,
];

const PAGE_SIZE: usize = 0x1000;
const MEGABYTE: usize = 1024 * 1024;
const GIGABYTE: usize = MEGABYTE * 1024;
// Maximum physical memory that the PMM can keep track of (ie. the maximum physical memory we can
// allocate)
const MAX_MEMORY_ALLOCATOR_CAN_BOOKKEEP: usize = GIGABYTE * 16;
const MAX_FRAMES_ALLOCATOR_CAN_BOOKKEEP: usize = MAX_MEMORY_ALLOCATOR_CAN_BOOKKEEP / PAGE_SIZE;

#[derive(Debug, Copy, Clone)]
struct PhysicalFrame(u64);

/// If we have a maximum of 16GB of RAM tracked, each array to track the frames will occupy 512kb.
static mut FREE_FRAMES: Mutex<Queue<PhysicalAddr, MAX_FRAMES_ALLOCATOR_CAN_BOOKKEEP>> =
    Mutex::new(Queue::new());

fn page_ceil(mut addr: usize) -> usize {
    ((addr) + PAGE_SIZE - 1) & !(PAGE_SIZE - 1)
}

fn page_floor(mut addr: usize) -> usize {
    (addr - (addr % PAGE_SIZE))
}

#[no_mangle]
pub unsafe fn pmm_init() {
    let boot_info = {
        let boot_info_raw = boot_info_get();
        &*boot_info_raw
    };

    // Mark usable sections of the address space
    // TODO(PT): Will there be any bug with PMM allocating the frame used for the init kernel stack?
    let mut free_frames_queue = FREE_FRAMES.lock();
    for region in &boot_info.mem_regions[..boot_info.mem_region_count as usize] {
        if region.region_type != PhysicalMemoryRegionType::Usable {
            continue;
        }
        // Floor each region to a frame boundary
        // This will cut off a bit of usable memory, but we'll only lose a few frames at most
        let base = page_ceil(region.addr);
        // Subtract whatever extra we got by aligning to a frame boundary above
        let mut region_size = page_floor(region.len - (base - region.addr));

        let page_count = (region_size + (PAGE_SIZE - 1)) / PAGE_SIZE;
        for page_idx in 0..page_count {
            let frame_addr = base + (page_idx * PAGE_SIZE);
            if FRAMES_TO_HIDE_FROM_PMM.contains(&frame_addr) {
                continue;
            }
            // Keep track of the frame
            free_frames_queue.enqueue(PhysicalAddr(frame_addr));
        }
    }
}

#[no_mangle]
pub unsafe fn pmm_alloc() -> usize {
    let mut free_frames_queue = FREE_FRAMES.lock();
    if free_frames_queue.is_empty() {
        // Invoke _panic directly, since the panic! macro will invoke string formatting
        // machinery (which is not allowed here)
        _panic(
            "Exhausted available physical frames\0".as_ptr() as *const u8,
            "pmm/lib.rs\0".as_ptr() as *const u8,
            0,
        );
    }
    let allocated_frame = free_frames_queue.dequeue().unwrap();

    // Memset the frame to all zeroes, for convenience
    let frame_in_ram_remap = phys_addr_to_virt_ram_remap(allocated_frame);
    let frame_slice_in_vmem = {
        let raw_slice =
            core::ptr::slice_from_raw_parts_mut(frame_in_ram_remap.0 as *mut u8, PAGE_SIZE);
        &mut *raw_slice
    };
    frame_slice_in_vmem.fill(0);

    return allocated_frame.0;
}

#[no_mangle]
pub unsafe fn pmm_free(frame_addr: usize) {
    let mut free_frames_queue = FREE_FRAMES.lock();
    free_frames_queue.enqueue(PhysicalAddr(frame_addr)).unwrap();
}
