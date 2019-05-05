use crate::raw_pa_ringbuffer;
use crate::rpa_error::RingBufferError;
use crate::types::{MemoryBlock, PaUtilRingBuffer, RingBufferSizeT};

pub fn initialize_ring_buffer(
  element_size_bytes: RingBufferSizeT,
  element_count: RingBufferSizeT,
  data_ptr: &MemoryBlock,
) -> Result<PaUtilRingBuffer, RingBufferError> {
  let mut pa_ringbuffer = ::std::ptr::null_mut();

  let rb_init_ans = unsafe {
    raw_pa_ringbuffer::PaUtil_InitializeRingBuffer(pa_ringbuffer,
                                                   element_size_bytes as raw_pa_ringbuffer::ring_buffer_size_t,
                                                   element_count as raw_pa_ringbuffer::ring_buffer_size_t,
                                                   data_ptr.inner) as i32
  };
  if rb_init_ans == -1 {
    return Err(RingBufferError::NotPower2("Ring buffer size is not power of 2."));
  }

  let rpa_ringbuffer = PaUtilRingBuffer {
    inner: pa_ringbuffer
  };
  Ok(rpa_ringbuffer)
}


pub fn flush_ring_buffer(rbuf: &PaUtilRingBuffer) {
  unsafe { raw_pa_ringbuffer::PaUtil_FlushRingBuffer(rbuf.inner) }
}


pub fn write_ring_buffer(rbuf: &PaUtilRingBuffer) {}

