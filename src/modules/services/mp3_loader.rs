use std::error::Error;
use std::fs::File;
use std::path::Path;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

use super::audio_loader::AudioFileLoader;

/// **Mp3Loader Struct**
///
/// This struct loads MP3 files, extracting samples, sample rate, and channel count.
pub struct Mp3Loader;

impl AudioFileLoader for Mp3Loader {
    /// Loads audio samples, sample rate, and channel count from an MP3 file.
    ///
    /// # Parameters
    /// - `path`: The path to the MP3 file.
    ///
    /// # Returns
    /// - `Ok((Vec<f32>, u32, u16))`: A tuple containing audio samples, sample rate, and channels.
    /// - `Err(Box<dyn Error>)`: An error if the file cannot be opened or decoded.
    fn load_samples(&self, path: &Path) -> Result<(Vec<f32>, u32, u16), Box<dyn Error>> {
        let src = File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        let mut hint = Hint::new();
        hint.with_extension("mp3");

        let fmt_opts = FormatOptions::default();
        let dec_opts = DecoderOptions::default();
        let meta_opts = MetadataOptions::default();

        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .expect("Unsupported format");

        let mut format = probed.format;

        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .expect("No supported audio tracks found.");

        let sample_rate = track.codec_params.sample_rate.expect("Sample rate missing");
        let channels = track
            .codec_params
            .channels
            .expect("Channel count missing")
            .bits();

        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("Unsupported codec.");

        let mut samples = Vec::new();
        let track_id = track.id;

        while let Ok(packet) = format.next_packet() {
            if packet.track_id() == track_id {
                match decoder.decode(&packet) {
                    Ok(decoded) => match decoded {
                        AudioBufferRef::F32(buffer) => {
                            samples.extend(buffer.chan(0).iter().copied())
                        }
                        AudioBufferRef::S32(buffer) => samples
                            .extend(buffer.chan(0).iter().map(|&s| s as f32 / i32::MAX as f32)),
                    },
                    Err(err) => {
                        eprintln!("Decode error: {:?}", err);
                        continue;
                    }
                }
            }
        }

        Ok((samples, sample_rate, channels as u16))
    }
}
