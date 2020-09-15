use crate::{raw, RawError, Softdevice};
use defmt::info;
use fixed::types::I30F2;

#[derive(defmt::Format)]
pub enum TempError {
    Raw(RawError),
}

impl From<RawError> for TempError {
    fn from(err: RawError) -> Self {
        TempError::Raw(err)
    }
}

/// Get temperature reading in Celcius
///
/// Note this blocks for ~50us
pub fn temperature_celsius(_sd: &Softdevice) -> Result<I30F2, TempError> {
    let mut temp: i32 = 0;
    let ret = unsafe { raw::sd_temp_get(&mut temp) };
    match RawError::convert(ret) {
        Ok(()) => {}
        Err(err) => {
            info!("sd_temp_get err {:?}", err);
            return Err(TempError::Raw(err));
        }
    }
    Ok(I30F2::from_bits(temp))
}
