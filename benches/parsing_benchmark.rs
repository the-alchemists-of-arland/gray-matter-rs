use criterion::{black_box, criterion_group, criterion_main, Criterion};
use gray_matter::engine::YAML;
use gray_matter::{Matter, ParsedEntity};
use serde::{Deserialize, Serialize};

// Test data structures
#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct BlogPost {
    title: String,
    author: String,
    date: String,
    tags: Vec<String>,
    published: bool,
    meta: PostMeta,
}

#[derive(Deserialize, Serialize, Debug, Clone, PartialEq)]
struct PostMeta {
    word_count: u32,
    reading_time: u32,
    category: String,
}

fn parse_old(input: &str) -> Result<BlogPost, Box<dyn std::error::Error>> {
    use gray_matter_old::engine::YAML;
    use gray_matter_old::Matter;
    let matter: Matter<YAML> = Matter::new();

    if let Some(parsed) = matter.parse_with_struct(input) {
        Ok(parsed.data)
    } else {
        Err("No front matter found".into())
    }
}

fn parse_new(input: &str) -> Result<BlogPost, Box<dyn std::error::Error>> {
    let matter: Matter<YAML> = Matter::new();

    let parsed: ParsedEntity<BlogPost> = matter.parse(input)?;

    if let Some(blog_post) = parsed.data {
        Ok(blog_post)
    } else {
        Err("No front matter found".into())
    }
}

// Test data generation
fn generate_test_data() -> Vec<(&'static str, String)> {
    vec![
        ("simple", r#"---
title: "Hello World"
author: "John Doe"
date: "2023-01-01"
tags: ["rust", "parsing"]
published: true
meta:
  word_count: 500
  reading_time: 3
  category: "tutorial"
---
This is the content of the blog post."#.to_string()),

        ("complex", r#"---
title: "Advanced Rust Programming Techniques"
author: "Jane Smith"
date: "2023-12-15"
tags: ["rust", "advanced", "programming", "performance", "memory-management"]
published: true
meta:
  word_count: 2500
  reading_time: 12
  category: "advanced-tutorial"
---
This is a comprehensive guide to advanced Rust programming techniques including memory management, performance optimization, and concurrent programming patterns."#.to_string()),

        ("minimal", r#"---
title: "Quick Note"
author: "Bob"
date: "2023-06-01"
tags: ["note"]
published: false
meta:
  word_count: 50
  reading_time: 1
  category: "note"
---
Just a quick note."#.to_string()),
    ]
}

fn benchmark_old(c: &mut Criterion) {
    let test_data = generate_test_data();

    c.bench_function("old_simple", |b| {
        let (_, input) = &test_data[0];
        b.iter(|| parse_old(black_box(input)))
    });

    c.bench_function("old_complex", |b| {
        let (_, input) = &test_data[1];
        b.iter(|| parse_old(black_box(input)))
    });

    c.bench_function("old_minimal", |b| {
        let (_, input) = &test_data[2];
        b.iter(|| parse_old(black_box(input)))
    });
}

fn benchmark_new(c: &mut Criterion) {
    let test_data = generate_test_data();

    c.bench_function("new_simple", |b| {
        let (_, input) = &test_data[0];
        b.iter(|| parse_new(black_box(input)))
    });

    c.bench_function("new_complex", |b| {
        let (_, input) = &test_data[1];
        b.iter(|| parse_new(black_box(input)))
    });

    c.bench_function("new_minimal", |b| {
        let (_, input) = &test_data[2];
        b.iter(|| parse_new(black_box(input)))
    });
}

fn benchmark_comparison(c: &mut Criterion) {
    let test_data = generate_test_data();
    let mut group = c.benchmark_group("comparison");

    for (name, input) in test_data.iter() {
        group.bench_with_input(format!("old_{}", name), input, |b, input| {
            b.iter(|| parse_old(black_box(input)))
        });

        group.bench_with_input(format!("new_{}", name), input, |b, input| {
            b.iter(|| parse_new(black_box(input)))
        });
    }

    group.finish();
}

// Throughput benchmarks
fn benchmark_throughput(c: &mut Criterion) {
    let test_data = generate_test_data();
    let complex_input = &test_data[1].1;

    let mut group = c.benchmark_group("throughput");
    group.throughput(criterion::Throughput::Bytes(complex_input.len() as u64));

    group.bench_function("old_throughput", |b| {
        b.iter(|| parse_old(black_box(complex_input)))
    });

    group.bench_function("new_throughput", |b| {
        b.iter(|| parse_new(black_box(complex_input)))
    });

    group.finish();
}

// Batch processing benchmark
fn benchmark_batch_processing(c: &mut Criterion) {
    let test_data = generate_test_data();
    let inputs: Vec<_> = test_data.iter().map(|(_, input)| input.as_str()).collect();

    c.bench_function("old_batch", |b| {
        b.iter(|| {
            for input in &inputs {
                let _ = parse_old(black_box(input));
            }
        })
    });

    c.bench_function("new_batch", |b| {
        b.iter(|| {
            for input in &inputs {
                let _ = parse_new(black_box(input));
            }
        })
    });
}

criterion_group!(
    benches,
    benchmark_old,
    benchmark_new,
    benchmark_comparison,
    benchmark_throughput,
    benchmark_batch_processing
);
criterion_main!(benches);
