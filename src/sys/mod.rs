pub(crate) mod io_default;
#[cfg(feature = "unix-iovec")]
mod io_unix_iovec;

pub(crate) mod io {
    #[cfg(not(feature = "unix-iovec"))]
    pub(crate) use super::io_default::*;

    #[cfg(feature = "unix-iovec")]
    pub(crate) use super::io_unix_iovec::*;
}
