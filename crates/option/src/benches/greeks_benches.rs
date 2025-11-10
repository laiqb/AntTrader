use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion, Throughput};
use ant_option::black_scholes_greeks;
use ant_option::imply_vol;
use statrs::distribution::Normal;

// 复用线程本地正态分布（与核心逻辑一致，避免重复初始化）
thread_local! {
    static STANDARD_NORMAL: Normal = Normal::new(0.0, 1.0).unwrap();
}

// 测试 1：Black76 期货期权 Greeks 计算
fn bench_black76(c: &mut Criterion) {
    let params = (
        68000.0,    // f: 期货现价
        0.03,       // r: 无风险利率
        0.18,       // sigma: 波动率
        true,       // is_call: 看涨
        70000.0,    // k: 执行价
        0.5,        // t: 剩余期限（年化）
        10.0,       // multiplier: 合约乘数
    );

    let mut group = c.benchmark_group("Black76_Greeks");
    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("Futures_Call", "Greeks"), |b| {
        b.iter(|| {
            let (f, r, sigma, is_call, k, t, multiplier) = params;
            let result = black_scholes_greeks(
                criterion::black_box(2),
                criterion::black_box(f),
                criterion::black_box(r),
                criterion::black_box(0.0),
                criterion::black_box(sigma),
                criterion::black_box(is_call),
                criterion::black_box(k),
                criterion::black_box(t),
                criterion::black_box(multiplier),
            );
            criterion::black_box(result);
        });
    });
    group.finish();
}

// 测试 2：隐含波动率求解
fn bench_iv(c: &mut Criterion) {
    let params = (
        1523.45,    // market_price
        68000.0,    // s_or_f
        70000.0,    // k
        0.5,        // t
        0.03,       // r
        0.0,        // b
        true,       // is_call
        10.0,       // multiplier
        1e-6,       // tolerance
        500,        // max_iter
    );

    let mut group = c.benchmark_group("Implied_Vol");
    group.throughput(Throughput::Elements(1));
    group.bench_function(BenchmarkId::new("Futures_Call", "IV"), |b| {
        b.iter(|| {
            let (mp, s_f, k, t, r, b, ic, m, tol, mi) = params;
            let iv = imply_vol(
                criterion::black_box(2),
                criterion::black_box(mp),
                criterion::black_box(s_f),
                criterion::black_box(r),
                criterion::black_box(0.0),
                criterion::black_box(ic),
                criterion::black_box(k),
                criterion::black_box(t),
                criterion::black_box(100.00),
            );
            criterion::black_box(iv);
        });
    });
    group.finish();
}

// 注册测试组
criterion_group!(benches, bench_black76, bench_iv);
criterion_main!(benches);