use core::marker::PhantomData;
use core::ops::{Neg, Add, Sub, Mul, Div};

pub mod rmode {
    #[cfg(target_env = "msvc")]
    extern "C" {
        fn _controlfp_s(current: *mut u32, new: u32, mask: u32) -> u32;
    }

    pub trait NativeRoundingMode {}

    pub enum DefaultRounding {}

    impl NativeRoundingMode for DefaultRounding {}

    pub enum Switchable {}

    impl NativeRoundingMode for Switchable {}

    pub trait EditRoundingMode: Sized {
        type RoundingState: Clone;

        fn rmode_controler() -> Result<RoundingModeControler<Self>, ()>;
        fn current_rounding_state() -> Self::RoundingState;
        unsafe fn set_rounding_state(Self::RoundingState);
        unsafe fn upward();
        unsafe fn downward();
        unsafe fn to_nearest();
        unsafe fn toward_zero();
    }

    #[cfg(all(feature = "hwrm", any(target_arch = "x86", target_arch = "x86_64")))]
    mod rmodelocal {
        extern crate stdsimd;
        #[cfg(target_feature = "sse")]
        use self::stdsimd::vendor::{_MM_GET_ROUNDING_MODE, _MM_SET_ROUNDING_MODE, _MM_ROUND_DOWN,
                                    _MM_ROUND_NEAREST, _MM_ROUND_TOWARD_ZERO, _MM_ROUND_UP};
        use super::{EditRoundingMode,RoundingModeControler};
        macro_rules! impl_rmode {
            ($type:ty) => (
                impl EditRoundingMode for $type {
                    #[cfg(target_feature = "-sse")]
                    type RoundingState = u16;
                    #[cfg(all(target_arch = "x86",target_feature = "sse"))]
                    type RoundingState = (u16, u32);
                    #[cfg(all(target_arch = "x86_64",target_feature = "sse"))]
                    type RoundingState = u32;

                    #[inline]
                    fn rmode_controler() -> Result<RoundingModeControler<Self>, ()> {
                        #[cfg(target_feature = "-sse")]
                        {
                            Ok(RoundingModeControler {
                                initial_state: {()/* FPU */}
                            })
                        }
                        #[cfg(all(target_arch = "x86",target_feature = "sse"))]
                        {
                            Ok(RoundingModeControler {
                                initial_state: {({()/* FPU */} ,unsafe{_MM_GET_ROUNDING_MODE()})}
                            })
                        }
                        #[cfg(all(target_arch = "x86_64",target_feature = "sse"))]
                        {
                            Ok(RoundingModeControler {
                                initial_state: unsafe{_MM_GET_ROUNDING_MODE()}
                            })
                        }
                    }
                    #[inline]
                    fn current_rounding_state() -> Self::RoundingState {
                        #[cfg(target_feature = "-sse")]
                            {()/* FPU */}
                        #[cfg(all(target_arch = "x86",target_feature = "sse"))]
                            {({()/* FPU */} ,unsafe{_MM_GET_ROUNDING_MODE()})}
                        #[cfg(all(target_arch = "x86_64",target_feature = "sse"))]
                            unsafe{_MM_GET_ROUNDING_MODE()}
                    }
                    #[inline]
                    unsafe fn set_rounding_state(state: Self::RoundingState){
                        #[cfg(target_feature = "-sse")]
                            {()/* FPU */}
                        #[cfg(all(target_arch = "x86",target_feature = "sse"))]
                            {({()/* FPU */} ,_MM_GET_ROUNDING_MODE(state.1))}
                        #[cfg(all(target_arch = "x86_64",target_feature = "sse"))]
                            {_MM_SET_ROUNDING_MODE(state)}
                    }
                    #[inline]
                    unsafe fn upward() {
                        #[cfg(target_feature = "-sse")]
                            {()/* FPU */}
                        #[cfg(all(target_arch = "x86",target_feature = "sse"))]
                            {({()/* FPU */} ,_MM_GET_ROUNDING_MODE(_MM_ROUND_UP))}
                        #[cfg(all(target_arch = "x86_64",target_feature = "sse"))]
                            {_MM_SET_ROUNDING_MODE(_MM_ROUND_UP)}
                    }
                    #[inline]
                    unsafe fn downward() {
                        #[cfg(target_feature = "-sse")]
                            {()/* FPU */}
                        #[cfg(all(target_arch = "x86",target_feature = "sse"))]
                            {({()/* FPU */} ,_MM_GET_ROUNDING_MODE(_MM_ROUND_DOWN))}
                        #[cfg(all(target_arch = "x86_64",target_feature = "sse"))]
                            {_MM_SET_ROUNDING_MODE(_MM_ROUND_DOWN)}
                    }
                    #[inline]
                    unsafe fn to_nearest() {
                        #[cfg(target_feature = "-sse")]
                            {()/* FPU */}
                        #[cfg(all(target_arch = "x86",target_feature = "sse"))]
                            {({()/* FPU */} ,_MM_GET_ROUNDING_MODE(_MM_ROUND_NEAREST))}
                        #[cfg(all(target_arch = "x86_64",target_feature = "sse"))]
                            {_MM_SET_ROUNDING_MODE(_MM_ROUND_NEAREST)}
                    }
                    #[inline]
                    unsafe fn toward_zero() {
                        #[cfg(target_feature = "-sse")]
                            {()/* FPU */}
                        #[cfg(all(target_arch = "x86",target_feature = "sse"))]
                            {({()/* FPU */} ,{_MM_GET_ROUNDING_MODE(_MM_ROUND_TOWARD_ZERO)})}
                        #[cfg(all(target_arch = "x86_64",target_feature = "sse"))]
                            {_MM_SET_ROUNDING_MODE(_MM_ROUND_TOWARD_ZERO)}
                    }
                }
            )
        }
        impl_rmode!(f64);
        impl_rmode!(f32);
    }

