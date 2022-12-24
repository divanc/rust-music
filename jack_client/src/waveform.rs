use std::f32::consts::PI;

#[derive(Copy,Clone, Debug)]
pub enum Waveform {
    Sine,
    Square,
    Sawtooth,
}

impl Waveform {
    pub fn process(self, nframes: usize, input: &[f32], output: &mut [f32]) {
        match self {
            Self::Sine => Self::sine(nframes, input, output),
            Self::Square => Self::square(nframes, input, output),
            Self::Sawtooth => Self::sawtooth(nframes, input, output),
        }
    }

    fn sine(nframes: usize, input: &[f32], output: &mut [f32]) {
        for i in 0..nframes {
            let frame = input[i];

            output[i] = (2.0 * PI * frame).sin();
        }
    }

    fn square(nframes: usize, input: &[f32], output: &mut [f32]) {
        for i in 0..nframes {
            let frame = input[i];
            output[i] = if frame < 0.5 { -1.0 } else { 1.0 }
        }
    }

    fn sawtooth(nframes: usize, input: &[f32], output: &mut [f32]) {
        for i in 0..nframes {
            let frame = input[i];

            output[i] = 2.0 * frame - 1.0;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use float_cmp::approx_eq;

    #[test]
    fn waveform_sine() {
        const NFRAMES: usize = 5;
        let input = [0.0, 0.25, 0.5, 0.75, 1.0];
        let mut output = [0.0; NFRAMES];
        let wave = Waveform::Sine;

        wave.process(NFRAMES, &input, &mut output);

        let expected = [0.0, 1.0, 0.0, -1.0, 0.0];

        for i in 0..NFRAMES {
            assert!(approx_eq!(
                f32,
                output[i],
                expected[i],
                epsilon = 0.00001,
                ulps = 2
            ));
        }
    }

    #[test]
    fn waveform_square() {
        const NFRAMES: usize = 5;
        let input = [0.0, 0.25, 0.5, 0.75, 1.0];
        let mut output = [0.0; NFRAMES];
        let wave = Waveform::Square;

        wave.process(NFRAMES, &input, &mut output);

        let expected = [-1.0, -1.0, 1.0, 1.0, 1.0];
        assert_eq!(output, expected);
    }

    #[test]
    fn waveform_sawtooth() {
        const NFRAMES: usize = 5;
        let input = [0.0, 0.25, 0.5, 0.75, 1.0];
        let mut output = [0.0; NFRAMES];
        let wave = Waveform::Sawtooth;

        wave.process(NFRAMES, &input, &mut output);

        let expected = [-1.0, -0.5, 0.0, 0.5, 1.0];
        assert_eq!(output, expected);
    }
}
