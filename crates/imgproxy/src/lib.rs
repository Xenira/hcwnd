use std::fmt::Display;

use base64::{Engine as _, prelude::BASE64_URL_SAFE_NO_PAD};
use derive_builder::Builder;
use hmac::{Hmac, KeyInit as _, Mac as _};
use itertools::Itertools as _;
use sha2::Sha256;
use url::Url;

use crate::error::*;
pub mod error;

const RESIZE: &str = "rs";
const SIZE: &str = "s";
const RESIZING_TYPE: &str = "rt";
const WIDTH: &str = "w";
const HEIGHT: &str = "h";
const MIN_WIDTH: &str = "mw";
const MIN_HEIGHT: &str = "mh";
const ZOOM: &str = "z";
const DPR: &str = "dpr";
const ENLARGE: &str = "el";
const EXTEND: &str = "ex";
const EXTEND_ASPECT_RATIO: &str = "exar";
const GRAVITY: &str = "g";
const CROP: &str = "c";
const TRIM: &str = "t";
const PADDING: &str = "pd";
const AUTO_ROTATE: &str = "ar";
const ROTATE: &str = "rot";
const FLIP: &str = "fl";
const BACKGROUND: &str = "bg";
const BLUR: &str = "bl";
const SHARPEN: &str = "sh";
const PIXELATE: &str = "pix";
const WATERMARK: &str = "wm";
const STRIP_METADATA: &str = "sm";
const KEEP_COPYRIGHT: &str = "kcr";
const STRIP_COLOR_PROFILE: &str = "scp";
const ENFORCE_THUMBNAIL: &str = "eth";
const QUALITY: &str = "q";
const FORMAT_QUALITY: &str = "fq";
const MAX_BYTES: &str = "mb";
const FORMAT: &str = "f";
const SKIP_PROCESSING: &str = "skp";
const RAW: &str = "raw";
const CACHE_BUSTER: &str = "cb";
const EXPIRE: &str = "exp";
const FILENAME: &str = "fn";
const RETURN_ATTACHMENT: &str = "att";
const PRESET: &str = "pr";
const MAX_SRC_RESOLUTION: &str = "msr";
const MAX_SRC_FILE_SIZE: &str = "msfs";
const MAX_ANIMATION_FRAMES: &str = "maf";
const MAX_ANIMATION_FRAME_RESOLUTION: &str = "mafr";
const MAX_RESULT_DIMENSION: &str = "mrd";

pub trait FormatProcessingOption {
    fn name(&self) -> &str;
    fn options(&self) -> String;
    fn format(&self) -> String {
        format!("{}:{}", self.name(), self.options().trim_end_matches(':'))
    }
}

pub struct ImageUrl {
    image_url: String,
    processing_options: Vec<ProcessingOption>,
    format: String,
}

impl ImageUrl {
    pub fn new(image_url: Url) -> Self {
        let encoded_url = BASE64_URL_SAFE_NO_PAD.encode(image_url.as_str());
        let format = image_url
            .path_segments()
            .and_then(Iterator::last)
            .and_then(|filename| filename.split('.').next_back())
            .unwrap_or("png")
            .to_string();
        Self {
            image_url: encoded_url,
            processing_options: Vec::new(),
            format,
        }
    }

    pub fn parse(image_url: &str) -> Result<Self> {
        let url = Url::parse(image_url).map_err(|e| Error::InvalidUrl(e.to_string()))?;
        Ok(Self::new(url))
    }

    pub fn with_option(mut self, option: ProcessingOption) -> Self {
        self.processing_options.push(option);

        self
    }
}

impl Display for ImageUrl {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let options_str = self
            .processing_options
            .iter()
            .map(|option| option.format())
            .join("/");
        write!(f, "{options_str}/{}.{}", self.image_url, self.format)
    }
}