    #[cfg(all(feature = "hwrm", any(target_arch = "x86", target_arch = "x86_64")))]
    pub use self::rmodelocal::*;

    #[derive(Debug)]
    pub struct RoundingModeControler<S: EditRoundingMode> {
        initial_state: S::RoundingState,
    }

    impl<S: EditRoundingMode> RoundingModeControler<S> {
        #[inline]
        pub fn rollback(&self) {
            unsafe { S::set_rounding_state(self.initial_state.clone()) }
        }
        #[inline(never)]
        pub unsafe fn upward_session<O, F>(&mut self, func: F) -> O
            where F: FnOnce() -> O
        {
            let state = S::current_rounding_state();
            S::upward();
            let r = func();
            S::set_rounding_state(state);
            r
        }
        #[inline(never)]
        pub unsafe fn downward_session<O, F>(&mut self, func: F) -> O
            where F: FnOnce() -> O
        {
            let state = S::current_rounding_state();
            S::downward();
            let r = func();
            S::set_rounding_state(state);
            r
        }
        #[inline(never)]
        pub unsafe fn to_nearest_session<O, F>(&mut self, func: F) -> O
            where F: FnOnce() -> O
        {
            let state = S::current_rounding_state();
            S::to_nearest();
            let r = func();
            S::set_rounding_state(state);
            r
        }
        #[inline(never)]
        pub unsafe fn toward_zero_session<O, F>(&mut self, func: F) -> O
            where F: FnOnce() -> O
        {
            let state = S::current_rounding_state();
            S::toward_zero();
            let r = func();
            S::set_rounding_state(state);
            r
        }
        #[inline(never)]
        pub unsafe fn upward_then<O, F>(&mut self, func: F) -> O
            where F: FnOnce() -> O
        {
            S::upward();
            func()
        }
        #[inline(never)]
        pub unsafe fn downward_then<O, F>(&mut self, func: F) -> O
            where F: FnOnce() -> O
        {
            S::downward();
            func()
        }
        #[inline(never)]
        pub unsafe fn to_nearest_then<O, F>(&mut self, func: F) -> O
            where F: FnOnce() -> O
        {
            S::to_nearest();
            func()
        }
        #[inline(never)]
        pub unsafe fn toward_zero_then<O, F>(&mut self, func: F) -> O
            where F: FnOnce() -> O
        {
            S::toward_zero();
            func()
        }
    }

    impl<S: EditRoundingMode> Drop for RoundingModeControler<S> {
        fn drop(&mut self) {
            unsafe {
                let _ = S::set_rounding_state(self.initial_state.clone());
            };
        }
    }

    #[cfg(feature = "hwrm")]
    #[test]
    fn rf64() {
        use roundops::rmode::*;
        let mut c = f64::rmode_controler().unwrap();
        let x = 0.1;
        let y = 10.7;

        let v = vec![1., 10., 3146136.314, 6136.1346, 5367.67467, -134562.4537];
        assert!(unsafe { c.upward_session(|| v.iter().sum::<f64>()) } >
                unsafe { c.downward_session(|| v.iter().sum::<f64>()) });
        println!("{} > {}",
                 unsafe { c.upward_session(|| v.iter().sum::<f64>()) },
                 unsafe { c.downward_session(|| v.iter().sum::<f64>()) });
        assert!(unsafe { c.upward_session(|| x + y) } > unsafe { c.downward_session(|| x + y) });
    }
}

pub trait RoundingMethod {
    type HostMethod: rmode::NativeRoundingMode;
    type Num;
}

