mod util;

type Real = f32;

pub fn binarization(grayscale: &[u8]) -> Result<Vec<u8>, &'static str> {
    let length = grayscale.len();

    if length == 0 {
        return Err("A `grayscale` slice is empty");
    }

    let mut alpha: Real = 0.3;

    let mut freq = [0u64; 256];
    let mut probability = [0.0 as Real; 256];

    for g in grayscale {
        freq[*g as usize] += 1;
    }

    // `t` (threshold value) is the most frequent color in the image
    let t = util::argmax(freq.iter()).unwrap();

    let mut h:  Real = 0.0;
    let mut hw: Real = 0.0;
    let mut hb: Real = 0.0;
    let mut xw: Real = 0.0;
    let mut xb: Real = 0.0;

    for (i, v) in freq.iter().enumerate() {
        if *v != 0 {
            probability[i] = *v as Real / length as Real;

            let a = probability[i] * probability[i].log(length as Real);

            h += a;

            if i <= t {
                hb += a;
                xb += *v as Real
            } else {
                hw += a;
                xw += *v as Real;
            }
        }
    }

    h *= -1.0;
    hw *= -1.0;
    hb *= -1.0;

    xw /= length as Real;
    xb /= length as Real;

    let mut mb = 2.2 as Real;
    let mut mw = 3.0 as Real;

    if h <= 0.26 {
        // class 1
        if 0.1 <= hw {
            // typewritten documents with dark ink and bright paper
            mb = 2.5;
            mw = 4.5;
        } else if 0.08 < hw && hw < 0.1 {
            // documents with the ink faded
            mb = 6.0;
            mw = 6.0;
            alpha = 0.35;
        } else {
            // documents with dark ink and paper
            mb = 4.0;
            mw = 4.0;
        }
    } else if 0.26 < h && h < 0.30 {
        // class 2
        if 0.1 < hw {
            mw = 1.5;
        }

        if t > 200 {
            mw = 9.0;
        }
    } else {
        // class 3; documents with more black pixels than expected in a normal document
        mb = 1.0;

        mw = 2.0;

        if t >= 185 {
            if 0.071 < hw && hw < 0.096 {
                mw = 9.0;
            } else if 0.096 <= hw && hw < 0.2 {
                mw = 6.0;
            }
        }
    }

    let mut sum_below_t = 0.0 as Real;
    let mut sum_above_t = 0.0 as Real;

    for (i, p) in probability.iter().enumerate() {
        if i <= t {
            sum_below_t += (*p).powf(alpha);
        } else {
            sum_above_t += (*p).powf(alpha);
        }
    }

    let hb_alpha = xb / (alpha - 1.0) - sum_below_t / (alpha - 1.0);
    let hw_alpha = xw / (alpha - 1.0) - sum_above_t / (alpha - 1.0);

    let th = (mb * hb_alpha + mw * hw_alpha).round() as u8;

    let mut bin: Vec<u8> = vec![0u8; length];

    for i in 0..length {
        if grayscale[i] > th {
            bin[i] = 255;
        }
    }

    Ok(bin)
}