pub enum ProcessingOption {
    /// This is a meta-option that defines the [ResizeMode], width, height, enlarge, and extend. All arguments are optional and can be omitted to use their default values.
    Resize(ResizingOptions),
    /// This is a meta-option that defines the width, height, enlarge, and extend. All arguments are optional and can be omitted to use their default values.
    Size(SizeOptions),
    /// Defines how imgproxy will resize the source image.
    ResizingType(ResizeMode),
    /// Defines the width of the resulting image. When set to 0, imgproxy will calculate width using the defined height and source aspect ratio. When set to 0 and resizing type is force, imgproxy will keep the original width.
    Width(u32),
    /// Defines the height of the resulting image. When set to 0, imgproxy will calculate resulting height using the defined width and source aspect ratio. When set to 0 and resizing type is force, imgproxy will keep the original height.
    Height(u32),
    /// Defines the minimum width of the resulting image.
    MinWidth(u32),
    /// Defines the minimum height of the resulting image.
    MinHeight(u32),
    Zoom(Zoom),
    /// When set, imgproxy will multiply the image dimensions according to this factor for HiDPI (Retina) devices. The value must be greater than 0.
    Dpr(u32),
    /// When set to `true`, imgproxy will enlarge the image if it is smaller than the given size.
    Enlarge(bool),
    /// TODO: add gravity option
    Extend(bool),
    /// TODO: add gravity option
    ExtendAspectRatio(bool),
    /// When imgproxy needs to cut some parts of the image, it is guided by the gravity option.
    Gravity(GravityOptions),
    Crop(CropOptions),
    Trim(TrimOptions),
    Padding(PaddingOptions),
    /// When set to `true`, imgproxy will automatically rotate images based on the EXIF Orientation parameter (if available in the image meta data). The orientation tag will be removed from the image in all cases. Normally this is controlled by the `IMGPROXY_AUTO_ROTATE` configuration but this processing option allows the configuration to be set for each request.
    AutoRotate(bool),
    /// Rotates the image on the specified angle. The orientation from the image metadata is applied before the rotation unless autorotation is disabled.
    Rotate(RotateAngle),
    /// When set, imgproxy will flip the image along the specified axes.
    Flip(bool, bool),
    /// When set, imgproxy will fill the resulting image background with the specified color. Useful when you convert an image with alpha-channel to JPEG.
    ///
    /// With no arguments provided, disables any background manipulations.
    Background(Option<Colour>),
    /// When set, imgproxy will apply a gaussian blur filter to the resulting image. The value of [ProcessingOption::Blur] defines the size of the mask imgproxy will use.
    Blur(f32),
    /// When set, imgproxy will apply the sharpen filter to the resulting image. The value of [ProcessingOption::Sharpen] defines the size of the mask imgproxy will use.
    ///
    /// As an approximate guideline, use 0.5 sigma for 4 pixels/mm (display resolution), 1.0 for 12 pixels/mm and 1.5 for 16 pixels/mm (300 dpi == 12 pixels/mm).
    Sharpen(f32),
    /// When set, imgproxy will apply the pixelate filter to the resulting image. The value of [ProcessingOption::Pixelate] defines individual pixel size.
    Pixelate(u32),
    /// Places a watermark on the processed image.
    Watermark(WatermarkOptions),
    /// When set to `true`, imgproxy will strip the output images' metadata (EXIF, IPTC, etc.). This is normally controlled by the `IMGPROXY_STRIP_METADATA` configuration but this processing option allows the configuration to be set for each request.
    StripMetadata(bool),
    /// When set to `true`, imgproxy will not remove copyright info while stripping metadata. This is normally controlled by the `IMGPROXY_KEEP_COPYRIGHT` configuration but this processing option allows the configuration to be set for each request.
    KeepCopyright(bool),
    /// When set to `true`, imgproxy will transform the embedded color profile (ICC) to sRGB and remove it from the image. Otherwise, imgproxy will try to keep it as is. This is normally controlled by the `IMGPROXY_STRIP_COLOR_PROFILE` configuration but this processing option allows the configuration to be set for each request.
    StripColorProfile(bool),
    /// When set to `true` and the source image has an embedded thumbnail, imgproxy will always use the embedded thumbnail instead of the main image. Currently, only thumbnails embedded in heic and avif are supported. This is normally controlled by the `IMGPROXY_ENFORCE_THUMBNAIL` configuration but this processing option allows the configuration to be set for each request.
    EnforceThumbnail(bool),
    /// Redefines quality of the resulting image, as a percentage. When set to `0`, quality is assumed based on `IMGPROXY_QUALITY` and [ProcessingOption::FormatQuality].
    Quality(f32),
    /// Adds or redefines `IMGPROXY_FORMAT_QUALITY` values.
    FormatQuality(Vec<(String, f32)>),
    /// When set, imgproxy automatically degrades the quality of the image until the image size is under the specified amount of bytes.
    ///
    /// # Warning
    /// When [ProcessingOption::MaxBytes] is set, imgproxy saves image multiple times to achieve the specified image size.
    ///
    /// # Info
    /// Applicable only to `jpg`, `webp`, `heic`, and `tiff`.
    MaxBytes(usize),
    /// Specifies the resulting image format. Alias for the [ImageUrl::format] part of the URL.
    Format(String),
    /// When set, imgproxy will skip the processing of the listed formats. Also available as the `IMGPROXY_SKIP_PROCESSING_FORMATS` configuration.
    ///
    /// ## Info
    /// - Processing can only be skipped when the requested format is the same as the source format.
    /// - Video thumbnail processing can't be skipped.
    SkipProcessing(Vec<String>),
    /// When set to `true`, imgproxy will respond with a raw unprocessed, and unchecked source image. There are some differences between [ProcessingOption::Raw] and [ProcessingOption::SkipProcessing] options:
    ///
    /// - While the skip_processing option has some conditions to skip the processing, the [ProcessingOption::Raw] option allows to skip processing no matter what
    /// - With the [ProcessingOption::Raw] option set, imgproxy doesn't check the source image's type, resolution, and file size. Basically, the [ProcessingOption::Raw] option allows streaming of any file type
    /// - With the [ProcessingOption::Raw] option set, imgproxy won't download the whole image to the memory. Instead, it will stream the source image directly to the response lowering memory usage
    /// - The requests with the [ProcessingOption::Raw] option set are not limited by the `IMGPROXY_WORKERS` config
    Raw(bool),
    /// Cache buster doesn't affect image processing but its changing allows for bypassing the CDN, proxy server and browser cache. Useful when you have changed some things that are not reflected in the URL, like image quality settings, presets, or watermark data.
    ///
    /// It's highly recommended to prefer the [ProcessingOption::CacheBuster] option over a URL query string because that option can be properly signed.
    CacheBuster(String),
    /// When set, imgproxy will check the provided unix timestamp and return `404` when expired.
    Expire(u64),
    /// Defines a filename for the `Content-Disposition` header. When not specified, imgproxy will get the filename from the source URL.
    Filename(String),
    /// When set to `true`, imgproxy will return `attachment` in the `Content-Disposition` header, and the browser will open a 'Save as' dialog. This is normally controlled by the `IMGPROXY_RETURN_ATTACHMENT` configuration but this processing option allows the configuration to be set for each request.
    ReturnAttachment(bool),
    /// Defines a list of presets to be used by imgproxy. Feel free to use as many presets in a single URL as you need.
    Preset(Vec<String>),
    /// Allows redefining `IMGPROXY_MAX_SRC_RESOLUTION` config.
    ///
    /// # Warning
    /// Since this option allows redefining a security restriction, its usage is not allowed unless the `IMGPROXY_ALLOW_SECURITY_OPTIONS` config is set to true.
    MaxSrcResolution(u32),
    /// Allows redefining `IMGPROXY_MAX_SRC_FILE_SIZE` config.
    ///
    /// # Warning
    /// Since this option allows redefining a security restriction, its usage is not allowed unless the `IMGPROXY_ALLOW_SECURITY_OPTIONS` config is set to true.
    MaxSrcFileSize(usize),
    /// Allows redefining `IMGPROXY_MAX_ANIMATION_FRAMES` config.
    ///
    /// # Warning
    /// Since this option allows redefining a security restriction, its usage is not allowed unless the `IMGPROXY_ALLOW_SECURITY_OPTIONS` config is set to true.
    MaxAnimationFrames(u32),
    /// Allows redefining `IMGPROXY_MAX_ANIMATION_FRAME_RESOLUTION` config.
    ///
    /// # Warning
    /// Since this option allows redefining a security restriction, its usage is not allowed unless the `IMGPROXY_ALLOW_SECURITY_OPTIONS` config is set to true.
    MaxAnimationFrameResolution(u32),
    /// Allows redefining `IMGPROXY_MAX_RESULT_DIMENSION` config.
    ///
    /// # Warning
    /// Since this option allows redefining a security restriction, its usage is not allowed unless the `IMGPROXY_ALLOW_SECURITY_OPTIONS` config is set to true.
    MaxResultDimension(u32),
}

