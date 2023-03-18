use super::{BUFFER_SIZE, ERROR_HEADER_SIZE};

#[repr(transparent)]
pub struct TunBuffer(Box<[u8; BUFFER_SIZE]>);

impl TunBuffer {
    pub fn new() -> Self {
        Self(
            unsafe { Box::<[u8]>::new_zeroed_slice(BUFFER_SIZE).assume_init() }
                .try_into()
                .unwrap(),
        )
    }

    #[inline]
    pub fn as_ref(&mut self) -> &mut [u8; BUFFER_SIZE] {
        self.0.as_mut()
    }

    pub fn read_buffer(&mut self) -> &mut [u8] {
        &mut self.as_ref()[ERROR_HEADER_SIZE..]
    }
}
