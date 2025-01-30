pub use laststraw_core::*;
pub use laststraw_core_macro::*;
// This Framework is split between two crates, due to proc macro, needs.

use minifb::{CursorStyle, KeyRepeat};
use minifb::{Key, Window, WindowOptions};
use quote::quote;
use rusttype::{point, Font, Scale};
use std::{collections::HashMap, f64::RADIX};
use syn::{parse_macro_input, Block};