impl FormatProcessingOption for ProcessingOption {
    fn name(&self) -> &str {
        match self {
            ProcessingOption::Resize(_) => RESIZE,
            ProcessingOption::Size(_) => SIZE,
            ProcessingOption::ResizingType(_) => RESIZING_TYPE,
            ProcessingOption::Width(_) => WIDTH,
            ProcessingOption::Height(_) => HEIGHT,
            ProcessingOption::MinWidth(_) => MIN_WIDTH,
            ProcessingOption::MinHeight(_) => MIN_HEIGHT,
            ProcessingOption::Zoom(_) => ZOOM,
            ProcessingOption::Dpr(_) => DPR,
            ProcessingOption::Enlarge(_) => ENLARGE,
            ProcessingOption::Extend(_) => EXTEND,
            ProcessingOption::ExtendAspectRatio(_) => EXTEND_ASPECT_RATIO,
            ProcessingOption::Gravity(_) => GRAVITY,
            ProcessingOption::Crop(_) => CROP,
            ProcessingOption::Trim(_) => TRIM,
            ProcessingOption::Padding(_) => PADDING,
            ProcessingOption::AutoRotate(_) => AUTO_ROTATE,
            ProcessingOption::Rotate(_) => ROTATE,
            ProcessingOption::Flip(_, _) => FLIP,
            ProcessingOption::Background(_) => BACKGROUND,
            ProcessingOption::Blur(_) => BLUR,
            ProcessingOption::Sharpen(_) => SHARPEN,
            ProcessingOption::Pixelate(_) => PIXELATE,
            ProcessingOption::Watermark(_) => WATERMARK,
            ProcessingOption::StripMetadata(_) => STRIP_METADATA,
            ProcessingOption::KeepCopyright(_) => KEEP_COPYRIGHT,
            ProcessingOption::StripColorProfile(_) => STRIP_COLOR_PROFILE,
            ProcessingOption::EnforceThumbnail(_) => ENFORCE_THUMBNAIL,
            ProcessingOption::Quality(_) => QUALITY,
            ProcessingOption::FormatQuality(_) => FORMAT_QUALITY,
            ProcessingOption::MaxBytes(_) => MAX_BYTES,
            ProcessingOption::Format(_) => FORMAT,
            ProcessingOption::SkipProcessing(_) => SKIP_PROCESSING,
            ProcessingOption::Raw(_) => RAW,
            ProcessingOption::CacheBuster(_) => CACHE_BUSTER,
            ProcessingOption::Expire(_) => EXPIRE,
            ProcessingOption::Filename(_) => FILENAME,
            ProcessingOption::ReturnAttachment(_) => RETURN_ATTACHMENT,
            ProcessingOption::Preset(_) => PRESET,
            ProcessingOption::MaxSrcResolution(_) => MAX_SRC_RESOLUTION,
            ProcessingOption::MaxSrcFileSize(_) => MAX_SRC_FILE_SIZE,
            ProcessingOption::MaxAnimationFrames(_) => MAX_ANIMATION_FRAMES,
            ProcessingOption::MaxAnimationFrameResolution(_) => MAX_ANIMATION_FRAME_RESOLUTION,
            ProcessingOption::MaxResultDimension(_) => MAX_RESULT_DIMENSION,
        }
    }

