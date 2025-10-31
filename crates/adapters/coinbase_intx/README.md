# ant-coinbase-intx

[![build](https://github.com/nautechsystems/ant_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/ant_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/ant-coinbase-intx)](https://docs.rs/ant-coinbase-intx/latest/ant-coinbase-intx/)
[![crates.io version](https://img.shields.io/crates/v/ant-coinbase-intx.svg)](https://crates.io/crates/ant-coinbase-intx)
![license](https://img.shields.io/github/license/nautechsystems/ant_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/antTrader)

[antTrader](http://anttrader.io) adapter for [Coinbase International](https://www.coinbase.com/en/international-exchange) exchange.

The `ant-coinbase-intx` crate provides integration with the Coinbase International API for
institutional trading on their derivatives exchange.

## Platform

[antTrader](http://anttrader.io) is an open-source, high-performance, production-grade
algorithmic trading platform, providing quantitative traders with the ability to backtest
portfolios of automated trading strategies on historical data with an event-driven engine,
and also deploy those same strategies live, with no code changes.

antTrader's design, architecture, and implementation philosophy prioritizes software correctness and safety at the
highest level, with the aim of supporting mission-critical, trading system backtesting and live deployment workloads.

## Feature Flags

This crate provides feature flags to control source code inclusion during compilation:

- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).
- `extension-module`: Builds as a Python extension module (used with `python`).

## Documentation

See [the docs](https://docs.rs/ant-coinbase-intx) for more detailed usage.

## License

The source code for antTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).
Contributions to the project are welcome and require the completion of a standard [Contributor License Agreement (CLA)](https://github.com/nautechsystems/ant_trader/blob/develop/CLA.md).

---

antTrader™ is developed and maintained by ANT Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://anttrader.io>.

<img src="https://anttrader.io/ant-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 ANT Systems Pty Ltd. All rights reserved.</span>
