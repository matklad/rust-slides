extern crate rayon;

fn quick_sort(xs: &mut[i32]) {
    if xs.len() <= 1 { return }
    let mid = partition(xs);
    let (lo, hi) = xs.split_at_mut(mid);
    rayon::join(|| quick_sort(lo), || quick_sort(hi));
}

//fn partition(xs: &mut[i32]) -> usize { /* … */ }

fn main() {
    let mut xs = [1, 3, 0, 6, 2, 4, 92];
    quick_sort(&mut xs);
}


fn partition(xs: &mut[i32]) -> usize {
    let pivot = xs.len() - 1;
    let mut i = 0;
    for j in 0..pivot {
        if xs[j] <= xs[pivot] {
            xs.swap(i, j);
            i += 1;
        }
    }
    xs.swap(i, pivot);
    i
}

