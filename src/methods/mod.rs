mod hardware;
mod emulation;
mod emulation_unchecked;
mod succpred;
mod succpred_unchecked;
mod roughwrap;
mod roughwrap_unchecked;

pub use self::hardware::Hardware;

pub use self::emulation::EmulationRegular;
#[cfg(any(feature = "use-fma", feature = "doc"))]
pub use self::emulation::EmulationFma;
#[cfg(not(feature = "use-fma"))]
pub use self::EmulationRegular as Emulation;
#[cfg(any(feature = "use-fma", feature = "doc"))]
pub use self::EmulationFma as Emulation;

pub use self::emulation_unchecked::EmulationRegularUnchecked;
#[cfg(any(feature = "use-fma", feature = "doc"))]
pub use self::emulation_unchecked::EmulationFmaUnchecked;
#[cfg(not(feature = "use-fma"))]
pub use self::EmulationRegularUnchecked as EmulationUnchecked;
#[cfg(any(feature = "use-fma", feature = "doc"))]
pub use self::EmulationFmaUnchecked as EmulationUnchecked;

pub use self::succpred::SuccPred;
pub use self::succpred_unchecked::SuccPredUnchecked;

pub use self::roughwrap::RoughWrapping;
pub use self::roughwrap_unchecked::RoughWrappingUnchecked;
