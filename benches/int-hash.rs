use std::collections::BTreeMap;
use std::collections::HashMap;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

fn bench(c: &mut Criterion) {
    let mut btree: BTreeMap<usize, bool> = BTreeMap::new();
    btree.insert(1248, true);

    let mut map: HashMap<usize, bool> = HashMap::new();
    map.insert(1248, true);

    let vector = vec![
        (1240, false),
        (1245, false),
        (1246, false),
        (1247, false),
        (1248, false),
    ];

    c.bench_function("treemap", |b| {
        b.iter(|| {
            btree.get(&black_box(0));
        })
    });

    c.bench_function("hashmap", |b| {
        b.iter(|| {
            map.get(&black_box(0));
        })
    });

    c.bench_function("vector", |b| {
        b.iter(|| {
            let mut value = None;
            for v in &vector {
                if v.0 == 1248 {
                    value.replace(black_box(v.1));
                    break;
                }
            }
            black_box(value);
        })
    });
}

criterion_group!(benches, bench,);
criterion_main!(benches);
