use std::{f64::consts::PI, fs::File, process::id};

use itertools::Itertools;
use itertools_num::linspace;
use plotly::{
    layout::{Annotation, Legend},
    Layout, Plot, Scatter,
};
use signal_gen::*;

fn main() {
    let sample_rate = 1_000_000;
    let time = 3;
    let wave_freq = 3500;
    let harmonic_num = 4;
    let amp_max = 1000.0;

    let noise_signal = gen_noise(sample_rate, time);
    let freq_signal = gen_freq(sample_rate, time, wave_freq, harmonic_num);

    let i_data = scale_signal(
        freq_signal
            .into_iter()
            .zip(noise_signal.into_iter())
            .map(|(a, b)| a + b)
            .collect(),
        amp_max,
    );
    //let q_data = phase_shift_signal(&i_data, PI / 2.);
    /*
        let iq_data = i_data
            .clone()
            .into_iter()
            .zip(q_data.clone().into_iter())
            .flat_map(|(a, b)| [a, b])
           .collect::<Vec<_>>();
    */
    let x_nums = (0..1000).collect::<Vec<_>>();

    let trace = Scatter::new(x_nums.clone(), i_data.into_iter().take(1000).collect());

    let mut plot = Plot::new();
    for _ in 0..3 {
        let noise_signal = gen_noise(sample_rate, time);
        let freq_signal = gen_freq(sample_rate, time, wave_freq, harmonic_num);

        let i_data = scale_signal(
            freq_signal
                .into_iter()
                .zip(noise_signal.into_iter())
                .map(|(a, b)| a + b)
                .collect(),
            amp_max,
        );

        dbg!(i_data.len());

        plot.add_trace(Scatter::new(x_nums.clone(), i_data[..2000].to_vec()))
    }
    //plot.add_trace(trace);
    let layout = Layout::new();
    plot.set_layout(layout);

    plot.write_html("plot.html");
}
