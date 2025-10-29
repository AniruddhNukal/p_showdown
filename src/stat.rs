use std::cmp::Eq;

macro_rules! define_stat_newtype {
    ($struct_name:ident, $mask:expr) => {
        #[derive(Debug, Eq, PartialEq, Default, Clone, Copy)]
        pub struct $struct_name(u16);

        impl Stat for $struct_name {
            fn new(i: u16) -> Self {
                Self($mask & i)
            }

            fn get(&self) -> u16 {
                self.0
            }
        }
    };

    ($struct_name:ident) => {
        define_stat_newtype!($struct_name, 0xFFFF);
    };
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

#[derive(Debug, Default, Clone)]
pub struct StatVec<T: Stat> {
    hp: T,
    atk: T,
    def: T,
    spa: T,
    spd: T,
    spe: T,
}

impl<T: Stat> StatVec<T> {
    pub fn new(hp: T, atk: T, def: T, spa: T, spd: T, spe: T) -> Self {
        StatVec::<T> {
            hp,
            atk,
            def,
            spa,
            spd,
            spe,
        }
    }

    pub fn from_int(hp: u16, atk: u16, def: u16, spa: u16, spd: u16, spe: u16) -> Self {
        StatVec::<T> {
            hp: T::new(hp),
            atk: T::new(atk),
            def: T::new(def),
            spa: T::new(spa),
            spd: T::new(spd),
            spe: T::new(spe),
        }
    }
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

pub trait Stat {
    fn new(i: u16) -> Self;
    fn get(&self) -> u16;
}

define_stat_newtype!(BaseStat);
define_stat_newtype!(EV, 0xFF);
define_stat_newtype!(IV, 0x1F);

pub trait Stage {
    fn new() -> Self;
    fn get(&self) -> f64;
    fn increment(&mut self);
    fn decrement(&mut self);
}

impl_stage_newtype!(StatStage, 2.);
impl_stage_newtype!(AccEvaStage, 3.);

#[cfg(test)]
mod tests {
    use crate::stat::{EV, IV, Stage, Stat, StatStage};

    #[test]
    fn mask_test_ev() {
        let proper = EV::new(2);
        let improper = EV::new(258);

        assert_eq!(proper, improper)
    }

    #[test]
    fn mask_test_iv() {
        let proper = IV::new(31);
        let improper = IV::new(63);

        assert_eq!(proper, improper)
    }

    #[test]
    fn upper_bound_test_stage() {
        let mut max = StatStage::new();
        for i in 0..6 {
            max.increment();
        }

        let mut max_more = max.clone();
        max_more.increment();

        assert_eq!(max, max_more)
    }
}