pub trait RoundAdd: RoundingMethod {
    fn add_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn add_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundSub: RoundingMethod {
    fn sub_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn sub_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundMul: RoundingMethod {
    fn mul_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn mul_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundDiv: RoundingMethod {
    fn div_up(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
    fn div_down(lhs: Self::Num, rhs: Self::Num) -> Self::Num;
}

pub trait RoundSqrt: RoundMul {
    fn sqrt_up(n: Self::Num) -> Self::Num;
    fn sqrt_down(n: Self::Num) -> Self::Num;
}


pub trait RoundOps<T: Add + Sub + Mul + Div>
    : RoundAdd<Num = T> + RoundSub<Num = T> + RoundMul<Num = T> + RoundDiv<Num = T>
    {
}

impl<S, T> RoundOps<T> for S
    where S: RoundAdd<Num = T> + RoundSub<Num = T> + RoundMul<Num = T> + RoundDiv<Num = T>,
          T: Add + Sub + Mul + Div
{
}

pub mod direction {
    pub trait Direction: Clone {
        type Inversed: Direction;
    }

    #[derive(Clone)]
    pub enum Upward {}

    impl Direction for Upward {
        type Inversed = Downward;
    }

    #[derive(Clone)]
    pub enum Downward {}

    impl Direction for Downward {
        type Inversed = Upward;
    }
}

#[derive(Clone)]
pub struct RoundedNum<Dir: direction::Direction, Num, Method>(Num, PhantomData<(Dir, Method)>);
impl<Dir: direction::Direction, Num, Method> RoundedNum<Dir, Num, Method> {
    #[inline(always)]
    pub fn new(num: Num) -> Self {
        RoundedNum(num, PhantomData)
    }
    #[inline(always)]
    pub fn extract(self) -> Num {
        self.0
    }
}

impl<D: direction::Direction, N: Neg<Output = N>, M> Neg for RoundedNum<D, N, M> {
    type Output = RoundedNum<D, N, M>;
    fn neg(self) -> RoundedNum<D, N, M> {
        RoundedNum(-self.0, PhantomData)
    }
}

macro_rules! impl_rnum_op {
    ($dir:ty, $op:ident, $rop:ident, $fn:ident, $rfn:ident) => (
        impl<N: $op, M: $rop<Num = N>> $op<RoundedNum<$dir, N, M>>
            for RoundedNum<$dir, N, M> {
            type Output = RoundedNum<$dir, N, M>;
            #[inline(always)]
            fn $fn(self, rhs: RoundedNum<$dir, N, M>) -> RoundedNum<$dir, N, M> {
                RoundedNum(M::$rfn(self.0, rhs.0), PhantomData)
            }
        }
    )
}

macro_rules! impl_rnum_sqrt {
    ($dir:ty, $rfn:ident) => (
        impl<N: Mul, M: RoundSqrt + RoundMul<Num = N>> RoundedNum<$dir, N, M> {
            #[inline(always)]
            pub fn sqrt(self) -> RoundedNum<$dir, N, M> {
                RoundedNum(M::$rfn(self.0), PhantomData)
            }
        }
    )
}

impl_rnum_op!(direction::Upward, Add, RoundAdd, add, add_up);
impl_rnum_op!(direction::Upward, Sub, RoundSub, sub, sub_up);
impl_rnum_op!(direction::Upward, Mul, RoundMul, mul, mul_up);
impl_rnum_op!(direction::Upward, Div, RoundDiv, div, div_up);
impl_rnum_op!(direction::Downward, Add, RoundAdd, add, add_down);
impl_rnum_op!(direction::Downward, Sub, RoundSub, sub, sub_down);
impl_rnum_op!(direction::Downward, Mul, RoundMul, mul, mul_down);
impl_rnum_op!(direction::Downward, Div, RoundDiv, div, div_down);
impl_rnum_sqrt!(direction::Upward, sqrt_up);
impl_rnum_sqrt!(direction::Downward, sqrt_down);

#[macro_export]
macro_rules! rnum_init {
    (<$dir:path,$n:ty,$m:ty>,($numf:expr $(, $num:expr)+)) => (
        (RoundedNum::<$dir, $n, $m>::new($numf) $(, RoundedNum::<$dir, $n, $m>::new($num))+)
    )
}

pub trait RoundedSession: Clone + Sized {
    type Num: Clone;
    #[inline]
    fn calc_with<Dir: direction::Direction>(input: Vec<Self::Num>,
                                            func: fn(Vec<RoundedNum<Dir, Self::Num, Self>>)
                                                     -> Vec<RoundedNum<Dir, Self::Num, Self>>)
                                            -> Vec<Self::Num> {
        func(input
                 .into_iter()
                 .map(|num| RoundedNum(num, PhantomData))
                 .collect::<Vec<_>>())
                .into_iter()
                .map(|e| e.0)
                .collect::<Vec<_>>()
    }
}
