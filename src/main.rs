use num::complex::Complex;
use std::f32;
use std::time::Instant;

fn main() {
    let waveform: Vec<_> = (0..10000)
        .map(|i| 0.001 * (i as f32))
        .map(|i| i.sin())
        .collect();

    println!("{:?}", &waveform[..20]);

    println!("Started calculated DFT for {} samples", waveform.len());
    let start = Instant::now();
    println!("{:?}", &dft(&waveform)[..20]);
    println!("This took {:?}", start.elapsed());
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
