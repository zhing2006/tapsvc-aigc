use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(
    name = "tapsvc-aigc",
    about = "TAPSVC AIGC CLI — image, audio, video generation"
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Subcommand)]
pub enum Command {
    /// Image generation
    Image {
        #[command(subcommand)]
        command: ImageCommand,
    },
    /// Audio text-to-speech
    Audio {
        #[command(subcommand)]
        command: AudioCommand,
    },
    /// Video generation
    Video {
        #[command(subcommand)]
        command: VideoCommand,
    },
}

#[derive(Subcommand)]
pub enum ImageCommand {
    /// Generate images from text prompt
    Generate {
        /// Model name
        #[arg(short, long)]
        model: String,

        /// Text prompt
        #[arg(short, long)]
        prompt: Option<String>,

        /// Read prompt from file
        #[arg(long)]
        prompt_file: Option<String>,

        /// Image size
        #[arg(long, default_value = "1024x1024")]
        size: String,

        /// Number of images to generate
        #[arg(short, long, default_value_t = 1)]
        n: u32,

        /// Quality level (auto, high, medium, low)
        #[arg(long, default_value = "auto")]
        quality: String,

        /// Output image format (png, jpeg, webp)
        #[arg(long, default_value = "png")]
        response_format: String,

        /// Background type (transparent, opaque, auto)
        #[arg(long, default_value = "auto")]
        background: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum AudioCommand {
    /// Generate speech from text
    Speech {
        /// Model name
        #[arg(short, long)]
        model: String,

        /// Voice name
        #[arg(long)]
        voice: String,

        /// Text input
        #[arg(short, long)]
        input: Option<String>,

        /// Read text from file
        #[arg(long)]
        input_file: Option<String>,

        /// Output format
        #[arg(long, default_value = "mp3")]
        format: String,

        /// Speech speed (0.25 - 4.0)
        #[arg(long, default_value_t = 1.0)]
        speed: f32,

        /// Voice style instructions (e.g. "Speak in a cheerful tone")
        #[arg(long)]
        instructions: Option<String>,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
}

#[derive(Subcommand)]
pub enum VideoCommand {
    /// Generate video from text/image
    Generate {
        /// Model name
        #[arg(short, long)]
        model: String,

        /// Text prompt
        #[arg(short, long)]
        prompt: Option<String>,

        /// Read prompt from file
        #[arg(long)]
        prompt_file: Option<String>,

        /// Reference image (image-to-video)
        #[arg(long)]
        image: Option<String>,

        /// First frame image
        #[arg(long)]
        first_frame: Option<String>,

        /// Last frame image
        #[arg(long)]
        last_frame: Option<String>,

        /// Video duration in seconds (4-15)
        #[arg(long, default_value_t = 5)]
        duration: u32,

        /// Resolution (480p, 720p, 1080p, 2K)
        #[arg(long, default_value = "1080p")]
        resolution: String,

        /// Aspect ratio (1:1, 16:9, 9:16, 4:3, 3:4, 21:9, adaptive)
        #[arg(long, default_value = "16:9")]
        aspect_ratio: String,

        /// Add watermark to video
        #[arg(long, default_value_t = false)]
        watermark: bool,

        /// Generate audio along with video
        #[arg(long, default_value_t = false)]
        generate_audio: bool,

        /// Poll interval in seconds
        #[arg(long, default_value_t = 5)]
        poll_interval: u64,

        /// Timeout in seconds
        #[arg(long, default_value_t = 300)]
        timeout: u64,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
}
