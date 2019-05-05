use crate::raw_pa_util;
use crate::types::MemoryBlock;

pub fn allocate_memory(size: i32) -> MemoryBlock {
  unsafe {
    let memory = raw_pa_util::PaUtil_AllocateMemory(size as ::std::os::raw::c_long);
    MemoryBlock {
      inner: memory
    }
  }
}

pub fn free_memory(block: &MemoryBlock) {
  unsafe {
    raw_pa_util::PaUtil_FreeMemory(block.inner)
  }
}

pub fn count_currently_allocated_blocks() -> i32 {
  unsafe {
    raw_pa_util::PaUtil_CountCurrentlyAllocatedBlocks() as i32
  }
}

pub fn initialize_clock() {
  unsafe { raw_pa_util::PaUtil_InitializeClock() }
}

pub fn get_time() -> f64 {
  unsafe { raw_pa_util::PaUtil_GetTime() }
}
