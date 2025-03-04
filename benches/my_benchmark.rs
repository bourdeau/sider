use criterion::async_executor::FuturesExecutor;
use criterion::{criterion_group, criterion_main, BenchmarkId, Criterion};
use indexmap::IndexMap;
use sider::process::process_command;
use std::sync::Arc;
use tokio::runtime::Runtime;
use tokio::sync::RwLock;

fn benchmark_process_command(c: &mut Criterion) {
    let rt = Runtime::new().unwrap();
    let db = Arc::new(RwLock::new(IndexMap::new()));

    for &size in &[1, 10, 100, 1000, 10_000] {
        c.bench_with_input(BenchmarkId::new("set_command", size), &size, |b, &size| {
            b.to_async(FuturesExecutor).iter(|| {
                let db = db.clone();
                rt.spawn(async move {
                    for i in 0..size {
                        let command = format!("SET key{} value{}", i, i);
                        process_command(command.clone(), &db, true).await;
                    }
                })
            });
        });

        c.bench_with_input(BenchmarkId::new("get_command", size), &size, |b, &size| {
            b.to_async(FuturesExecutor).iter(|| {
                let db = db.clone();
                rt.spawn(async move {
                    for i in 0..size {
                        let command = format!("GET key{}", i);
                        process_command(command.clone(), &db, true).await;
                    }
                })
            });
        });

        c.bench_with_input(BenchmarkId::new("del_command", size), &size, |b, &size| {
            b.to_async(FuturesExecutor).iter(|| {
                let db = db.clone();
                rt.spawn(async move {
                    for i in 0..size {
                        let command = format!("DEL key{}", i);
                        process_command(command.clone(), &db, true).await;
                    }
                })
            });
        });

        c.bench_with_input(
            BenchmarkId::new("keys_command", size),
            &size,
            |b, &_size| {
                b.to_async(FuturesExecutor).iter(|| {
                    let db = db.clone();
                    rt.spawn(async move {
                        let command = "KEYS *".to_string();
                        process_command(command.clone(), &db, true).await;
                    })
                });
            },
        );
    }
}

criterion_group!(benches, benchmark_process_command);
criterion_main!(benches);
