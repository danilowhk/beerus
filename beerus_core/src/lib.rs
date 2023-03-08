#![cfg_attr(not(feature = "std"), no_std)]
#![allow(incomplete_features)]
#![feature(specialization)]
#![feature(async_fn_in_trait)]

#[macro_use]
extern crate core;

#[macro_use]
extern crate alloc;

pub mod config;
pub mod ethers_helper;
pub mod lightclient;
pub mod starknet_helper;
