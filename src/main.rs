use rand::Rng;

pub trait MySort<T>
where
    T: Ord,
{
    fn two_way_merge(&mut self, mid: usize);
    fn merge_sort(&mut self);
}

impl<T> MySort<T> for [T]
where
    T: Ord + Clone + Copy,
{
    // merge self[..mid] and self[mid..]
    fn two_way_merge(&mut self, mid: usize) {
        let forward = self[..mid].to_vec();
        let backward = self[mid..].to_vec();

        let mut f_i = 0;
        let mut b_i = 0;
        for x in self {
            if forward.get(f_i).is_some() && backward.get(b_i).is_some() {
                if forward[f_i] <= backward[b_i] {
                    *x = forward[f_i];
                    f_i += 1;
                } else {
                    *x = backward[b_i];
                    b_i += 1;
                }
            } else {
                if forward.get(f_i).is_some() {
                    *x = forward[f_i];
                    f_i += 1;
                } else {
                    *x = backward[b_i];
                    b_i += 1;
                }
            }
        }
    }

    fn merge_sort(&mut self) {
        if self.len() <= 1 {
            return;
        }

        let mid = self.len() / 2;

        self[..mid].merge_sort();
        self[mid..].merge_sort();

        self.two_way_merge(mid);
    }
}

fn main() {
    let mut rng = rand::thread_rng();
    let len = 1 << 21;
    println!("length of vector: {}", len);
    let mut v = (0..len)
        .map(|_| rng.gen_range(0..1000000))
        .collect::<Vec<usize>>();
    let mut u = v.clone();

    let start = std::time::Instant::now();
    v.merge_sort();
    let end = start.elapsed();
    println!("my merge sort: {} ns", end.subsec_nanos() / 1_000_000);

    let start = std::time::Instant::now();
    u.sort();
    let end = start.elapsed();
    println!("std sort: {} ns", end.subsec_nanos() / 1_000_000);
}
