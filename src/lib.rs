#![doc = include_str!("../README.md")]

#[cfg(feature = "clpsolver")]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));