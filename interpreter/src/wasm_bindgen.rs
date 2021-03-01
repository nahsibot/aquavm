/*
 * Copyright 2020 Fluence Labs Limited
 *
 * Licensed under the Apache License, Version 2.0 (the "License");
 * you may not use this file except in compliance with the License.
 * You may obtain a copy of the License at
 *
 *     http://www.apache.org/licenses/LICENSE-2.0
 *
 * Unless required by applicable law or agreed to in writing, software
 * distributed under the License is distributed on an "AS IS" BASIS,
 * WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
 * See the License for the specific language governing permissions and
 * limitations under the License.
 */

#![allow(unused_attributes)]
#![warn(rust_2018_idioms)]
#![deny(
    dead_code,
    nonstandard_style,
    unused_imports,
    unused_mut,
    unused_variables,
    unused_unsafe,
    unreachable_patterns
)]

mod ast;
mod logger;

use interpreter_lib::execute_aqua;

use log::LevelFilter;
use wasm_bindgen::prelude::*;

pub const DEFAULT_LOG_LEVEL: LevelFilter = LevelFilter::Trace;

#[wasm_bindgen(start)]
pub fn main() {
    logger::init_logger();
}

#[wasm_bindgen]
pub fn invoke(init_peer_id: String, aqua: String, prev_data: Vec<u8>, data: Vec<u8>, log_level: &str) -> String {
    use std::str::FromStr;

    let log_level = log::LevelFilter::from_str(log_level).unwrap_or(DEFAULT_LOG_LEVEL);
    log::set_max_level(log_level);

    let outcome = execute_aqua(init_peer_id, aqua, prev_data, data);
    serde_json::to_string(&outcome).expect("Cannot parse InterpreterOutcome")
}

#[wasm_bindgen]
pub fn ast(script: String) -> String {
    ast::ast(script)
}
