#[cfg(feature = "mpi-stubs")]
// ensure that mpi-stubs-sys is linked
extern crate mpi_stubs_sys as _;

// ensure that std-compat-sys is linked
extern crate std_compat_sys as _;
