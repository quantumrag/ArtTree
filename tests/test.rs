
use art::ArtTree;
use rand::distributions::Alphanumeric;
use rand::Rng;
use std::hint::black_box;

#[test]
fn sanity_test() {
    type InsrtType = u64;
    let mut t = ArtTree::new();

    let mut rng = rand::thread_rng();

    let n = 5011;

    let mut keys = Vec::new();
    for _ in 0..n {
        keys.push(rng.gen::<InsrtType>());
    }

    for i in 0..n {
        black_box(t.insert(keys[i], keys[i]));
    }

    for i in 0..n {
        match t.get(&keys[i]) {
            None => assert!(false),
            Some(x) => assert_eq!(*x, keys[i]),
        }
    }
}

#[test]
fn sanity_test_u32() {
    type InsrtType = u32;
    let mut t = ArtTree::new();

    let mut rng = rand::thread_rng();

    let n = 5011;

    let mut keys = Vec::new();
    for _ in 0..n {
        keys.push(rng.gen::<InsrtType>());
    }

    for i in 0..n {
        black_box(t.insert(keys[i], keys[i]));
    }

    for i in 0..n {
        match t.get(&keys[i]) {
            None => assert!(false),
            Some(x) => assert_eq!(*x, keys[i]),
        }
    }
}

#[test]
fn sanity_seq_test() {
    let mut t = ArtTree::new();

    let n = 5011 as u32;

    for i in 0..n {
        black_box(t.insert(i, i));
    }

    for i in 0..n {
        match t.get(&i) {
            None => assert!(false),
            Some(x) => assert_eq!(*x, i),
        }
    }
}


#[test]
fn short_string_test() {
    let mut rng = rand::thread_rng();

    let mut keys = Vec::with_capacity(100);

    let mut t = ArtTree::new();
    for i in 0..100 {
        let s: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(50)
            .map(char::from)
            .collect();
        keys.push(s.clone());
        black_box(t.insert(s, i));
    }

    for i in 0..100 {
        match t.get(&keys[i]) {
            None => assert!(false),
            Some(x) => assert_eq!(*x, i),
        }
    }
}

#[test]
fn long_string_test() {
    let mut rng = rand::thread_rng();

    let mut keys = Vec::with_capacity(100);

    let mut t = ArtTree::new();
    for i in 0..100 {
        let s: String = (&mut rng)
            .sample_iter(&Alphanumeric)
            .take(500)
            .map(char::from)
            .collect();
        keys.push(s.clone());
        black_box(t.insert(s, i));
    }

    for i in 0..100 {
        match t.get(&keys[i]) {
            None => assert!(false),
            Some(x) => assert_eq!(*x, i),
        }
    }
}

#[test]
fn delete_test() {
    let mut t = ArtTree::new();

    let n = 1000 as u32;
    for i in 0..n {
        t.insert(i,i);
    }

    for i in 0..n {
        assert!(t.get(&i).is_some());
    }

    for i in (0..n).step_by(2) {
        match t.remove(&i) {
            Some(x) => assert_eq!(x, i),
            None => assert!(false),
        }
    }

    for i in 0..n {
        if i % 2 == 1 {
            match t.get(&i) {
                Some(x) => assert_eq!(*x, i),
                None => assert!(false),
            }
        } else if t.get(&i).is_some() {
            assert!(false);
        }
    }
}
