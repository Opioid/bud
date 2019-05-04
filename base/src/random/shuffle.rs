use super::Generator;

pub fn biased_shuffle<T>(data: &mut [T], rng: &mut Generator) {
    // Divisionless optimization with slight bias from
    // https://lemire.me/blog/2016/06/30/fast-random-shuffling/
    // (Upper variant has bias as well!)
    // More related information:
    // http://www.pcg-random.org/posts/bounded-rands.html
    for i in (0..data.len()).rev() {
        let r = rng.random_uint() as u64;
        let m = r * (i as u64 + 1);
        let other = m >> 32;

        data.swap(i, other as usize);
    }
}
