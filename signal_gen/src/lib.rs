use dft::Plan;
use itertools::Itertools;
use rand::Rng;
use realfft::RealFftPlanner;
use rustfft::{
    num_complex::{Complex, Complex64},
    num_traits::Zero,
    FftPlanner,
};
use std::f64::consts::PI;

// EVERYTHING BELOW THIS LINE IS STOLEN
// ----------------------------------------------------------------------------

/// Generate a random noise signal.
///
/// # Parameters:
///
/// `sample_rate`: The number of samples per second.
///
/// `duration`: The duration of the signal in seconds.
///
/// Returns a vector of random noise.
pub fn gen_noise(sample_rate: usize, duration: usize) -> Vec<f64> {
    let mut rng = rand::thread_rng();

    std::iter::repeat_with(|| rng.gen())
        .take(sample_rate * duration)
        .collect()
}

/// Generate a harmonic signal.
///
/// # Parameters
/// `sample_rate`: The number of samples per second.
///
/// `duration`: The duration of the signal in seconds.
///
/// `frequency`: The base frequency of the signal in Hz.
///  
/// `harmonics`: The number of harmonics to include in the signal.  
/// # Return
/// Returns a vector of the harmonic signal.
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
    let plan = Plan::new(dft::Operation::Forward, signal.len());
    let mut data = signal
        .iter()
        .map(|&x| dft::c64::new(x, 0.))
        .collect::<Vec<_>>();

    dft::transform(&mut data, &plan);

    let plan = Plan::new(dft::Operation::Inverse, signal.len());
    dft::transform(&mut data, &plan);

    let data = data.iter().map(|x| x.re).collect::<Vec<_>>();

    assert_eq!(&signal[..3], &data[..3]);

    data
}
