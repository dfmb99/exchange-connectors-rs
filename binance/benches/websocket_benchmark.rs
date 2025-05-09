use criterion::{criterion_group, criterion_main, Criterion};

use binance::websocket::spot::*;

use core::time::Duration;

fn criterion_benchmark(c: &mut Criterion) {
    let mut group = c.benchmark_group("websockets-decoder");

    let btc_symbol_json =
        reqwest::blocking::get("https://api.binance.com/api/v3/ticker/price?symbol=BTCUSDT")
            .unwrap()
            .text()
            .unwrap();

    let mut web_socket: WebSockets<'_> = WebSockets::new(|_event: WebsocketEvent| Ok(()));

    group.sample_size(200);
    group.measurement_time(Duration::new(35, 0));
    group.bench_function("handle_msg BTCUSDT symbol", |b| {
        b.iter(|| web_socket.test_handle_msg(&btc_symbol_json))
    });
    group.finish();
}

criterion_group!(benches, criterion_benchmark);
criterion_main!(benches);