    fn options(&self) -> String {
        match self {
            ProcessingOption::Resize(options) => options.options(),
            ProcessingOption::Size(options) => options.options(),
            ProcessingOption::ResizingType(mode) => mode.options(),
            ProcessingOption::Width(width) => width.to_string(),
            ProcessingOption::Height(height) => height.to_string(),
            ProcessingOption::MinWidth(min_width) => min_width.to_string(),
            ProcessingOption::MinHeight(min_height) => min_height.to_string(),
            ProcessingOption::Zoom(zoom) => zoom.options(),
            ProcessingOption::Dpr(dpr) => dpr.to_string(),
            ProcessingOption::Enlarge(enlarge) => enlarge.to_string(),
            ProcessingOption::Extend(extend) => extend.to_string(),
            ProcessingOption::ExtendAspectRatio(ear) => ear.to_string(),
            ProcessingOption::Gravity(gravity_options) => gravity_options.options(),
            ProcessingOption::Crop(crop_options) => crop_options.options(),
            ProcessingOption::Trim(trim_options) => trim_options.options(),
            ProcessingOption::Padding(padding_options) => padding_options.options(),
            ProcessingOption::AutoRotate(auto_rotate) => auto_rotate.to_string(),
            ProcessingOption::Rotate(rotate_angle) => rotate_angle.to_string(),
            ProcessingOption::Flip(flip_x, flip_y) => format!("{}:{}", flip_x, flip_y),
            ProcessingOption::Background(color) => {
                if let Some(colour) = color {
                    match colour {
                        Colour::Rgb(r, g, b) => format!("{r}:{g}:{b}"),
                        Colour::Hex(hex) => hex.clone(),
                    }
                } else {
                    String::new()
                }
            }
            ProcessingOption::Blur(blur) => blur.to_string(),
            ProcessingOption::Sharpen(sharpen) => sharpen.to_string(),
            ProcessingOption::Pixelate(pixelate) => pixelate.to_string(),
            ProcessingOption::Watermark(watermark_options) => watermark_options.options(),
            ProcessingOption::StripMetadata(strip_metadata) => strip_metadata.to_string(),
            ProcessingOption::KeepCopyright(keep_copyright) => keep_copyright.to_string(),
            ProcessingOption::StripColorProfile(scp) => scp.to_string(),
            ProcessingOption::EnforceThumbnail(enforce_thumbnail) => enforce_thumbnail.to_string(),
            ProcessingOption::Quality(quality) => quality.to_string(),
            ProcessingOption::FormatQuality(format_quality) => format_quality
                .iter()
                .map(|(fmt, quality)| format!("{fmt}:{quality}"))
                .join(":"),
            ProcessingOption::MaxBytes(max_bytes) => max_bytes.to_string(),
            ProcessingOption::Format(format) => format.clone(),
            ProcessingOption::SkipProcessing(skip_processing) => skip_processing.join(":"),
            ProcessingOption::Raw(raw) => raw.to_string(),
            ProcessingOption::CacheBuster(cache_buster) => cache_buster.clone(),
            ProcessingOption::Expire(expire) => expire.to_string(),
            ProcessingOption::Filename(filename) => filename.clone(),
            ProcessingOption::ReturnAttachment(return_attachment) => return_attachment.to_string(),
            ProcessingOption::Preset(presets) => presets.join(":"),
            ProcessingOption::MaxSrcResolution(msr) => msr.to_string(),
            ProcessingOption::MaxSrcFileSize(max_src_file_size) => max_src_file_size.to_string(),
            ProcessingOption::MaxAnimationFrames(maf) => maf.to_string(),
            ProcessingOption::MaxAnimationFrameResolution(mafr) => mafr.to_string(),
            ProcessingOption::MaxResultDimension(mrd) => mrd.to_string(),
        }
    }
}

