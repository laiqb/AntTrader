# Ant-system

[![build](https://github.com/nautechsystems/ant_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/ant_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/Ant-system)](https://docs.rs/Ant-system/latest/Ant-system/)
[![crates.io version](https://img.shields.io/crates/v/Ant-system.svg)](https://crates.io/crates/Ant-system)
![license](https://img.shields.io/github/license/nautechsystems/ant_trader?color=blue)

System-level components and orchestration for [AntTrader](http://nautilustrader.io).

The `Ant-system` crate provides the core system architecture for orchestrating trading systems,
including the kernel that manages all engines, configuration management,
and system-level factories for creating components:

- `AntKernel` - Core system orchestrator managing engines and components.
- `AntKernelConfig` - Configuration for kernel initialization.
- System builders and factories for component creation.

## Platform

[AntTrader](http://nautilustrader.io) is an open-source, high-performance, production-grade
algorithmic trading platform, providing quantitative traders with the ability to backtest
portfolios of automated trading strategies on historical data with an event-driven engine,
and also deploy those same strategies live, with no code changes.

AntTrader's design, architecture, and implementation philosophy prioritizes software correctness and safety at the
highest level, with the aim of supporting mission-critical, trading system backtesting and live deployment workloads.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation,
depending on the intended use case, i.e. whether to provide Python bindings
for the [ant_trader](https://pypi.org/project/ant_trader) Python package,
or as part of a Rust only build.

- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
- `extension-module`: Builds the crate as a Python extension module.

## Documentation

See [the docs](https://docs.rs/Ant-system) for more detailed usage.

## License

The source code for AntTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).
Contributions to the project are welcome and require the completion of a standard [Contributor License Agreement (CLA)](https://github.com/nautechsystems/ant_trader/blob/develop/CLA.md).

---

AntTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://nautilustrader.io>.

<img src="https://nautilustrader.io/Ant-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. All rights reserved.</span>
