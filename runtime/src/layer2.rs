// Modern, minimalistic & standard-compliant cold wallet library.
//
// SPDX-License-Identifier: Apache-2.0
//
// Written in 2020-2023 by
//     Dr Maxim Orlovsky <orlovsky@lnp-bp.org>
//
// Copyright (C) 2020-2023 LNP/BP Standards Association. All rights reserved.
// Copyright (C) 2020-2023 Dr Maxim Orlovsky. All rights reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

pub trait Layer2 {
    type Descr: Layer2Descriptor;
    type Data: Layer2Data;
    type Cache: Layer2Cache;
}

pub trait Layer2Descriptor {}

pub trait Layer2Data: Default {}

pub trait Layer2Cache: Default {}

impl Layer2 for () {
    type Descr = ();
    type Data = ();
    type Cache = ();
}

impl Layer2Descriptor for () {}

impl Layer2Data for () {}

impl Layer2Cache for () {}
