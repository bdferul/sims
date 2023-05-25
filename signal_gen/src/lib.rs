use plotly::scatter_mapbox;
use rand::Rng;
use realfft::RealFftPlanner;
use rustfft::{num_complex::Complex, FftPlanner};
use std::{f64::consts::PI, time::Instant};

// EVERYTHING BELOW THIS LINE IS STOLEN
// ----------------------------------------------------------------------------

/// Generate a random noise signal.
pub fn gen_noise(sample_rate: usize, duration: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    std::iter::repeat_with(|| rng.gen())
        .take(sample_rate * duration)
        .collect()
}

/// Generate a harmonic signal.
/// ```
/// let signal = signal_gen::gen_freq(7,7,7,7);
/// ```
pub fn gen_freq(
    sample_rate: usize,
    duration: usize,
    frequency: usize,
    harmonics: usize,
) -> Vec<f64> {
    let sample_size = sample_rate * duration;
    let linsp = itertools_num::linspace(0.0, duration as f64, sample_size + 1)
        .take(sample_size)
        .collect::<Vec<_>>();
    let mut waveform = vec![0.0; linsp.len()];

    for n in 1..=harmonics {
        let amplitude = 1.0 / n as f64;
        let phase: f64 = rand::random::<f64>() * 2.0 * PI;

        waveform
            .iter_mut()
            .zip(
                linsp
                    .iter()
                    .map(|l| l * 2.0 * PI * n as f64 * frequency as f64 + phase)
                    .map(f64::sin),
            )
            .for_each(|(a, b)| {
                *a += amplitude * b;
            });
    }

    waveform
}

/// Scale a signal to a range defined by max_value.
pub fn scale_signal(signal: Vec<f64>, max_value: f64) -> Vec<f64> {
    signal.into_iter().map(|x| x * max_value).collect()
}

/// Shift the phase of a signal.
pub fn phase_shift_signal(signal: &Vec<f64>, shift_angle: f64) -> Vec<f64> {
    let timer = Instant::now();
    let length = signal.len();
    let len_sqrt = (length as f64).sqrt();

    let mut planner = RealFftPlanner::<f64>::new();
    let r2c = planner.plan_fft_forward(length);

    let mut indata = signal.clone();
    let mut spectrum = r2c.make_output_vec();
    let mut scratch = r2c.make_scratch_vec();
    r2c.process_with_scratch(&mut indata, &mut spectrum, &mut scratch)
        .unwrap();

    dbg!(timer.elapsed());

    for x in &mut spectrum {
        *x /= len_sqrt;
        *x *= shift_angle.cos() + shift_angle.sin() * Complex::i();
    }

    dbg!(timer.elapsed());

    spectrum[0].im = 0.;
    spectrum.last_mut().unwrap().im = 0.;

    let c2r = planner.plan_fft_inverse(length);
    let mut outdata = c2r.make_output_vec();

    c2r.process_with_scratch(&mut spectrum, &mut outdata, &mut scratch)
        .unwrap();

    for x in &mut outdata {
        *x /= len_sqrt;
    }

    dbg!(timer.elapsed());
    outdata
}

pub fn _phase_shift_signal(signal: &Vec<f64>, shift_angle: f64) -> Vec<f64> {
    let timer = Instant::now();
    let length = signal.len();
    let len_sqrt = (length as f64).sqrt();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(length);

    let mut buffer = signal
        .iter()
        .map(|&x| Complex::new(x, 0.))
        .collect::<Vec<_>>();
    fft.process(&mut buffer);

    for x in &mut buffer {
        *x /= len_sqrt;
        *x *= shift_angle.cos() + shift_angle.sin() * Complex::i();
    }

    let fft = planner.plan_fft_inverse(length);
    fft.process(&mut buffer);

    for x in &mut buffer {
        *x /= len_sqrt;
    }

    dbg!(timer.elapsed());
    buffer.iter().map(|x| x.re).collect()
}
