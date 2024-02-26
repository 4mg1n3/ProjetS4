use hound;
use plotters::prelude::*;
use rustfft::num_complex::Complex;
use rustfft::FftPlanner;
//use hound::{WavReader, Error};
//use std::convert::Infallible;
//use plotters_bitmap::BitMapBackendError;
//use std::fs::File;
//use image::{Rgb, RgbImage};
use std::collections::VecDeque;
use rustfft::num_traits::Zero;







fn read_wav_file(filename: &str) -> Vec<f32> {
    let mut reader = hound::WavReader::open(filename).expect("Failed to open WAV file");
    let spec = reader.spec();
    assert_eq!(spec.sample_format, hound::SampleFormat::Int); // Assurez-vous que les échantillons sont au format entier
    // Déterminez le facteur de conversion nécessaire pour normaliser les échantillons
    let int_scale = 1.0 / (i32::MAX as f32); // Convertit de i32 à f32 en échelle [-1.0, 1.0]
    // Lisez et convertissez les échantillons en flottants normalisés
    reader.samples::<i32>().map(|s| s.unwrap() as f32 * int_scale).collect()
}

fn perform_fft(samples: Vec<f32>) -> Vec<Complex<f32>> {
    let windowed_samples: Vec<f32> = samples.iter().enumerate().map(|(i, &s)| s * (0.54 - 0.46 * f32::cos(2.0 * std::f32::consts::PI * i as f32 / (samples.len() as f32 - 1.0)))).collect();
    
    let mut input: Vec<Complex<f32>> = windowed_samples.into_iter().map(Complex::from).collect();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(input.len());
    fft.process(&mut input);
    input
}

fn stft(samples: Vec<f32>, window_size: usize, overlap: usize) -> Vec<Vec<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(window_size);
    let mut stft_result = Vec::new();

    for i in (0..samples.len() - window_size).step_by(overlap) {
        let windowed_samples: Vec<f32> = samples[i..i + window_size]
            .iter()
            .enumerate()
            .map(|(j, &s)| s * (0.54 - 0.46 * f32::cos(2.0 * std::f32::consts::PI * j as f32 / (window_size as f32 - 1.0))))
            .collect();

        let mut input: Vec<Complex<f32>> = windowed_samples
            .into_iter()
            .map(|s| Complex::new(s, 0.0))
            .collect();

        fft.process(&mut input);

        let magnitudes: Vec<f32> = input.iter().map(|c| c.norm()).collect();
        stft_result.push(magnitudes);
    }
    stft_result
}

// Function to plot waveform
fn plot_waveform(samples: &[f32], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Determine the Y-axis range based on the maximum and minimum values of samples
    let y_min = samples.iter().cloned().fold(f32::INFINITY, f32::min);
    let y_max = samples.iter().cloned().fold(f32::NEG_INFINITY, f32::max);
    let y_range = y_min..y_max;

    let mut chart = ChartBuilder::on(&root)
        .caption("Waveform Plot", ("Arial", 30).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..samples.len() as f64, y_range)?;

    chart.configure_mesh().draw()?;
    
    chart.draw_series(LineSeries::new(
        samples.iter().enumerate().map(|(i, &s)| (i as f64, f32::from(s))),
        &BLACK,
    ))?;
    
    

    Ok(())
}

// Function to calculate envelope
fn calculate_envelope(samples: &[f32]) -> Vec<f32> {
    let fft_len = samples.len().next_power_of_two();
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(fft_len);

    // Perform Hilbert transform
    let mut input: Vec<Complex<f32>> = samples.iter().map(|&s| Complex::new(s, 0.0)).collect();
    input.resize_with(fft_len, || Complex::zero());
    fft.process(&mut input);

    // Inverse transform to get the analytic signal
    let inverse_fft = planner.plan_fft_inverse(fft_len);
    inverse_fft.process(&mut input);

    // Calculate envelope magnitude
    let envelope: Vec<f32> = input.iter().map(|c| c.norm()).collect();
    envelope
}

// Function to plot envelope
fn plot_envelope(envelope: &[f32], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_value = envelope.iter().cloned().fold(f32::INFINITY, f32::min);
    let max_value = envelope.iter().cloned().fold(f32::NEG_INFINITY, f32::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Envelope Plot", ("Arial", 30).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..envelope.len() as f64, min_value..max_value)?;

    chart.configure_mesh().draw()?;

    chart.draw_series(LineSeries::new(
        envelope.iter().enumerate().map(|(i, &e)| (i as f64, e)),
        &GREEN,
    ))?;

    Ok(())
}


// Function to calculate spectrogram
fn spectrogram(samples: &[f32], window_size: usize, overlap: usize) -> Vec<Vec<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(window_size);

    let mut spectrogram_result = Vec::new();
    let mut buffer: VecDeque<f32> = VecDeque::with_capacity(window_size);

    for i in (0..samples.len() - window_size).step_by(overlap) {
        buffer.clear();
        buffer.extend(samples[i..i + window_size].iter());

        let mut windowed_samples: Vec<Complex<f32>> = buffer
            .iter()
            .map(|&s| Complex::new(s, 0.0))
            .collect();
        windowed_samples.resize_with(window_size, Complex::zero);

        fft.process(&mut windowed_samples);

        let magnitudes: Vec<f32> = windowed_samples.iter().map(|c| c.norm()).collect();
        spectrogram_result.push(magnitudes);
    }
    spectrogram_result
}

