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

use super::CallEvidencePath;

use serde::Deserialize;
use serde::Serialize;

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub(crate) struct CallEvidenceCtx {
    pub(crate) current_path: CallEvidencePath,
    pub(crate) current_subtree_elements_count: usize,
    // TODO: consider change it to Vec for optimization
    pub(crate) new_path: CallEvidencePath,
}

impl CallEvidenceCtx {
    pub fn new(current_path: CallEvidencePath) -> Self {
        let current_subtree_elements_count = current_path.len();
        Self {
            current_path,
            current_subtree_elements_count,
            new_path: CallEvidencePath::new(),
        }
    }
}