/// # Resize
///
/// ```
/// resize:%resizing_type:%width:%height:%enlarge:%extend
/// rs:%resizing_type:%width:%height:%enlarge:%extend
/// ```
///
/// This is a meta-option that defines the [ResizeMode], width, height, enlarge, and extend. All arguments are optional and can be omitted to use their default values.
#[derive(Builder, Debug, Default)]
#[builder(pattern = "owned", setter(into), default)]
pub struct ResizingOptions {
    pub mode: Option<ResizeMode>,
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub enlarge: Option<bool>,
    pub extend: Option<bool>,
}

impl FormatProcessingOption for ResizingOptions {
    fn name(&self) -> &str {
        RESIZE
    }
    fn options(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.mode
                .as_ref()
                .map(FormatProcessingOption::options)
                .unwrap_or_default(),
            self.width.map(|w| w.to_string()).unwrap_or_default(),
            self.height.map(|h| h.to_string()).unwrap_or_default(),
            self.enlarge.map(|e| e.to_string()).unwrap_or_default(),
            self.extend.map(|e| e.to_string()).unwrap_or_default()
        )
    }
}

/// # Size
/// ```
/// size:%width:%height:%enlarge:%extend
/// s:%width:%height:%enlarge:%extend
/// ```
///
/// This is a meta-option that defines the width, height, enlarge, and extend. All arguments are optional and can be omitted to use their default values.
#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct SizeOptions {
    pub width: Option<u32>,
    pub height: Option<u32>,
    pub enlarge: Option<bool>,
    pub extend: Option<bool>,
}

