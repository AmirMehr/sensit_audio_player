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
/// This struct provides functionality for loading MP3 files, extracting samples, sample rate,
/// and channel count for audio playback.
pub struct Mp3Loader;

impl AudioFileLoader for Mp3Loader {
    /// Loads audio samples, sample rate, and channel count from an MP3 file.
    ///
    /// # Parameters
    /// - `path`: The path to the MP3 file.
    ///
    /// # Returns
    /// - `Ok((Vec<f32>, u32, u16))`: A tuple containing audio samples, sample rate, and channel count.
    /// - `Err(Box<dyn Error>)`: An error if the file cannot be opened or decoded.
    fn load_samples(&self, path: &Path) -> Result<(Vec<f32>, u32, u16), Box<dyn Error>> {
        // Open the MP3 file as a media source stream.
        let src = File::open(path)?;
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        // Use a hint to inform symphonia of the file format (MP3).
        let mut hint = Hint::new();
        hint.with_extension("mp3");

        // Set up format, decoder, and metadata options.
        let fmt_opts = FormatOptions::default();
        let dec_opts = DecoderOptions::default();
        let meta_opts = MetadataOptions::default();

        // Probe the file to identify the format and read the file data.
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .expect("Unsupported format");

        let mut format = probed.format;

        // Find the first valid audio track with a compatible codec.
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .expect("No supported audio tracks found.");

        // Extract sample rate and channel count from the codec parameters.
        let sample_rate = track.codec_params.sample_rate.expect("Sample rate missing");
        let channels = track
            .codec_params
            .channels
            .expect("Channel count missing")
            .bits();

        // Initialize the decoder for the selected track.
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("Unsupported codec.");

        let mut samples = Vec::new();
        let track_id = track.id;

        // Process each packet in the audio track.
        while let Ok(packet) = format.next_packet() {
            if packet.track_id() == track_id {
                // Decode the audio packet.
                match decoder.decode(&packet) {
                    Ok(decoded) => match decoded {
                        // Handle 32-bit floating-point samples.
                        AudioBufferRef::F32(buffer) => {
                            let channel_count = buffer.spec().channels.count();
                            if channel_count == 1 {
                                // Duplicate samples for mono files to play on both channels in stereo.
                                samples.extend(buffer.chan(0).iter().flat_map(|&s| vec![s, s]));
                            } else {
                                // Add samples directly for stereo files.
                                samples.extend(buffer.chan(0).iter().copied());
                            }
                        }
                        // Handle 32-bit integer samples.
                        AudioBufferRef::S32(buffer) => {
                            let channel_count = buffer.spec().channels.count();
                            if channel_count == 1 {
                                samples.extend(buffer.chan(0).iter().flat_map(|&s| {
                                    vec![s as f32 / i32::MAX as f32, s as f32 / i32::MAX as f32]
                                }));
                            } else {
                                samples.extend(
                                    buffer.chan(0).iter().map(|&s| s as f32 / i32::MAX as f32),
                                );
                            }
                        }
                    },
                    Err(err) => {
                        eprintln!("Decode error: {:?}", err);
                        continue;
                    }
                }
            }
        }

        // Return the decoded samples, sample rate, and channel count.
        Ok((samples, sample_rate, channels as u16))
    }
}
