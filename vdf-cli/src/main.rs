// Copyright 2018 Chia Network Inc and Block Notary Inc
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//   http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// and limitations under the License.#![forbid(unsafe_code)]
#![forbid(warnings)]
extern crate vdf;
#[macro_use]
extern crate clap;
fn main() {
    let _matches = clap_app!(vdf =>
        (version: "0.1.0")
        (author: "Block Notary <poa.networks>")
        (about: "CLI to Verifiable Delay Functions")
    ).get_matches();
}
