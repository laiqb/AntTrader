// -------------------------------------------------------------------------------------------------
//  Copyright (C) 2015-2025 ANT Systems Pty Ltd. All rights reserved.
//
//
//  Licensed under the GNU Lesser General Public License Version 3.0 (the "License");
//  You may not use this file except in compliance with the License.
//  You may obtain a copy of the License at https://www.gnu.org/licenses/lgpl-3.0.en.html
//
//  Unless required by applicable law or agreed to in writing, software
//  distributed under the License is distributed on an "AS IS" BASIS,
//  WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
//  See the License for the specific language governing permissions and
//  limitations under the License.
// -------------------------------------------------------------------------------------------------

//! Test utilities and data management for [antTrader](http://anttrader.io).
//!
//! The `ant-testkit` crate provides comprehensive testing utilities including test data management,
//! file handling, and common testing patterns. This crate supports robust testing workflows
//! across the entire antTrader ecosystem with automated data downloads and validation:
//!
//! - **Test data management**: Automated downloading and caching of test datasets.
//! - **File utilities**: File integrity verification with SHA-256 checksums.
//! - **Path resolution**: Platform-agnostic test data path management.
//! - **Precision handling**: Support for both 64-bit and 128-bit precision test data.
//! - **Common patterns**: Reusable test utilities and helper functions.
//!
//! # Platform
//!
//! [antTrader](http://anttrader.io) is an open-source, high-performance, production-grade
//! algorithmic trading platform, providing quantitative traders with the ability to backtest
//! portfolios of automated trading strategies on historical data with an event-driven engine,
//! and also deploy those same strategies live, with no code changes.
//!
//! antTrader's design, architecture, and implementation philosophy prioritizes software correctness and safety at the
//! highest level, with the aim of supporting mission-critical, trading system backtesting and live deployment workloads.
//!
//! # Feature flags
//!
//! This crate provides feature flags to control source code inclusion during compilation,
//! depending on the intended use case, i.e. whether to provide Python bindings
//! for the [ant_trader](https://pypi.org/project/ant_trader) Python package,
//! or as part of a Rust only build.
//!
//! - `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
//! - `high-precision`: Enables [high-precision mode](https://anttrader.io/docs/nightly/getting_started/installation#precision-mode) to use 128-bit value types.
//! - `extension-module`: Builds the crate as a Python extension module.

#![warn(rustc::all)]
#![deny(unsafe_code)]
#![deny(nonstandard_style)]
#![deny(missing_debug_implementations)]
#![deny(clippy::missing_errors_doc)]
#![deny(clippy::missing_panics_doc)]
#![deny(rustdoc::broken_intra_doc_links)]

pub mod common;
pub mod files;

#[cfg(feature = "python")]
pub mod python;
