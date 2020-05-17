use num::complex::Complex;
use std::f32;
use std::time::Instant;

fn main() {
    let waveform: Vec<_> = (0..2_i32.pow(12))
        .map(|i| 0.1 * (i as f32))
        .map(|i| i.sin())
        .collect();

    for i in &waveform {
        // println!("{}", i);
    }

    // println!("Started calculating DFT for {} samples...", waveform.len());
    // let start = Instant::now();
    let d = dft(&waveform);
    for i in d {
        println!("{}", i.re);
    }
    // println!("{:?}", &d[..20]);
    // println!("This took {:?}", start.elapsed());

    // println!("calculating using FFT...");
    // let start2 = Instant::now();
    let f = cooley_tukey(&waveform);
    // println!("{:?}", &f[..20]);
    // println!("This took {:?}", start2.elapsed());
}

fn dft(x: &[f32]) -> Vec<Complex<f32>> {
    let n = x.len();
    (0..n)
        .map(|i| {
            x.iter()
                .enumerate()
                .fold(Complex::new(0.0, 0.0), |acc, (k, j)| {
                    acc + j * Complex::new(
                        0.0,
                        (-2.0 * f32::consts::PI * i as f32 * k as f32) / n as f32,
                    )
                    .exp()
                })
        })
        .collect()
}

fn cooley_tukey(x: &[f32]) -> Vec<Complex<f32>> {
    let n = x.len();
    if n <= 1 {
        return vec![Complex::new(x[0], 0.0)];
    }

    // okay i admit this looks really bad
    let evens: Vec<_> = x
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 == 0)
        .map(|(_, a)| *a)
        .collect();
    let odds: Vec<_> = x
        .iter()
        .enumerate()
        .filter(|(i, _)| i % 2 != 0)
        .map(|(_, a)| *a)
        .collect();

    let evens = cooley_tukey(&evens);
    let odds = cooley_tukey(&odds);

    let mut dst = vec![Complex::new(0.0, 0.0); n];
    for k in 0..n / 2 {
        dst[k] = evens[k]
            + Complex::new(0.0, (-2.0 * f32::consts::PI * k as f32) / n as f32).exp() * odds[k];
        dst[k + n / 2] = evens[k]
            - Complex::new(0.0, (-2.0 * f32::consts::PI * k as f32) / n as f32).exp() * odds[k];
    }

    dst
}
