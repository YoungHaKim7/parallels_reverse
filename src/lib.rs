use rayon::{iter::ParallelIterator, slice::ParallelSliceMut};

#[derive(Debug, Clone)]
pub struct Item {
    pub data: i32,
}

pub fn par_binary_rev_sort(items: &mut [Item]) {
    let n = items.len();
    if n <= 1 {
        return;
    }

    // 1️⃣ Split into chunks (binary leaves)
    let chunk_size = 2.max(n / rayon::current_num_threads());

    items.par_chunks_mut(chunk_size).for_each(|chunk| {
        chunk.sort_unstable_by(|a, b| b.data.cmp(&a.data));
    });

    // 2️⃣ Merge chunks bottom-up (binary merge)
    let width = chunk_size;
    while width < n {
        items.par_chunks_mut(width * 2).for_each(|chunk| {
            let mid = chunk.len() / 2;
            if mid < chunk.len() {
                // ✅ FIX: split into two disjoint mutable slices
                let (left, right) = chunk.split_at_mut(mid);
                merge_rev(left, right);
            }
        });
    }
}

// Reverse-order merge (like mergesort)
pub fn merge_rev(left: &mut [Item], right: &mut [Item]) {
    let mut merged = Vec::with_capacity(left.len() + right.len());

    let (mut i, mut j) = (0, 0);
    while i < left.len() && j < right.len() {
        if left[i].data >= right[j].data {
            merged.push(left[i].clone());
            i += 1;
        } else {
            merged.push(right[j].clone());
            j += 1;
        }
    }

    merged.extend_from_slice(&left[i..]);
    merged.extend_from_slice(&right[j..]);

    left.iter_mut()
        .chain(right.iter_mut())
        .zip(merged)
        .for_each(|(dst, src)| *dst = src);
}
