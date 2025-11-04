use std::{
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc,
    },
    time::{Duration, Instant},
};

use futures::stream::{self, StreamExt};
use reqwest::Client;
use tokio::time::sleep;

#[derive(Clone)]
struct Cfg {
    url: String,
    token: String,
    total: u64,
    concurrency: usize,
    timeout: Duration,
    warmup: u64,
}

#[tokio::main]
async fn main() {
    // Token JWT fornecido pelo usuário
    let token_padrao = "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJpZF91c3VhcmlvIjoxLCJlbWFpbCI6ImFkbWluQGV4YW1wbGUuY29tIiwiZXhwIjoxNzYyMzYzMzUzfQ.blk_aCrkhBA-SzHJS7unTPaRt5EZE2gwT5eHJl_QIG8";
    
    let cfg = Cfg {
        url: std::env::var("URL").unwrap_or_else(|_| "http://127.0.0.1:3000/api/posts/my".into()),
        token: std::env::var("TOKEN").unwrap_or_else(|_| token_padrao.to_string()),
        total: std::env::var("N").ok().and_then(|s| s.parse().ok()).unwrap_or(20_000),
        concurrency: std::env::var("C").ok().and_then(|s| s.parse().ok()).unwrap_or(200),
        timeout: Duration::from_secs(10),
        warmup: std::env::var("WARMUP").ok().and_then(|s| s.parse().ok()).unwrap_or(2_000),
    };

    println!("🔐 Teste de carga para endpoint protegido");
    println!("📊 URL: {}", cfg.url);
    println!("📈 Total de requisições: {}", cfg.total);
    println!("⚡ Concorrência: {}", cfg.concurrency);
    println!("🔑 Token: {}...", &cfg.token[..20]);

    // Cliente HTTP com ajustes de pool/keep-alive
    let client = Client::builder()
        .http1_only()
        .pool_max_idle_per_host(cfg.concurrency.max(64))
        .pool_idle_timeout(Duration::from_secs(30))
        .tcp_nodelay(true)
        .timeout(cfg.timeout)
        .build()
        .expect("client");

    // Warmup
    if cfg.warmup > 0 {
        println!("\n🔥 Warmup: {} req @ c={}", cfg.warmup, cfg.concurrency.min(100));
        let start_w = Instant::now();
        run_load(&client, &cfg.url, &cfg.token, cfg.warmup, cfg.concurrency.min(100), false).await;
        println!("✅ Warmup concluído em {:.2}s\n", start_w.elapsed().as_secs_f64());
        sleep(Duration::from_millis(200)).await;
    }

    // Carga principal
    let t0 = Instant::now();
    let stats = run_load(&client, &cfg.url, &cfg.token, cfg.total, cfg.concurrency, true).await;
    let elapsed = t0.elapsed();

    let total = stats.success + stats.errors;
    let rps = total as f64 / elapsed.as_secs_f64();
    let avg_ms = if stats.samples > 0 {
        stats.latency_sum_ms as f64 / stats.samples as f64
    } else { 0.0 };

    println!("\n✅ Teste concluído!");
    println!("   ✅ Sucessos: {}", stats.success);
    println!("   ❌ Erros: {}", stats.errors);
    println!("   ⏱️  Tempo total: {:.2}s", elapsed.as_secs_f64());
    println!("   📈 Requisições/segundo: {:.2}", rps);
    println!("   ⚡ Tempo médio (apenas sucesso): {:.1}ms", avg_ms);
    println!("   📉 Taxa de sucesso: {:.2}%", (stats.success as f64 / total as f64) * 100.0);
    
    if stats.errors > 0 {
        println!("   ⚠️  Alguns erros ocorreram. Verifique se o token está válido e o servidor está rodando.");
    } else {
        println!("   🚀 Performance: Excelente!");
    }
}

struct Stats {
    success: u64,
    errors: u64,
    latency_sum_ms: u64,
    samples: u64,
}

async fn run_load(
    client: &Client,
    url: &str,
    token: &str,
    total: u64,
    concurrency: usize,
    show_progress: bool,
) -> Stats {
    let success = Arc::new(AtomicU64::new(0));
    let errors = Arc::new(AtomicU64::new(0));
    let latency_sum_ms = Arc::new(AtomicU64::new(0));
    let samples = Arc::new(AtomicU64::new(0));

    // Stream de índices 0..total, consumido com concorrência fixa
    let stream = stream::iter(0..total);

    stream
        .for_each_concurrent(concurrency, |i| {
            let client = client.clone();
            let url = url.to_string();
            let token = token.to_string();
            let success = success.clone();
            let errors = errors.clone();
            let latency_sum_ms = latency_sum_ms.clone();
            let samples = samples.clone();

            async move {
                // Progress a cada 1000 requisições
                if show_progress && i % 1000 == 0 && i > 0 {
                    println!("📊 Progresso: {}/{} requisições...", i, total);
                }

                let t0 = Instant::now();
                match client
                    .get(&url)
                    .header("Authorization", format!("Bearer {}", token))
                    .send()
                    .await
                {
                    Ok(resp) => {
                        let ok = resp.status().is_success();
                        let ms = t0.elapsed().as_millis() as u64;

                        if ok {
                            success.fetch_add(1, Ordering::Relaxed);
                            latency_sum_ms.fetch_add(ms, Ordering::Relaxed);
                            samples.fetch_add(1, Ordering::Relaxed);
                        } else {
                            errors.fetch_add(1, Ordering::Relaxed);
                            // Log apenas alguns erros para não poluir a saída
                            if i < 10 || i % 1000 == 0 {
                                eprintln!("❌ Erro na requisição {}: status {}", i, resp.status());
                            }
                        }
                    }
                    Err(e) => {
                        errors.fetch_add(1, Ordering::Relaxed);
                        if i < 10 || i % 1000 == 0 {
                            eprintln!("❌ Erro na requisição {}: {}", i, e);
                        }
                    }
                }
            }
        })
        .await;

    Stats {
        success: success.load(Ordering::Relaxed),
        errors: errors.load(Ordering::Relaxed),
        latency_sum_ms: latency_sum_ms.load(Ordering::Relaxed),
        samples: samples.load(Ordering::Relaxed),
    }
}

