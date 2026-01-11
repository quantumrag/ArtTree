use art::ArtTree;
use criterion::{black_box, criterion_group, criterion_main, Criterion};
use rand::distributions::Alphanumeric;
use rand::rngs::StdRng;
use rand::{Rng, SeedableRng};
use std::collections::{BTreeMap, HashMap};

const N: usize = 100_000;
const N_STR_20: usize = 40_000;
const N_STR_100: usize = 7_000;
const N_STR_1000: usize = 1_000;
const N_SEARCH: usize = 100_000;

fn new_rng() -> StdRng {
    // Deterministic seed for reproducible benches.
    StdRng::from_seed([42u8; 32])
}

fn rand_bytes(rng: &mut impl Rng, len: usize) -> Vec<u8> {
    rng.sample_iter(&Alphanumeric).take(len).collect()
}

fn bench_insert_u64(c: &mut Criterion) {
    c.bench_function("insert/art/u64", |b| {
        let mut rng = new_rng();
        b.iter(|| {
            let mut t = ArtTree::new();
            for _ in 0..N {
                black_box(t.insert(rng.gen::<u64>(), rng.gen::<u64>()));
            }
        })
    });

    c.bench_function("insert/btree/u64", |b| {
        let mut rng = new_rng();
        b.iter(|| {
            let mut t = BTreeMap::new();
            for _ in 0..N {
                black_box(t.insert(rng.gen::<u64>(), rng.gen::<u64>()));
            }
        })
    });

    c.bench_function("insert/hashmap/u64", |b| {
        let mut rng = new_rng();
        b.iter(|| {
            let mut t = HashMap::new();
            for _ in 0..N {
                black_box(t.insert(rng.gen::<u64>(), rng.gen::<u64>()));
            }
        })
    });
}

fn bench_insert_u32(c: &mut Criterion) {
    c.bench_function("insert/art/u32", |b| {
        let mut rng = new_rng();
        b.iter(|| {
            let mut t = ArtTree::new();
            for _ in 0..N {
                black_box(t.insert(rng.gen::<u32>(), rng.gen::<u32>()));
            }
        })
    });

    c.bench_function("insert/btree/u32", |b| {
        let mut rng = new_rng();
        b.iter(|| {
            let mut t = BTreeMap::new();
            for _ in 0..N {
                black_box(t.insert(rng.gen::<u32>(), rng.gen::<u32>()));
            }
        })
    });

    c.bench_function("insert/hashmap/u32", |b| {
        let mut rng = new_rng();
        b.iter(|| {
            let mut t = HashMap::new();
            for _ in 0..N {
                black_box(t.insert(rng.gen::<u32>(), rng.gen::<u32>()));
            }
        })
    });
}

fn bench_insert_vec(c: &mut Criterion) {
    for (len, n) in [(20usize, N_STR_20), (100, N_STR_100), (1000, N_STR_1000)] {
        c.bench_function(&format!("insert/art/vec_u8/{len}"), |b| {
            let mut rng = new_rng();
            b.iter(|| {
                let mut t: ArtTree<Vec<u8>, usize> = ArtTree::new();
                for i in 0..n {
                    black_box(t.insert(rand_bytes(&mut rng, len), i));
                }
            })
        });

        c.bench_function(&format!("insert/btree/vec_u8/{len}"), |b| {
            let mut rng = new_rng();
            b.iter(|| {
                let mut t: BTreeMap<Vec<u8>, usize> = BTreeMap::new();
                for i in 0..n {
                    black_box(t.insert(rand_bytes(&mut rng, len), i));
                }
            })
        });

        c.bench_function(&format!("insert/hashmap/vec_u8/{len}"), |b| {
            let mut rng = new_rng();
            b.iter(|| {
                let mut t: HashMap<Vec<u8>, usize> = HashMap::new();
                for i in 0..n {
                    black_box(t.insert(rand_bytes(&mut rng, len), i));
                }
            })
        });
    }
}

fn bench_search_seq_u64(c: &mut Criterion) {
    let mut art = ArtTree::new();
    let mut btree = BTreeMap::new();
    let mut hmap = HashMap::new();

    for i in 0u64..(N_SEARCH as u64) {
        art.insert(i, i);
        btree.insert(i, i);
        hmap.insert(i, i);
    }

    c.bench_function("search_seq/art/u64", |b| {
        b.iter(|| {
            for i in 0u64..(N_SEARCH as u64) {
                black_box(art.get(&i));
            }
        })
    });

    c.bench_function("search_seq/btree/u64", |b| {
        b.iter(|| {
            for i in 0u64..(N_SEARCH as u64) {
                black_box(btree.get(&i));
            }
        })
    });

    c.bench_function("search_seq/hashmap/u64", |b| {
        b.iter(|| {
            for i in 0u64..(N_SEARCH as u64) {
                black_box(hmap.get(&i));
            }
        })
    });
}

fn bench_search_rnd_u64(c: &mut Criterion) {
    let mut rng = new_rng();

    let mut art = ArtTree::new();
    let mut btree = BTreeMap::new();
    let mut hmap = HashMap::new();

    let mut keys: Vec<u64> = Vec::with_capacity(N_SEARCH);
    for _ in 0..N_SEARCH {
        let k = rng.gen::<u64>();
        keys.push(k);
        art.insert(k, k);
        btree.insert(k, k);
        hmap.insert(k, k);
    }

    c.bench_function("search_rnd/art/u64", |b| {
        b.iter(|| {
            for i in 0..(10 * keys.len()) {
                let k = keys[i % keys.len()];
                black_box(art.get(&k));
            }
        })
    });

    c.bench_function("search_rnd/btree/u64", |b| {
        b.iter(|| {
            for i in 0..(10 * keys.len()) {
                let k = keys[i % keys.len()];
                black_box(btree.get(&k));
            }
        })
    });

    c.bench_function("search_rnd/hashmap/u64", |b| {
        b.iter(|| {
            for i in 0..(10 * keys.len()) {
                let k = keys[i % keys.len()];
                black_box(hmap.get(&k));
            }
        })
    });
}

criterion_group!(
    benches,
    bench_insert_u64,
    bench_insert_u32,
    bench_insert_vec,
    bench_search_seq_u64,
    bench_search_rnd_u64,
);
criterion_main!(benches);