impl FormatProcessingOption for SizeOptions {
    fn name(&self) -> &str {
        "s"
    }
    fn options(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.width.map(|w| w.to_string()).unwrap_or_default(),
            self.height.map(|h| h.to_string()).unwrap_or_default(),
            self.enlarge.map(|e| e.to_string()).unwrap_or_default(),
            self.extend.map(|e| e.to_string()).unwrap_or_default()
        )
    }
}

#[derive(Default, Debug)]
pub enum ResizeMode {
    /// resizes the image while keeping aspect ratio to fit a given size.
    #[default]
    Fit,
    /// resizes the image while keeping aspect ratio to fill a given size and crops projecting parts.
    Fill,
    /// the same as [ResizeMode::Fill], but if the resized image is smaller than the requested size, imgproxy will crop the result to keep the requested aspect ratio.
    FillDown,
    /// resizes the image without keeping the aspect ratio.
    Force,
    /// if both source and resulting dimensions have the same orientation (portrait or landscape), imgproxy will use [ResizeMode::Fill]. Otherwise, it will use [ResizeMode::Fit].
    Auto,
}

impl FormatProcessingOption for ResizeMode {
    fn name(&self) -> &str {
        RESIZING_TYPE
    }

    fn options(&self) -> String {
        match self {
            ResizeMode::Fit => "fit",
            ResizeMode::Fill => "fill",
            ResizeMode::FillDown => "fill-down",
            ResizeMode::Force => "force",
            ResizeMode::Auto => "auto",
        }
        .to_string()
    }
}

/// # Zoom
/// ```
/// zoom:%zoom_x_y
/// z:%zoom_x_y
///
/// zoom:%zoom_x:%zoom_y
/// z:%zoom_x:%zoom_y
/// ```
///
/// When set, imgproxy will multiply the image dimensions according to these factors. The values must be greater than 0.
///
/// Can be combined with width and height options. In this case, imgproxy calculates scale factors for the provided size and then multiplies it with the provided zoom factors.
#[derive(Debug)]
pub enum Zoom {
    Uniform(u32),
    Xy(u32, u32),
}

impl FormatProcessingOption for Zoom {
    fn name(&self) -> &str {
        ZOOM
    }

    fn options(&self) -> String {
        match self {
            Zoom::Uniform(factor) => factor.to_string(),
            Zoom::Xy(x_factor, y_factor) => format!("{}:{}", x_factor, y_factor),
        }
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct GravityOptions {
    /// specifies the gravity type
    pub gravity: Gravity,
    /// specifies the gravity offset along the X axis
    /// When greater or equal to 1: Absolute offset in pixels.
    /// When less than 1: Relative value
    #[builder(default)]
    pub x_offset: Option<i32>,
    /// specifies the gravity offset along the Y axis
    /// When greater or equal to 1: Absolute offset in pixels.
    /// When less than 1: Relative value
    #[builder(default)]
    pub y_offset: Option<i32>,
}

impl FormatProcessingOption for GravityOptions {
    fn name(&self) -> &str {
        GRAVITY
    }

    fn options(&self) -> String {
        format!(
            "{}:{}:{}",
            self.gravity,
            self.x_offset.map(|x| x.to_string()).unwrap_or_default(),
            self.y_offset.map(|y| y.to_string()).unwrap_or_default()
        )
    }
}

#[derive(Debug)]
pub enum Gravity {
    /// Top edge
    North,
    /// Bottom edge
    South,
    /// Right edge
    East,
    /// Left edge
    West,
    /// Top right corner
    NorthEast,
    /// Top left corner
    NorthWest,
    /// Bottom right corner
    SouthEast,
    /// Bottom left corner
    SouthWest,
    /// Center of the image
    Center,
    /// smart gravity. libvips detects the most "interesting" section of the image and considers it as the center of the resulting image. Offsets are not applicable here.
    Smart,
}

impl Display for Gravity {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let gravity_str = match self {
            Gravity::North => "no",
            Gravity::South => "so",
            Gravity::East => "ea",
            Gravity::West => "we",
            Gravity::NorthEast => "noea",
            Gravity::NorthWest => "nowe",
            Gravity::SouthEast => "soea",
            Gravity::SouthWest => "sowe",
            Gravity::Center => "ce",
            Gravity::Smart => "sm",
        };
        write!(f, "{}", gravity_str)
    }
}

#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct CropOptions {
    pub width: f32,
    pub height: f32,
    /// accepts the same values as the [GravityOptions]. When [CropOptions::gravity] is not set, imgproxy will use the value of the [ProcessingOption::Gravity] option.
    pub gravity: Option<GravityOptions>,
}

