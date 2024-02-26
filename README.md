
# Rust Audio Analysis

This is a small Rust project for performing audio analysis, including FFT, STFT, waveform plotting, envelope calculation, and spectrogram visualization.

## Prerequisites

Make sure you have [Rust](https://www.rust-lang.org/tools/install) and [Cargo](https://doc.rust-lang.org/cargo/getting-started/installation.html) installed on your system.

## Getting Started

1. Clone the repository:

   ```bash
   git clone https://github.com/4mg1n3/ProjetS4.git
   cd ProjetS4
   ```

2. Build and run the project:

   ```bash
   cargo run
   ```

3. Check the generated plots in the project directory.

## Features

- **FFT (Fast Fourier Transform):** Perform FFT on audio samples and plot the frequency domain.
- **STFT (Short-Time Fourier Transform):** Calculate and visualize the spectrogram of the audio signal.
- **Waveform Plotting:** Plot the time-domain waveform of the audio signal.
- **Envelope Calculation:** Compute and visualize the envelope of the audio signal.

## File Structure

- `src/main.rs`: Main entry point of the Rust program.
- `src/audio_analysis.rs`: Module containing functions for audio analysis.
- `resources/sample.wav`: Example audio file for analysis.

## Usage

Adjust the parameters in `main.rs` and the analysis functions in `audio_analysis.rs` to fit your specific use case. The generated plots will be saved in the project directory.

## Dependencies

- `hound`: Reading and writing WAV files.
- `plotters`: Plotting library.
- `rustfft`: Fast Fourier Transform library.
- `image`: Image processing library.

## License

This project is licensed under the MIT License - see the [LICENSE](LICENSE) file for details.

## Acknowledgments

- This project uses the [plotters](https://github.com/38/plotters) library for plotting.
- FFT and STFT computations are done using the [rustfft](https://github.com/awelkie/rustfft) library.
- Audio file reading/writing is done using the [hound](https://github.com/ruuda/hound) library.

