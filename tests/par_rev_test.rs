use parallels_reverse::*;
use rayon::slice::ParallelSliceMut;

fn is_desc_sorted(items: &[Item]) -> bool {
    items.windows(2).all(|w| w[0].data >= w[1].data)
}

#[test]
fn test_empty() {
    let mut v: Vec<Item> = vec![];
    par_binary_rev_sort(&mut v);
    assert!(v.is_empty());
}

#[test]
fn test_single() {
    let mut v = vec![Item { data: 42 }];
    par_binary_rev_sort(&mut v);
    assert_eq!(v[0].data, 42);
}

#[test]
fn test_small() {
    let mut v = vec![
        Item { data: 3 },
        Item { data: 1 },
        Item { data: 4 },
        Item { data: 2 },
    ];

    par_binary_rev_sort(&mut v);

    assert!(is_desc_sorted(&v));
    assert_eq!(
        v.iter().map(|x| x.data).collect::<Vec<_>>(),
        vec![4, 3, 2, 1]
    );
}

#[test]
fn test_duplicates() {
    let mut v = vec![
        Item { data: 5 },
        Item { data: 1 },
        Item { data: 5 },
        Item { data: 3 },
    ];

    par_binary_rev_sort(&mut v);

    assert!(is_desc_sorted(&v));
    assert_eq!(
        v.iter().map(|x| x.data).collect::<Vec<_>>(),
        vec![5, 5, 3, 1]
    );
}

#[test]
fn test_large() {
    let mut v: Vec<Item> = (0..10_000).map(|i| Item { data: i }).collect();

    // reverse input
    v.reverse();

    par_binary_rev_sort(&mut v);

    assert!(is_desc_sorted(&v));
    assert_eq!(v.first().unwrap().data, 9_999);
    assert_eq!(v.last().unwrap().data, 0);
}

#[test]
fn test_matches_rayon_sort() {
    let mut v1: Vec<Item> = (0..1000).map(|i| Item { data: i }).collect();

    let mut v2 = v1.clone();

    par_binary_rev_sort(&mut v1);

    v2.par_sort_unstable_by(|a, b| b.data.cmp(&a.data));

    assert_eq!(
        v1.iter().map(|x| x.data).collect::<Vec<_>>(),
        v2.iter().map(|x| x.data).collect::<Vec<_>>(),
    );
}
