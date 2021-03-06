// Copyright (c) 2018 FaultyRAM
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your option. This file may not be
// copied, modified, or distributed except according to those terms.

//! A simple multimedia decoding library.

#![forbid(warnings)]
#![forbid(future_incompatible)]
#![forbid(unused)]
#![forbid(box_pointers)]
#![forbid(elided_lifetime_in_path)]
#![forbid(missing_copy_implementations)]
#![forbid(missing_debug_implementations)]
#![forbid(missing_docs)]
#![forbid(single_use_lifetime)]
#![forbid(trivial_casts)]
#![forbid(trivial_numeric_casts)]
#![forbid(unreachable_pub)]
#![forbid(unused_import_braces)]
#![deny(unused_qualifications)]
#![forbid(unused_results)]
#![forbid(variant_size_differences)]
#![cfg_attr(feature = "clippy", feature(plugin))]
#![cfg_attr(feature = "clippy", plugin(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy))]
#![cfg_attr(feature = "clippy", forbid(clippy_complexity))]
#![cfg_attr(feature = "clippy", forbid(clippy_correctness))]
#![cfg_attr(feature = "clippy", forbid(clippy_pedantic))]
#![cfg_attr(feature = "clippy", forbid(clippy_perf))]
#![cfg_attr(feature = "clippy", forbid(clippy_style))]
