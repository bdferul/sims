use plotly::{Layout, Plot, Scatter};
use signal_gen::*;
use std::{f64::consts::PI, iter::Inspect, time::Instant};

fn main() {
    let timer = Instant::now();
    let sample_rate = 1_000_000;
    let time = 3;
    let wave_freq = 3500;
    let harmonic_num = 4;
    let amp_max = 1000.0;
    let sample_size = 5000;

    let noise_signal = gen_noise(sample_rate, time);
    let freq_signal = gen_freq(sample_rate, time, wave_freq, harmonic_num);

    dbg!(timer.elapsed());

    let i_data = scale_signal(
        freq_signal
            .into_iter()
            .zip(noise_signal.into_iter())
            .map(|(a, b)| a + b)
            .collect(),
        amp_max,
    )[..sample_size]
        .to_vec();
    dbg!(timer.elapsed());
    let q_data = phase_shift_signal(&i_data, PI);
    dbg!(timer.elapsed());
    let h_data = phase_shift_signal(&i_data, PI / 2.);
    dbg!(timer.elapsed());
    let t_data = phase_shift_signal(&i_data, PI / 3.);
    dbg!(timer.elapsed());

    let datas = (0..=20)
        .step_by(2)
        .map(|x| x as f64 / 10.)
        .map(|x| phase_shift_signal(&i_data, PI * x));

    /*
        let iq_data = i_data
            .clone()
            .into_iter()
            .zip(q_data.clone().into_iter())
            .flat_map(|(a, b)| [a, b])
           .collect::<Vec<_>>();
    */

    let x_nums = (0..sample_size).collect::<Vec<_>>();

    let mut plot = Plot::new();

    for d in datas {
        plot.add_trace(Scatter::new(x_nums.clone(), d[..sample_size].to_vec()))
    }
    //plot.add_trace(trace);
    let layout = Layout::new();
    plot.set_layout(layout);

    plot.write_html("out.html");
    dbg!(timer.elapsed());
}
