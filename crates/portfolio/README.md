# ant-portfolio

[![build](https://github.com/nautechsystems/ant_trader/actions/workflows/build.yml/badge.svg?branch=master)](https://github.com/nautechsystems/ant_trader/actions/workflows/build.yml)
[![Documentation](https://img.shields.io/docsrs/ant-portfolio)](https://docs.rs/ant-portfolio/latest/ant-portfolio/)
[![crates.io version](https://img.shields.io/crates/v/ant-portfolio.svg)](https://crates.io/crates/ant-portfolio)
![license](https://img.shields.io/github/license/nautechsystems/ant_trader?color=blue)
[![Discord](https://img.shields.io/badge/Discord-%235865F2.svg?logo=discord&logoColor=white)](https://discord.gg/antTrader)

Portfolio management and risk analysis for [antTrader](http://anttrader.io).

The `ant-portfolio` crate provides comprehensive portfolio management capabilities including
real-time position tracking, performance calculations, and risk management. This includes
sophisticated portfolio analytics and multi-currency support:

- **Portfolio tracking**: Real-time portfolio state management with position and balance monitoring.
- **Account management**: Support for cash and margin accounts across multiple venues.
- **Performance calculations**: Real-time unrealized PnL, realized PnL, and mark-to-market valuations.
- **Risk management**: Initial margin calculations, maintenance margin tracking, and exposure monitoring.
- **Multi-currency support**: Currency conversion and cross-currency risk exposure analysis.
- **Configuration options**: Flexible settings for price types, currency conversion, and portfolio behavior.

The crate handles complex portfolio scenarios including multi-venue trading, currency conversions,
and sophisticated margin calculations for both live trading and backtesting environments.

## Platform

[antTrader](http://anttrader.io) is an open-source, high-performance, production-grade
algorithmic trading platform, providing quantitative traders with the ability to backtest
portfolios of automated trading strategies on historical data with an event-driven engine,
and also deploy those same strategies live, with no code changes.

antTrader's design, architecture, and implementation philosophy prioritizes software correctness and safety at the
highest level, with the aim of supporting mission-critical, trading system backtesting and live deployment workloads.

## Feature flags

This crate provides feature flags to control source code inclusion during compilation,
depending on the intended use case, i.e. whether to provide Python bindings
for the [ant_trader](https://pypi.org/project/ant_trader) Python package,
or as part of a Rust only build.

- `python`: Enables Python bindings from [PyO3](https://pyo3.rs).

## Documentation

See [the docs](https://docs.rs/ant-portfolio) for more detailed usage.

## License

The source code for antTrader is available on GitHub under the [GNU Lesser General Public License v3.0](https://www.gnu.org/licenses/lgpl-3.0.en.html).
Contributions to the project are welcome and require the completion of a standard [Contributor License Agreement (CLA)](https://github.com/nautechsystems/ant_trader/blob/develop/CLA.md).

---

antTrader™ is developed and maintained by Nautech Systems, a technology
company specializing in the development of high-performance trading systems.
For more information, visit <https://anttrader.io>.

<img src="https://anttrader.io/ant-logo-white.png" alt="logo" width="400" height="auto"/>

<span style="font-size: 0.8em; color: #999;">© 2015-2025 Nautech Systems Pty Ltd. All rights reserved.</span>
