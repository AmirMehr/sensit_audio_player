use std::error::Error;
use std::fs::File;
use std::path::Path;
use symphonia::core::audio::{AudioBufferRef, Signal};
use symphonia::core::codecs::{DecoderOptions, CODEC_TYPE_NULL};
use symphonia::core::formats::FormatOptions;
use symphonia::core::io::MediaSourceStream;
use symphonia::core::meta::MetadataOptions;
use symphonia::core::probe::Hint;

/// **Mp3Loader Struct**
///
/// This struct provides functionality to load MP3 audio files
/// and convert them into a vector of samples (`Vec<f32>`).
pub struct Mp3Loader;

impl Mp3Loader {
    /// **Load Samples from MP3 File**
    ///
    /// This method reads the MP3 file, decodes it frame by frame,
    /// and converts the audio samples into a vector of floating-point values.
    ///
    /// # Parameters
    /// - `path`: A reference to the path of the MP3 file.
    ///
    /// # Returns
    /// - `Ok((Vec<f32>, u32))`: A vector of audio samples and the sample rate on success.
    /// - `Err(Box<dyn Error>)`: An error if the file cannot be opened or decoded.
    pub fn load_samples(&self, path: &Path) -> Result<(Vec<f32>, u32), Box<dyn Error>> {
        // Open the MP3 file from the given path.
        let src = File::open(path)?;

        // Wrap the opened file in a MediaSourceStream for reading.
        let mss = MediaSourceStream::new(Box::new(src), Default::default());

        // Create a format hint based on the file extension (e.g., "mp3").
        let mut hint = Hint::new();
        hint.with_extension("mp3");

        // Use default options for format, decoder, and metadata handling.
        let fmt_opts = FormatOptions::default();
        let dec_opts = DecoderOptions::default();
        let meta_opts = MetadataOptions::default();

        // Probe the media source to determine its format and prepare for decoding.
        let probed = symphonia::default::get_probe()
            .format(&hint, mss, &fmt_opts, &meta_opts)
            .expect("Unsupported format");

        // Get the format reader from the probed media source.
        let mut format = probed.format;

        // Find the first audio track with a supported codec.
        let track = format
            .tracks()
            .iter()
            .find(|t| t.codec_params.codec != CODEC_TYPE_NULL)
            .expect("No supported audio tracks found.");

        // Extract the sample rate from the track parameters.
        let sample_rate = track.codec_params.sample_rate.expect("Sample rate missing");

        // Create a decoder for the found audio track.
        let mut decoder = symphonia::default::get_codecs()
            .make(&track.codec_params, &dec_opts)
            .expect("Unsupported codec.");

        // Initialize the samples vector to store decoded audio samples.
        let mut samples = Vec::new();

        // Store the track ID to filter packets belonging to the target track.
        let track_id = track.id;

        // Loop through each packet in the audio stream.
        while let Ok(packet) = format.next_packet() {
            // Process packets that belong to the selected track.
            if packet.track_id() == track_id {
                // Decode the packet and match the result on success.
                match decoder.decode(&packet) {
                    Ok(decoded) => match decoded {
                        // Handle floating-point samples (F32).
                        AudioBufferRef::F32(buffer) => {
                            samples.extend(buffer.chan(0).iter().copied());
                        }
                        // Handle 32-bit signed integer samples (S32).
                        AudioBufferRef::S32(buffer) => {
                            samples
                                .extend(buffer.chan(0).iter().map(|&s| s as f32 / i32::MAX as f32));
                        }
                    },
                    // On decoding error, log it and skip to the next packet.
                    Err(err) => {
                        eprintln!("Decode error: {:?}", err);
                        continue; // Skip the faulty frame.
                    }
                }
            }
        }

        // Return the successfully decoded samples and the sample rate.
        Ok((samples, sample_rate))
    }
}
