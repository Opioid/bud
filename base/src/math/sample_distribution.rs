use super::float2;

pub fn golden_ratio_2d(samples: &mut [float2], r: float2) {
    // set the initial first coordinate
    let mut x = r.v[0];
    let mut min = x;
    let mut idx = 0;

    // set the first coordinates
    for (i, s) in samples.iter_mut().enumerate() {
        (*s).v[1] = x;

        // keep the minimum
        if x < min {
            min = x;
            idx = i;
        }

        // increment the coordinate
        x += 0.618033988749894;
        if x >= 1.0 {
            x -= 1.0;
        }
    }

    // find the first Fibonacci >= N
    let mut f = 1;
    let mut fp = 1;
    let mut parity = 0;

    while f + fp < samples.len() {
        let tmp = f;
        f += fp;
        fp = tmp;
        parity += 1;
    }

    // set the increment and decrement
    let mut inc = fp;
    let mut dec = f;
    if parity & 1 > 0 {
        inc = f;
        dec = fp;
    }

    // permute the first coordinates
    samples[0].v[0] = samples[idx].v[1];
    for i in 1..samples.len() {
        if idx < dec {
            idx += inc;
            if idx >= samples.len() {
                idx -= dec;
            }
        } else {
            idx -= dec;
        }
        samples[i].v[0] = samples[idx].v[1];
    }

    // set the initial second coordinate
    let mut y = r.v[1];
    // set the second coordinates
    for s in samples.iter_mut() {
        (*s).v[1] = y;

        // increment the coordinate
        y += 0.618033988749894;
        if y >= 1.0 {
            y -= 1.0;
        }
    }
}
