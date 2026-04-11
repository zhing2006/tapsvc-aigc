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
    /// Edit an existing image
    Edit {
        /// Model name
        #[arg(short, long)]
        model: String,

        /// Input image path (PNG/JPEG/WebP, < 25MB)
        #[arg(long)]
        image: String,

        /// Text prompt
        #[arg(short, long)]
        prompt: Option<String>,

        /// Read prompt from file
        #[arg(long)]
        prompt_file: Option<String>,

        /// Mask image path (PNG only, < 4MB, transparent areas mark edit regions)
        #[arg(long)]
        mask: Option<String>,

        /// Output image size
        #[arg(long, default_value = "1024x1024")]
        size: String,

        /// Number of images to generate (1-10)
        #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..=10))]
        n: u32,

        /// Output image format (png, jpeg, webp)
        #[arg(long, default_value = "png")]
        response_format: String,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
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

        /// Number of images to generate (1-10)
        #[arg(short, long, default_value_t = 1, value_parser = clap::value_parser!(u32).range(1..=10))]
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

        /// Speech speed
        #[arg(long, default_value_t = 1.0)]
        speed: f32,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
}

const VIDEO_TASK_STATUSES: [&str; 5] = ["queued", "running", "succeeded", "failed", "cancelled"];

#[derive(Subcommand)]
#[allow(clippy::large_enum_variant)]
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

        /// First frame image (image-to-video)
        #[arg(long)]
        first_frame: Option<String>,

        /// Last frame image (requires --first-frame)
        #[arg(long)]
        last_frame: Option<String>,

        /// Reference image path/URL (repeatable, max 9, mutually exclusive with --first-frame)
        #[arg(long, action = clap::ArgAction::Append)]
        ref_image: Vec<String>,

        /// Reference video URL (repeatable, max 3, URL only, mutually exclusive with --first-frame)
        #[arg(long, action = clap::ArgAction::Append)]
        ref_video: Vec<String>,

        /// Reference audio path/URL (repeatable, max 3, requires --ref-image or --ref-video)
        #[arg(long, action = clap::ArgAction::Append)]
        ref_audio: Vec<String>,

        /// Resolution (480p, 720p)
        #[arg(long, default_value = "720p", value_parser = ["480p", "720p"])]
        resolution: String,

        /// Aspect ratio (16:9, 4:3, 1:1, 3:4, 9:16, 21:9, adaptive)
        #[arg(long, default_value = "adaptive", value_parser = ["16:9", "4:3", "1:1", "3:4", "9:16", "21:9", "adaptive"])]
        aspect_ratio: String,

        /// Video duration in seconds (4-15, or -1 for auto)
        #[arg(long, default_value_t = 5)]
        duration: i32,

        /// Disable audio generation (audio is generated by default)
        #[arg(long, default_value_t = false)]
        no_audio: bool,

        /// Add watermark to video
        #[arg(long, default_value_t = false)]
        watermark: bool,

        /// Enable web search enhancement
        #[arg(long, default_value_t = false)]
        web_search: bool,

        /// Fix camera position
        #[arg(long, default_value_t = false)]
        camera_fixed: bool,

        /// Random seed for reproducibility
        #[arg(long)]
        seed: Option<u64>,

        /// Poll interval in seconds
        #[arg(long, default_value_t = 10)]
        poll_interval: u64,

        /// Timeout in seconds
        #[arg(long, default_value_t = 300)]
        timeout: u64,

        /// Output file path
        #[arg(short, long)]
        output: Option<String>,
    },
    /// Get a video generation task by ID
    Get {
        /// Task ID
        task_id: String,
    },
    /// List video generation tasks
    List {
        /// Filter by status
        #[arg(short, long, value_parser = VIDEO_TASK_STATUSES)]
        status: Option<String>,

        /// Filter by model
        #[arg(short, long)]
        model: Option<String>,

        /// Filter by task IDs
        #[arg(long, num_args = 1..)]
        task_ids: Option<Vec<String>>,

        /// Page number
        #[arg(short, long, default_value_t = 1)]
        page: u32,

        /// Page size
        #[arg(short = 'n', long, default_value_t = 10)]
        page_size: u32,
    },
    /// Delete a video generation task
    Delete {
        /// Task ID
        task_id: String,
    },
}
