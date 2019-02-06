use amethyst_assets::*;
use amethyst_error::Error;

use derivative::Derivative;
use serde::{Deserialize, Serialize};

use super::Source as Audio;

#[derive(Clone)]
pub struct AudioData(pub Vec<u8>);

/// Loads audio from wav files.
#[derive(Clone)]
pub struct WavFormat;

impl SimpleFormat<Audio> for WavFormat {
    const NAME: &'static str = "WAV";

    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: ()) -> Result<AudioData, Error> {
        Ok(AudioData(bytes))
    }
}

/// Loads audio from Ogg Vorbis files
#[derive(Clone)]
pub struct OggFormat;

impl SimpleFormat<Audio> for OggFormat {
    const NAME: &'static str = "OGG";

    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: ()) -> Result<AudioData, Error> {
        Ok(AudioData(bytes))
    }
}

/// Loads audio from Flac files.
#[derive(Clone)]
pub struct FlacFormat;

impl SimpleFormat<Audio> for FlacFormat {
    const NAME: &'static str = "FLAC";

    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: ()) -> Result<AudioData, Error> {
        Ok(AudioData(bytes))
    }
}

/// Loads audio from MP3 files.
#[derive(Clone)]
pub struct Mp3Format;

impl SimpleFormat<Audio> for Mp3Format {
    const NAME: &'static str = "MP3";

    type Options = ();

    fn import(&self, bytes: Vec<u8>, _: ()) -> Result<AudioData, Error> {
        Ok(AudioData(bytes))
    }
}
/// Aggregate sound format
#[derive(Debug, Clone, Deserialize, Serialize, Derivative)]
#[derivative(Default)]
pub enum AudioFormat {
    /// Ogg
    #[derivative(Default)]
    Ogg,
    /// Wav
    Wav,
    /// Flac
    Flac,
    /// Mp3
    Mp3,
}

impl SimpleFormat<Audio> for AudioFormat {
    const NAME: &'static str = "AudioFormat";

    type Options = ();

    fn import(&self, bytes: Vec<u8>, options: ()) -> Result<AudioData, Error> {
        match *self {
            AudioFormat::Ogg => SimpleFormat::import(&OggFormat, bytes, options),
            AudioFormat::Wav => SimpleFormat::import(&WavFormat, bytes, options),
            AudioFormat::Flac => SimpleFormat::import(&FlacFormat, bytes, options),
            AudioFormat::Mp3 => SimpleFormat::import(&Mp3Format, bytes, options),
        }
    }
}
