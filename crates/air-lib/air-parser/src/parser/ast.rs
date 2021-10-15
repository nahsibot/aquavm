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

mod impls;
mod traits;

pub use crate::parser::lexer::AstVariable;
pub use crate::parser::lexer::LastErrorPath;
pub use crate::parser::lexer::Number;

use serde::Deserialize;
use serde::Serialize;

use std::rc::Rc;

#[allow(clippy::large_enum_variant)] // for Null and Error variants
#[derive(Serialize, Debug, PartialEq)]
pub enum Instruction<'i> {
    Null(Null),
    Call(Call<'i>),
    Ap(Ap<'i>),
    Seq(Seq<'i>),
    Par(Par<'i>),
    Xor(Xor<'i>),
    Match(Match<'i>),
    MisMatch(MisMatch<'i>),
    FoldScalar(FoldScalar<'i>),
    FoldStream(FoldStream<'i>),
    Next(Next<'i>),
    Error,
}

#[derive(Serialize, Debug, PartialEq)]
pub enum PeerPart<'i> {
    PeerPk(CallInstrValue<'i>),
    PeerPkWithServiceId(CallInstrValue<'i>, CallInstrValue<'i>),
}

#[derive(Serialize, Debug, PartialEq)]
pub enum FunctionPart<'i> {
    FuncName(CallInstrValue<'i>),
    ServiceIdWithFuncName(CallInstrValue<'i>, CallInstrValue<'i>),
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Call<'i> {
    pub peer_part: PeerPart<'i>,
    pub function_part: FunctionPart<'i>,
    pub args: Rc<Vec<CallInstrArgValue<'i>>>,
    pub output: CallOutputValue<'i>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum ApArgument<'i> {
    ScalarVariable(&'i str),
    JsonPath(JsonPath<'i>),
    Number(Number),
    Boolean(bool),
    Literal(&'i str),
    EmptyArray,
    LastError(LastErrorPath),
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Ap<'i> {
    pub argument: ApArgument<'i>,
    pub result: AstVariable<'i>,
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CallInstrValue<'i> {
    InitPeerId,
    Literal(&'i str),
    Variable(AstVariable<'i>),
    JsonPath(JsonPath<'i>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum CallInstrArgValue<'i> {
    InitPeerId,
    LastError(LastErrorPath),
    Literal(&'i str),
    Number(Number),
    Boolean(bool),
    EmptyArray, // only empty arrays are allowed now
    Variable(AstVariable<'i>),
    JsonPath(JsonPath<'i>),
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum IterableScalarValue<'i> {
    ScalarVariable(&'i str),
    JsonPath {
        scalar_name: &'i str,
        path: &'i str,
        should_flatten: bool,
    },
}

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub enum MatchableValue<'i> {
    InitPeerId,
    Literal(&'i str),
    Number(Number),
    Boolean(bool),
    EmptyArray,
    Variable(AstVariable<'i>),
    JsonPath(JsonPath<'i>),
}

#[derive(Serialize, Debug, PartialEq, Clone)]
pub enum CallOutputValue<'i> {
    Variable(AstVariable<'i>),
    None,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Seq<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

#[derive(Serialize, Debug, PartialEq)]
pub struct Par<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

#[derive(Serialize, Debug, PartialEq)]
pub struct Xor<'i>(pub Box<Instruction<'i>>, pub Box<Instruction<'i>>);

#[derive(Serialize, Debug, PartialEq)]
pub struct Match<'i> {
    pub left_value: MatchableValue<'i>,
    pub right_value: MatchableValue<'i>,
    pub instruction: Box<Instruction<'i>>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct MisMatch<'i> {
    pub left_value: MatchableValue<'i>,
    pub right_value: MatchableValue<'i>,
    pub instruction: Box<Instruction<'i>>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct FoldScalar<'i> {
    pub iterable: IterableScalarValue<'i>,
    pub iterator: &'i str,
    pub instruction: Rc<Instruction<'i>>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct FoldStream<'i> {
    pub stream_name: &'i str,
    pub iterator: &'i str,
    pub instruction: Rc<Instruction<'i>>,
}

#[derive(Serialize, Debug, PartialEq)]
pub struct Next<'i>(pub &'i str);

#[derive(Serialize, Debug, PartialEq)]
pub struct Null;

#[derive(Debug, PartialEq, Clone, Serialize, Deserialize)]
pub struct JsonPath<'i> {
    pub variable: AstVariable<'i>,
    pub path: &'i str,
    pub should_flatten: bool,
}