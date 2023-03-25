#![allow(clippy::mut_from_ref)]

use super::{BUFFER_SIZE, ERROR_HEADER_SIZE};

#[repr(transparent)]
pub struct TunBuffer(Box<[u8; BUFFER_SIZE]>);

impl TunBuffer {
    pub fn new() -> Self {
        Self(unsafe {
            Box::<[u8]>::new_zeroed_slice(BUFFER_SIZE)
                .assume_init()
                .try_into()
                .unwrap_unchecked()
        })
    }

    #[inline]
    pub fn buf_ref(&self) -> &mut [u8; BUFFER_SIZE] {
        unsafe {
            std::slice::from_raw_parts_mut(self.0.as_ptr().cast_mut(), BUFFER_SIZE)
                .try_into()
                .unwrap_unchecked()
        }
    }

    pub fn read_buf(&self) -> &mut [u8] {
        &mut self.buf_ref()[ERROR_HEADER_SIZE..]
    }

    pub fn read_object<T>(&self, offset: usize) -> &mut T {
        self.object(ERROR_HEADER_SIZE + offset)
    }

    pub fn object<T>(&self, offset: usize) -> &mut T {
        unsafe {
            (self.buf_ref().as_ptr().byte_offset(offset as isize) as *mut T)
                .as_mut()
                .unwrap_unchecked()
        }
    }
}

impl Default for TunBuffer {
    fn default() -> Self {
        Self::new()
    }
}