impl FormatProcessingOption for CropOptions {
    fn name(&self) -> &str {
        CROP
    }

    fn options(&self) -> String {
        format!(
            "{}:{}:{}",
            self.width,
            self.height,
            self.gravity
                .as_ref()
                .map(|g| g.options())
                .unwrap_or_default()
        )
    }
}

/// # Trim
/// ```
/// trim:%threshold:%color:%equal_hor:%equal_ver
/// t:%threshold:%color:%equal_hor:%equal_ver
/// ```
///
/// Removes surrounding background.
///
/// ## Warning
/// Trimming requires an image to be fully loaded into memory. This disables scale-on-load and significantly increases memory usage and processing time. Use it carefully with large images.
///
/// ## Info
/// - If you know background color of your images then setting it explicitly via [TrimOptions::color] will also save some resources because imgproxy won't need to automatically detect it.
/// - Use a [TrimOptions::color] value of `FF00FF` for trimming transparent backgrounds as imgproxy uses magenta as a transparency key.
/// - The trimming of animated images is not supported.
#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct TrimOptions {
    /// color similarity tolerance
    pub threshold: u32,
    /// a hex-coded value of the color that needs to be cut off.
    pub color: Option<String>,
    /// set to `true`, imgproxy will cut only equal parts from left and right sides. That means that if 10px of background can be cut off from the left and 5px from the right, then 5px will be cut off from both sides. For example, this can be useful if objects on your images are centered but have non-symmetrical shadow.
    pub equal_hor: Option<bool>,
    /// acts like [TrimOptions::equal_hor] but for top/bottom sides.
    pub equal_ver: Option<bool>,
}

impl FormatProcessingOption for TrimOptions {
    fn name(&self) -> &str {
        TRIM
    }

    fn options(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.threshold,
            self.color.clone().unwrap_or_default(),
            self.equal_hor.map(|e| e.to_string()).unwrap_or_default(),
            self.equal_ver.map(|e| e.to_string()).unwrap_or_default()
        )
    }
}

/// # Padding
/// ```
/// padding:%top:%right:%bottom:%left
/// pd:%top:%right:%bottom:%left
/// ```
///
/// Defines padding size using CSS-style syntax. All arguments are optional but at least one dimension must be set. Padded space is filled according to the background option.
///
/// ## Info
/// - Padding is applied after all image transformations (except watermarking) and enlarges the generated image. This means that if your resize dimensions were 100x200px and you applied the `padding:10` option, then you will end up with an image with dimensions of 120x220px.
/// - Padding follows the [ProcessingOption::Dpr] option so it will also be scaled if you've set it.
#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct PaddingOptions {
    /// Top padding (and for all other sides if they haven't been explicitly set)
    pub top: u32,
    /// right padding (and left if it hasn't been explicitly set)
    pub right: Option<u32>,
    /// bottom padding
    pub bottom: Option<u32>,
    /// left padding
    pub left: Option<u32>,
}

impl FormatProcessingOption for PaddingOptions {
    fn name(&self) -> &str {
        PADDING
    }

