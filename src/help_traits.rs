pub trait AccesBigEndianBytesU16: Into<u16> {
    fn get_high_mut(&mut self) -> &mut u8;
    fn get_low_mut(&mut self) -> &mut u8;
    fn get_high(self) -> u8 {
        let [high, _low] = u16::to_be_bytes(self.into());
        high
    }
    fn get_low(self) -> u8 {
        let [_high, low] = u16::to_be_bytes(self.into());
        low
    }
    fn set_high(&mut self, byte: u8) {
        *self.get_high_mut() = byte;
    }
    fn set_low(&mut self, byte: u8) {
        *self.get_low_mut() = byte;
    }
}

#[cfg(target_endian = "little")]
impl AccesBigEndianBytesU16 for u16 {
    fn get_high_mut(&mut self) -> &mut u8 {
        unsafe {
            let ptr = self as *mut u16 as *mut u8;
            &mut *ptr.offset(1)
        }
    }

    fn get_low_mut(&mut self) -> &mut u8 {
        unsafe {
            let ptr = self as *mut u16 as *mut u8;
            &mut *ptr
        }
    }
}

#[cfg(target_endian = "big")]
impl AccesBigEndianBytesU16 for u16 {
    fn get_high(&mut self) -> &mut u8 {
        unsafe {
            let ptr = self as *mut u16 as *mut u8;
            &mut *ptr
        }
    }

    fn get_low(&mut self) -> &mut u8 {
        unsafe {
            let ptr = self as *mut u16 as *mut u8;
            &mut *ptr.offset(1)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AccesBigEndianBytesU16;

    #[test]
    fn test() {
        let mut value: u16 = 0;

        const HIGH_BYTE: u8 = 0x56;
        const LOW_BYTE: u8 = 0x24;

        *value.get_high_mut() = HIGH_BYTE;
        *value.get_low_mut() = LOW_BYTE;

        let [high, low] = u16::to_be_bytes(value);

        assert_eq!(high, HIGH_BYTE);
        assert_eq!(low, LOW_BYTE);
        assert_eq!(value, u16::from_be_bytes([HIGH_BYTE, LOW_BYTE]));
    }
}