// Function to plot spectrogram
fn plot_spectrogram(spectrogram: &[Vec<f32>], filename: &str) -> Result<(), Box<dyn std::error::Error>> {
    let root = BitMapBackend::new(filename, (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    let min_value = spectrogram.iter().flat_map(|row| row.iter()).cloned().fold(f32::INFINITY, f32::min);
    let max_value = spectrogram.iter().flat_map(|row| row.iter()).cloned().fold(f32::NEG_INFINITY, f32::max);

    let mut chart = ChartBuilder::on(&root)
        .caption("Spectrogram", ("Arial", 30).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..spectrogram.len() as f64, 0.0..1.0)?;

    chart.configure_mesh().draw()?;

    chart
        .draw_series(
            spectrogram
                .iter()
                .enumerate()
                .flat_map(|(x, row)| {
                    row.iter().enumerate().map(move |(y, &intensity)| {
                        let color = map_intensity_to_color(intensity, min_value, max_value);
                        Rectangle::new([(x as f64, y as f64), ((x + 1) as f64, (y + 1) as f64)], color.filled())
                    })
                }),
        )
        .expect("Error drawing spectrogram");

    Ok(())
}

// Function to map intensity to color
fn map_intensity_to_color(intensity: f32, min_value: f32, max_value: f32) -> RGBColor {
    let normalized_intensity = (intensity - min_value) / (max_value - min_value);
    let scaled_intensity = (normalized_intensity * 255.0) as u8;

    let red_component = (scaled_intensity as f32).powf(1.5) as u8;

    RGBColor(red_component, red_component - red_component / 5, red_component - red_component / 5)
}






fn execute1() -> Result<(), Box<dyn std::error::Error>>{
    let filename = "ressources/sample.wav"; 
    let samples = read_wav_file(filename);
    let sample2: Vec<f32> = samples.clone();
    let fft_result = perform_fft(samples.clone());

    // Paramètres de la STFT
    let window_size = 1024;
    let overlap = 512;
    let _stft_result = stft(samples, window_size, overlap);

    let fft_len = fft_result.len();
    let half_len = fft_len / 2;


    // Plot waveform
    plot_waveform(&sample2, "waveform_plot.png")?;

    // Plot envelope
    // Calculate and plot envelope
    let envelope = calculate_envelope(&sample2);
    plot_envelope(&envelope, "envelope_plot.png")?;
    
    // Plot spectrogram
    let spectrogram_result = spectrogram(&sample2, window_size, overlap);
    plot_spectrogram(&spectrogram_result, "spectrogram_plot.png")?;


    // Créer un backend de tracé et une zone de dessin
    let root = BitMapBackend::new("fft_stft_plot1.png", (800, 600)).into_drawing_area();
    root.fill(&WHITE)?;

    // Créer un graphique avec Plotters pour la FFT
    let mut fft_chart = ChartBuilder::on(&root)
        .caption("FFT Plot", ("Arial", 30).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..half_len as f64, 0.0..1.0)?;

    // Configurer la grille du graphique FFT
    fft_chart.configure_mesh().draw()?;  

     //Tracer les données FFT
       fft_chart.draw_series(LineSeries::new(
       fft_result.iter().take(half_len).enumerate().map(|(i, complex)| (i as f64, complex.norm() as f64)),
       &RED,
   ))?;
/*
    // Créer un graphique avec Plotters pour la STFT
    let mut stft_chart = ChartBuilder::on(&root)
        .caption("STFT Plot", ("Arial", 30).into_font())
        .x_label_area_size(30)
        .y_label_area_size(30)
        .build_cartesian_2d(0.0..(stft_result.len() / 2) as f64, 0.0..1.0)?;

    // Configurer la grille du graphique STFT
    stft_chart.configure_mesh().draw()?;

    // Tracer les données STFT
    for (_i, magnitudes) in stft_result.iter().enumerate() {
        let half_len = magnitudes.len() / 2; // Utilisez la moitié de la longueur du spectre
        
        stft_chart.draw_series(LineSeries::new(
            magnitudes.iter().take(half_len).enumerate().map(|(j, &mag)| (j as f64, mag as f64)),
            &BLUE,
        ))?;
    }

 */
    Ok(())
}


fn main() {
    let _ =execute1();
}