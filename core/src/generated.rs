#![allow(non_snake_case)] // because of the generated bindings.

use crate::sys;
use crate::get_api;
use super::*;

use std::sync::{Once, ONCE_INIT};
use std::ops::*;
use libc;

include!(concat!(env!("OUT_DIR"), "/core_types.rs"));
