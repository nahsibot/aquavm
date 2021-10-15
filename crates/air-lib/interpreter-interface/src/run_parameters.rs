/*
 * Copyright 2021 Fluence Labs Limited
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

use marine_rs_sdk::marine;
use serde::Deserialize;
use serde::Serialize;

/// Parameters that a host side should pass to an interpreter and that necessary for execution.
#[marine]
#[derive(Debug, Clone, Eq, PartialEq, Serialize, Deserialize)]
pub struct RunParameters {
    /// Peer id of a peer that start this particle.
    pub init_peer_id: String,

    /// Peer id of a current peer.
    pub current_peer_id: String,
}