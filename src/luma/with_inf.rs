use std::cmp::Ordering;
use std::ops;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WithInf<T> {
    Finite(T),
    PosInf,
    NegInf,
}

impl<T> WithInf<T> {
    pub fn unwrap_finite(self) -> T {
        match self {
            Self::Finite(v) => v,
            _ => panic!("called `WithInf<T>::unwrap_finite()` on a infinite value"),
        }
    }
}

impl<T: Default> Default for WithInf<T> {
    fn default() -> Self {
        Self::Finite(T::default())
    }
}

impl<T> From<T> for WithInf<T> {
    fn from(v: T) -> Self {
        Self::Finite(v)
    }
}

// TODO: move to somewhere
pub trait PosInf {
    fn pos_inf() -> Self;
}

pub trait NegInf {
    fn neg_inf() -> Self;
}

impl<T> PosInf for WithInf<T> {
    fn pos_inf() -> Self {
        Self::PosInf
    }
}

impl<T> NegInf for WithInf<T> {
    fn neg_inf() -> Self {
        Self::NegInf
    }
}

impl PosInf for f32 {
    fn pos_inf() -> Self {
        std::f32::INFINITY
    }
}

impl NegInf for f32 {
    fn neg_inf() -> Self {
        std::f32::NEG_INFINITY
    }
}

impl PosInf for f64 {
    fn pos_inf() -> Self {
        std::f64::INFINITY
    }
}

impl NegInf for f64 {
    fn neg_inf() -> Self {
        std::f64::NEG_INFINITY
    }
}

impl<T: PartialOrd> PartialOrd for WithInf<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        match self {
            WithInf::Finite(vs) => match other {
                WithInf::Finite(vo) => vs.partial_cmp(vo),
                WithInf::PosInf => Some(Ordering::Less),
                WithInf::NegInf => Some(Ordering::Greater),
            },
            WithInf::PosInf => match other {
                WithInf::Finite(..) => Some(Ordering::Greater),
                WithInf::PosInf => Some(Ordering::Equal),
                WithInf::NegInf => Some(Ordering::Greater),
            },
            WithInf::NegInf => match other {
                WithInf::Finite(..) => Some(Ordering::Less),
                WithInf::PosInf => Some(Ordering::Less),
                WithInf::NegInf => Some(Ordering::Equal),
            },
        }
    }
}

impl<T: Ord> Ord for WithInf<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl<T: ops::Add<T, Output = T>, Rhs: Into<WithInf<T>>> ops::Add<Rhs> for WithInf<T> {
    type Output = WithInf<T>;
    fn add(self, rhs: Rhs) -> Self::Output {
        let rhs_v = Into::<WithInf<T>>::into(rhs);
        match self {
            Self::Finite(vl) => {
                return match rhs_v {
                    Self::Finite(vr) => Self::Finite(vl + vr),
                    _ => rhs_v,
                };
            }
            Self::PosInf => match rhs_v {
                Self::NegInf => {}
                _ => return self,
            },
            Self::NegInf => match rhs_v {
                Self::PosInf => {}
                _ => return self,
            },
        }
        panic!("operating undefined addition on with-infinity values");
    }
}
