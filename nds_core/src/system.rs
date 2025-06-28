#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
pub struct Battery(u8);

impl Battery {
    fn from_raw(raw: u32) -> Self {
        Self(raw as u8)
    }

    pub fn level(&self) -> BatteryLevel {
        match self.0 & 0x0F {
            3 => BatteryLevel::Low,
            15 => BatteryLevel::High,
            level => BatteryLevel::Level(level),
        }
    }

    pub fn external(&self) -> bool {
        self.0 & 0x80 != 0
    }
}

pub enum BatteryLevel {
    Low,
    High,
    Level(u8),
}

pub fn battery() -> Battery {
    unsafe { Battery::from_raw(nds_sys::getBatteryLevel()) }
}