    fn options(&self) -> String {
        format!(
            "{}:{}:{}:{}",
            self.top,
            self.right.map(|r| r.to_string()).unwrap_or_default(),
            self.bottom.map(|b| b.to_string()).unwrap_or_default(),
            self.left.map(|l| l.to_string()).unwrap_or_default()
        )
    }
}

#[derive(Debug)]
pub enum RotateAngle {
    Rotate0,
    Rotate90,
    Rotate180,
    Rotate270,
}

impl Display for RotateAngle {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let angle_str = match self {
            RotateAngle::Rotate0 => "0",
            RotateAngle::Rotate90 => "90",
            RotateAngle::Rotate180 => "180",
            RotateAngle::Rotate270 => "270",
        };
        write!(f, "{}", angle_str)
    }
}

#[derive(Debug)]
pub enum Colour {
    Rgb(u8, u8, u8),
    Hex(String),
}

/// # Watermark
/// ```
/// watermark:%opacity:%position:%x_offset:%y_offset:%scale
/// wm:%opacity:%position:%x_offset:%y_offset:%scale
/// ```
///
/// Places a watermark on the processed image.
#[derive(Builder, Debug)]
#[builder(pattern = "owned", setter(into))]
pub struct WatermarkOptions {
    /// watermark opacity modifier. Final opacity is calculated like base_opacity * opacity.
    pub opacity: f32,
    /// specifies the position of the watermark
    pub position: Option<WatermarkPosition>,
    pub x_offset: Option<f32>,
    pub y_offset: Option<f32>,
    /// a floating-point number that defines the watermark size relative to the resultant image size. When set to 0 or when omitted, the watermark size won't be changed.
    pub scale: Option<f32>,
}

impl FormatProcessingOption for WatermarkOptions {
    fn name(&self) -> &str {
        WATERMARK
    }

    fn options(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}",
            self.opacity,
            self.position
                .as_ref()
                .map(|p| p.to_string())
                .unwrap_or_default(),
            self.x_offset.map(|x| x.to_string()).unwrap_or_default(),
            self.y_offset.map(|y| y.to_string()).unwrap_or_default(),
            self.scale.map(|s| s.to_string()).unwrap_or_default()
        )
    }
}

/// specifies the position of the watermark
#[derive(Default, Debug)]
pub enum WatermarkPosition {
    /// Center of the image
    #[default]
    Center,
    /// Top edge
    North,
    /// Bottom edge
    South,
    /// Right edge
    East,
    /// Left edge
    West,
    /// Top right corner
    NorthEast,
    /// Top left corner
    NorthWest,
    /// Bottom right corner
    SouthEast,
    /// Bottom left corner
    SouthWest,
    /// repeat and tile the watermark to fill the entire image
    Repeat,
}

impl Display for WatermarkPosition {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let position_str = match self {
            WatermarkPosition::Center => "ce",
            WatermarkPosition::North => "no",
            WatermarkPosition::South => "so",
            WatermarkPosition::East => "ea",
            WatermarkPosition::West => "we",
            WatermarkPosition::NorthEast => "noea",
            WatermarkPosition::NorthWest => "nowe",
            WatermarkPosition::SouthEast => "soea",
            WatermarkPosition::SouthWest => "sowe",
            WatermarkPosition::Repeat => "re",
        };
        write!(f, "{}", position_str)
    }
}

#[derive(Clone)]
pub struct SignedUrlRepo {
    salt: String,
    secret_key: String,
    prefix: String,
}

impl SignedUrlRepo {
    pub fn new(salt: String, secret_key: String, prefix: String) -> Self {
        Self {
            salt,
            secret_key,
            prefix,
        }
    }

    pub fn get(&self, image_url: &ImageUrl) -> Result<String> {
        let signed_url = self.sign_url(image_url);
        Ok(signed_url)
    }

    fn sign_url(&self, image_url: &ImageUrl) -> String {
        let original_url = image_url.to_string();
        let salted_url = format!("{}/{original_url}", self.salt);

        let mut mac = Hmac::<Sha256>::new_from_slice(self.secret_key.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(salted_url.as_bytes());
        let signature = mac.finalize();
        let signature = BASE64_URL_SAFE_NO_PAD.encode(signature.into_bytes());

        format!("/{}/{signature}/{original_url}", self.prefix)
    }
}
