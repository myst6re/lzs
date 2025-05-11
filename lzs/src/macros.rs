#[cfg(not(feature = "safe"))]
macro_rules! get {
    ($slice:ident,$offset:expr) => {
        *unsafe { $slice.get_unchecked($offset) }
    };
}
#[cfg(not(feature = "safe"))]
macro_rules! set {
    ($slice:ident,$offset:expr,$value:expr) => {
        *unsafe { $slice.get_unchecked_mut($offset) } = $value;
    };
}

#[cfg(feature = "safe")]
macro_rules! get {
    ($slice:ident,$offset:expr) => {
        $slice[$offset]
    };
}
#[cfg(feature = "safe")]
macro_rules! set {
    ($slice:ident,$offset:expr,$value:expr) => {
        $slice[$offset] = $value
    };
}

pub(crate) use get;
pub(crate) use set;
