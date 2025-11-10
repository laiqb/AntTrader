use statrs::distribution::{ContinuousCDF, Normal};

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "ant_trader.core.ant_pyo3.option")
)]
pub struct BlackScholesGreeksResult {
    pub price: f64,
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
}

thread_local! {
    static STANDARD_NORMAL: Normal = Normal::new(0.0, 1.0)
        .expect("创建标准正态分布失败！参数合法，此错误不应发生");
}

/// EFT 股票 股指 BS公式， 计算出价格以及对应的希腊字母
///```rust
/// dS_t = S_t * (b * dt + sigma * dW_t) (stock)
/// dC_t = r * C_t * dt (cash numeraire)
/// under_type: 底层资产类型，1[股票、etf、股指期货] 2[商品期货]
/// s: 标的价格，期货价格，例如ETF价格
/// r: 无风险利率
/// b: 标的资产的净持有成本率, b = r
/// sigma: 隐含波动率,
/// is_call: true[call option], false[put option]，
/// k: 执行价，
/// t: 剩余时间，转换成分钟计算更加准确
/// multiplier: 合约乘数， 比如300etf传入10000
/// ```
#[allow(clippy::too_many_arguments)]
pub fn black_scholes_greeks(
    under_type: i32,
    s: f64,
    r: f64,
    b: f64,
    sigma: f64,
    is_call: bool,
    k: f64,
    t: f64,
    multiplier: f64,
) -> BlackScholesGreeksResult {
    // let a = BlackScholesGreeksResult {
    //     price: 1.0,
    //     delta: 1.0,
    //     gamma: 1.0,
    //     vega: 1.0,
    //     theta: 1.0,
    // };
    // return a;

    match under_type {
        1 => {
            let phi = if is_call { 1.0 } else { -1.0 };
            let scaled_vol = sigma * t.sqrt();
            let d1 = ((s / k).ln() + (b + 0.5 * sigma.powi(2)) * t) / scaled_vol;
            let d2 = d1 - scaled_vol;

            let (cdf_phi_d1, cdf_phi_d2, dist_d1) = STANDARD_NORMAL.with(|normal_dist| {
                (
                    normal_dist.cdf(phi * d1),
                    normal_dist.cdf(phi * d2),
                    normal_dist.cdf(d1),
                )
            });

            let df = ((b - r) * t).exp();
            let s_t = s * df;
            let k_t = k * (-r * t).exp();

            let price = multiplier * phi * (s_t * cdf_phi_d1 - k_t * cdf_phi_d2);
            let delta = multiplier * phi * df * cdf_phi_d1;
            let gamma = multiplier * df * dist_d1 / (s * scaled_vol);
            let vega = multiplier * s_t * t.sqrt() * dist_d1 * 0.01; // in absolute percent change
            let theta = multiplier
                * (s_t * (-dist_d1 * sigma / (2.0 * t.sqrt()) - phi * (b - r) * cdf_phi_d1)
                - phi * r * k_t * cdf_phi_d2)
                * 0.0027378507871321013; // 1 / 365.25 in change per calendar day

            BlackScholesGreeksResult {
                price,
                delta,
                gamma,
                vega,
                theta,
            }
        }
        2 => {
            // 期货期权：Black76 模型计算理论价格
            let phi = if is_call { 1.0 } else { -1.0 }; // call put 标识
            let scaled_vol = sigma * t.sqrt(); // 经时间调整的波动率（σ√t）
            let d1 = ((s / k).ln() + (0.5 * sigma.powi(2)) * t) / scaled_vol;
            let d2 = d1 - scaled_vol;

            let (cdf_phi_d1, cdf_phi_d2, dist_d1) = STANDARD_NORMAL.with(|normal_dist| {
                (
                    normal_dist.cdf(phi * d1),
                    normal_dist.cdf(phi * d2),
                    normal_dist.cdf(d1),
                )
            });

            let df = (-r * t).exp(); // 3. 折现因子（仅执行价需要无风险利率折现）

            let price = multiplier * phi * df * (s * cdf_phi_d1 - k * cdf_phi_d2);
            let delta = multiplier * phi * df * cdf_phi_d1;
            let gamma = multiplier * df * dist_d1 / (s * scaled_vol);
            let vega = multiplier * df * t.sqrt() * dist_d1 * 0.01; // in absolute percent change
            let theta = multiplier
                * (df * s * (-dist_d1 * sigma / (2.0 * t.sqrt()) + phi * r * cdf_phi_d1)
                - phi * r * df * k * cdf_phi_d2)
                * 0.0027378507871321013; // 1 / 365.25 in change per calendar day

            BlackScholesGreeksResult {
                price,
                delta,
                gamma,
                vega,
                theta,
            }
        }
        _ => panic!("Unsupported under_type {}", under_type),
    }
}

#[repr(C)]
#[derive(Clone, Copy, Debug, PartialEq, PartialOrd)]
#[cfg_attr(
    feature = "python",
    pyo3::pyclass(module = "ant_trader.core.ant_pyo3.option")
)]
pub struct ImplyVolAndGreeksResult {
    pub vol: f64,
    pub price: f64,
    pub delta: f64,
    pub gamma: f64,
    pub vega: f64,
    pub theta: f64,
}

/// 二分法计算隐含波动率
///```rust
/// under_type: 底层资产类型，1[股票、etf、股指期货] 2[商品期货]
/// price: 合约价格
/// s: 标的价格，例如ETF价格
/// r: 无风险利率
/// b: 标的资产的净持有成本率, b = r
/// sigma: 隐含波动率,
/// is_call: true[call option], false[put option]，
/// k: 执行价，
/// t: 剩余时间，转换成分钟计算更加准确
/// multiplier: 合约乘数， 比如300etf传入10000
/// ```

pub fn imply_vol(under_type: i32,
                 price: f64,
                 s: f64,
                 r: f64,
                 b: f64,
                 is_call: bool,
                 k: f64,
                 t: f64,
                 multiplier: f64) -> ImplyVolAndGreeksResult {

    let calculate_theoretical_price = |sigma: f64| -> f64 {
        black_scholes_greeks(under_type, s,r,b, sigma, is_call, k, t, multiplier).price
    };

    let mut sigma_top = 2.0;  // 波动率上限
    let mut sigma_floor = 0.001;  // 波动率下限
    let mut count = 0;  // 计数器
    let min_precision = 0.00001;  // 精度
    let mut sigma = 0.0;

    let o_sigma_top_price = calculate_theoretical_price(sigma_top);
    let o_sigma_floor_price = calculate_theoretical_price(sigma_floor);
    while (o_sigma_floor_price - price).abs() >= min_precision &&
        (o_sigma_top_price - price).abs() >= min_precision {
        sigma = (sigma_floor + sigma_top) / 2.0;
        let o_mid_price = calculate_theoretical_price(sigma);

        if (o_mid_price - price).abs() <= min_precision {
            break;
        } else if (o_sigma_floor_price - price) * (o_mid_price - price) < 0.0 {
            sigma_top = sigma
        } else {
            sigma_floor = sigma
        }

        count += 1;
        if count > 200 {
            sigma = 0.8;
            break;
        }
    }

    let greeks = black_scholes_greeks(under_type, s,r,b, sigma, is_call, k, t, multiplier);
    ImplyVolAndGreeksResult {
        vol: sigma,
        price: greeks.price,
        delta: greeks.delta,
        gamma: greeks.gamma,
        vega: greeks.vega,
        theta: greeks.theta,
    }
}