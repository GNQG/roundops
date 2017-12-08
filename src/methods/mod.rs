extern crate safeeft;
#[cfg(feature = "use-fma")]
extern crate fma;

mod hardware;
mod emulation;
mod emulation_unchecked;
mod succpred;
mod succpred_unchecked;
mod roughwrap;
mod roughwrap_unchecked;

pub use self::emulation::EmulationRegular;
#[cfg(feature = "use-fma")]
pub use self::emulation::EmulationFma;
#[cfg(not(feature = "use-fma"))]
pub use self::EmulationRegular as Emulation;
#[cfg(feature = "use-fma")]
pub use self::EmulationFma as Emulation;

pub use self::emulation_unchecked::EmulationUnchecked;

pub use self::succpred::SuccPred;
pub use self::succpred_unchecked::SuccPredUnchecked;

pub use self::roughwrap::RoughWrapping;
pub use self::roughwrap_unchecked::RoughWrappingUnchecked;
