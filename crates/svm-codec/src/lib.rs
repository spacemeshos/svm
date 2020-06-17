#![allow(missing_docs)]
#![allow(unused)]
#![allow(dead_code)]
#![allow(unreachable_code)]
#![feature(vec_into_raw_parts)]

#[macro_use]
mod nibble;

mod api;
mod app;
mod error;
mod field;
mod func_args;
mod func_buf;
mod gas;
mod helpers;
mod host_ctx;
mod template;
mod traits;
mod transaction;
mod varuint14;
mod version;
mod wasm;

use app::{decode_spawn_app, encode_spawn_app, DefaultAppDeserializer, DefaultAppSerializer};
use error::ParseError;
use field::Field;
use func_args::{decode_func_args, decode_func_rets, encode_func_args, encode_func_rets};
use func_buf::{decode_func_buf, encode_func_buf};
use gas::{decode_gas_used, encode_gas_used};
use nibble::{concat_nibbles, Nibble, NibbleIter, NibbleWriter};
use template::{decode_deploy_template, encode_deploy_template};
use template::{DefaultAppTemplateDeserializer, DefaultAppTemplateSerializer};
use transaction::{decode_exec_app, encode_exec_app};
use varuint14::{decode_varuint14, encode_varuint14};
use version::{decode_version, encode_version};
