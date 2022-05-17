use std::collections::BTreeMap;
use std::collections::HashMap;
use std::hash::BuildHasher;
use std::hash::Hasher;

use criterion::{black_box, criterion_group, criterion_main, Criterion};

struct IntHasher(u64);

impl Hasher for IntHasher {
    fn write(&mut self, bytes: &[u8]) {
        self.0 = u64::from_le_bytes(bytes.try_into().unwrap());
    }

    fn finish(&self) -> u64 {
        self.0
    }
}

impl BuildHasher for IntHasher {
    type Hasher = IntHasher;
    fn build_hasher(&self) -> Self::Hasher {
        IntHasher(0)
    }
}

fn bench(c: &mut Criterion) {
    let mut btree: BTreeMap<usize, bool> = BTreeMap::new();
    btree.insert(1248, true);

    let mut map: HashMap<usize, bool> = HashMap::new();
    map.insert(1248, true);

    let mut intmap: HashMap<usize, bool, IntHasher> = HashMap::with_hasher(IntHasher(0));
    intmap.insert(1248, true);

    let vector = vec![
        // (1240, false),
        // (1245, false),
        (1246, false),
        (1247, false),
        (1248, false),
    ];

    c.bench_function("intmap", |b| {
        b.iter(|| {
            let v = intmap.get(&black_box(1248));
            // assert_eq!(*v.unwrap(), true);
        })
    });

    c.bench_function("treemap", |b| {
        b.iter(|| {
            btree.get(&black_box(1248));
        })
    });

    c.bench_function("hashmap", |b| {
        b.iter(|| {
            map.get(&black_box(1248));
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
