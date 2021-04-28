//!  Unsafe bindings to the [Bosch BSEC library](https://www.bosch-sensortec.com/software-tools/software/bsec/).
//!
//! ## Important license information
//!
//! The BSEC library is proprietary. Thus, it cannot be included in this crate and
//! its documentation need to be obtained separately. You are responsible for
//! adhering to that license in your products despite the source code of this crate
//! to generate the necessary bindings is published under a permissive license.
//!
//! Note that the source code of this crate does not contain any of the BSEC source
//! code, API declartions, or documentation.
//!
//! * [BSEC website to obtain your copy](https://www.bosch-sensortec.com/software-tools/software/bsec/)
//! * [BSEC license terms at the time of writing](https://www.bosch-sensortec.com/media/boschsensortec/downloads/bsec/2017-07-17_clickthrough_license_terms_environmentalib_sw_clean.pdf)
//!
//!
//! ## Usage
//!
//! To be able to use this crate it needs to know where to find the BSEC header
//! files and library on your system. These paths are provided as the configuration
//! options `bsec_include_path` and `bsec_library_path` to the Rust compiler.
//!
//! You can do this by creating a `.cargo/config` file in your crate with the
//! following content (adjust the paths accordingly):
//!
//! ```toml
//! [build]
//! rustflags = [
//!     '--cfg', 'bsec_include_path="/path/to/BSEC_1.4.8.0_Generic_Release/algo/normal_version/inc"',
//!     '--cfg', 'bsec_library_path="/path/to/BSEC_1.4.8.0_Generic_Release/algo/normal_version/bin/target-arch"',
//! ]
//! ```
//!
//! ## Building local BSEC documentation
//!
//! If you build the documentation locally with `cargo doc`, after the above setup,
//! BSEC documentation extracted from the header files will be included.

#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(non_upper_case_globals)]

include!(concat!(env!("OUT_DIR"), "/bsec.rs"));
