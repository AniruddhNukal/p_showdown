use thiserror::Error;

#[derive(Debug, Error)]
pub enum StatError {
    #[error("value of {value} cannot be stored in stat with bound of {bound}")]
    OutOfBounds { value: u16, bound: u16 },
}

pub struct IVSpread {
    stats: [u16; 6],
}

impl IVSpread {
    pub fn new<T: Into<u16> + Clone>(ivs: &[T; 6]) -> Result<Self, StatError> {
        let mut stats = [0u16; 6];
        for (i, iv) in ivs.iter().enumerate() {
            let iv = iv.clone().into();
            if iv > 31 {
                return Err(StatError::OutOfBounds {
                    value: iv,
                    bound: 31,
                });
            }
            stats[i] = iv;
        }
        Ok(IVSpread { stats })
    }

    pub fn get_stats(self) -> [u16; 6] {
        self.stats
    }
}

pub struct EVSpread {
    stats: [u16; 6],
}

impl EVSpread {
    pub fn new<T: Into<u16> + Clone>(evs: &[T; 6]) -> Result<Self, StatError> {
        let mut stats = [0u16; 6];
        for (i, ev) in evs.iter().enumerate() {
            let ev = ev.clone().into();
            if ev > 255 {
                return Err(StatError::OutOfBounds {
                    value: ev,
                    bound: 255,
                });
            }
            stats[i] = ev;
        }
        let total = stats.iter().sum();
        if total > 512 {
            return Err(StatError::OutOfBounds {
                value: total,
                bound: 512,
            });
        }
        Ok(EVSpread { stats })
    }

    pub fn get_stats(self) -> [u16; 6] {
        self.stats
    }
}

macro_rules! impl_stage_newtype {
    ($struct_name:ident, $base_val:expr) => {
        #[derive(Debug, Eq, PartialEq, Default, Clone)]
        pub struct $struct_name(i8);

        impl Stage for $struct_name {
            fn new() -> Self {
                Self(0)
            }

            fn get(&self) -> f64 {
                let base: f64 = $base_val;
                let modif: f64 = $base_val + (self.0 as f64);
                if self.0 > 0 {
                    return modif / base;
                }
                base / modif
            }

            fn increment(&mut self) {
                if self.0 != 6 {
                    self.0 += 1
                }
            }

            fn decrement(&mut self) {
                if self.0 != -6 {
                    self.0 -= 1
                }
            }
        }
    };
}

#[derive(Debug, Default)]
pub struct StageVec {
    atk: StatStage,
    def: StatStage,
    spa: StatStage,
    spd: StatStage,
    spe: StatStage,
    acc: AccEvaStage,
    eva: AccEvaStage,
}

impl StageVec {
    pub fn new() -> Self {
        StageVec::default()
    }
}

pub trait Stage {
    fn new() -> Self;
    fn get(&self) -> f64;
    fn increment(&mut self);
    fn decrement(&mut self);
}

impl_stage_newtype!(StatStage, 2.);
impl_stage_newtype!(AccEvaStage, 3.);

#[cfg(test)]
mod tests {}
