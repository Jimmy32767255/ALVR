use alvr_common::{
    ALVR_VERSION, DebugGroupsConfig, DebugGroupsConfigDefault, LogSeverity, LogSeverityDefault,
    LogSeverityDefaultVariant,
};
use alvr_system_info::{ClientFlavor, ClientFlavorDefault, ClientFlavorDefaultVariant};
use bytemuck::{Pod, Zeroable};
use serde::{Deserialize, Serialize};
use settings_schema::{
    ArrayDefault, DictionaryDefault, OptionalDefault, SettingsSchema, Switch, SwitchDefault,
    VectorDefault,
};

include!(concat!(env!("OUT_DIR"), "/openvr_property_keys.rs"));

pub enum OpenvrPropType {
    Bool,
    Float,
    Int32,
    Uint64,
    Vector3,
    Double,
    String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Debug)]
pub struct OpenvrProperty {
    pub key: OpenvrPropKey,
    pub value: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum FrameSize {
    Scale(#[schema(gui(slider(min = 0.25, max = 2.0, step = 0.01)))] f32),

    Absolute {
        #[schema(gui(slider(min = 32, max = 8192, step = 32)))]
        width: u32,
        #[schema(gui(slider(min = 32, max = 8192, step = 32)))]
        height: Option<u32>,
    },
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum EncoderQualityPreset {
    Quality = 0,
    Balanced = 1,
    Speed = 2,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum EncoderQualityPresetNvidia {
    P1 = 1,
    P2 = 2,
    P3 = 3,
    P4 = 4,
    P5 = 5,
    P6 = 6,
    P7 = 7,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum NvencTuningPreset {
    HighQuality = 1,
    LowLatency = 2,
    UltraLowLatency = 3,
    Lossless = 4,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum NvencMultiPass {
    Disabled = 0,
    #[schema(strings(display_name = "1/4 分辨率"))]
    QuarterResolution = 1,
    FullResolution = 2,
}

#[repr(u32)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum NvencAdaptiveQuantizationMode {
    Disabled = 0,
    Spatial = 1,
    Temporal = 2,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(gui = "button_group")]
pub enum RateControlMode {
    #[schema(strings(display_name = "CBR (固定比特率)"))]
    Cbr = 0,
    #[schema(strings(display_name = "VBR (可变比特率)"))]
    Vbr = 1,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(gui = "button_group")]
pub enum EntropyCoding {
    #[schema(strings(display_name = "CAVLC (上下文自适应可变长度编码)"))]
    Cavlc = 1,
    #[schema(strings(display_name = "CABAC (上下文自适应二进制算术编码)"))]
    Cabac = 0,
}

/// Except for preset, the value of these fields is not applied if == -1 (flag)
#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct NvencConfig {
    #[schema(strings(
        help = "P1 是最快的预设，P7 是生成更好质量的预设。P6 和 P7 太慢，无法使用。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub quality_preset: EncoderQualityPresetNvidia,
    #[schema(flag = "steamvr-restart")]
    pub tuning_preset: NvencTuningPreset,
    #[schema(strings(
        help = "以牺牲少量性能为代价减少压缩伪影"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub multi_pass: NvencMultiPass,
    #[schema(strings(
        help = r#"空间：有助于减少色彩条带，但高复杂度场景可能看起来更糟。
时间：有助于提高整体编码质量，速度上只有很小的权衡。"#
    ))]
    #[schema(flag = "steamvr-restart")]
    pub adaptive_quantization_mode: NvencAdaptiveQuantizationMode,
    #[schema(flag = "steamvr-restart")]
    pub low_delay_key_frame_scale: i64,
    #[schema(flag = "steamvr-restart")]
    pub refresh_rate: i64,
    #[schema(flag = "steamvr-restart")]
    pub enable_intra_refresh: bool,
    #[schema(flag = "steamvr-restart")]
    pub intra_refresh_period: i64,
    #[schema(flag = "steamvr-restart")]
    pub intra_refresh_count: i64,
    #[schema(flag = "steamvr-restart")]
    pub max_num_ref_frames: i64,
    #[schema(flag = "steamvr-restart")]
    pub gop_length: i64,
    #[schema(flag = "steamvr-restart")]
    pub p_frame_strategy: i64,
    #[schema(flag = "steamvr-restart")]
    pub rate_control_mode: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_buffer_size: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_initial_delay: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_max_bitrate: i64,
    #[schema(flag = "steamvr-restart")]
    pub rc_average_bitrate: i64,
    #[schema(flag = "steamvr-restart")]
    pub enable_weighted_prediction: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct AmfConfig {
    #[schema(
        strings(
            display_name = "启用高动态质量提升",
            help = r#"启用高动态质量提升模式。
允许编码器对视频的运动进行预分析，并利用这些信息进行更好的编码"#
        ),
        flag = "steamvr-restart"
    )]
    pub enable_hmqb: bool,
    #[schema(flag = "steamvr-restart")]
    pub use_preproc: bool,
    #[schema(gui(slider(min = 0, max = 10)))]
    #[schema(flag = "steamvr-restart")]
    pub preproc_sigma: u32,
    #[schema(gui(slider(min = 0, max = 10)))]
    #[schema(flag = "steamvr-restart")]
    pub preproc_tor: u32,
    #[schema(
        strings(
            display_name = "启用预分析",
            help = r#"在编码期间启用预分析。这可能会导致性能下降，但可能会提高质量。
不适用于“减少色彩条带”选项，需要启用“使用预处理”"#
        ),
        flag = "steamvr-restart"
    )]
    pub enable_pre_analysis: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct SoftwareEncodingConfig {
    #[schema(strings(
        display_name = "强制软件编码",
        help = "强制编码器使用 CPU 而不是 GPU"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub force_software_encoding: bool,

    #[schema(strings(display_name = "编码器线程数"))]
    #[schema(flag = "steamvr-restart")]
    pub thread_count: u32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct HDRConfig {
    #[schema(strings(
        display_name = "启用 HDR",
        help = "如果客户端没有偏好，则启用将 VR 层合成到 RGBA float16 帧缓冲区，并在着色器代码中进行 sRGB/YUV 转换。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub enable: Option<bool>,

    #[schema(strings(
        display_name = "强制 HDR sRGB 校正",
        help = "强制对所有合成的 SteamVR 层进行 sRGB 校正。如果 HDR 注入的游戏太暗，这会很有用。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub force_hdr_srgb_correction: bool,

    #[schema(strings(
        display_name = "钳制 HDR 扩展范围",
        help = "将 HDR 扩展范围钳制到 0.0~1.0，如果您只想通过 HDR 减少条带，这会很有用。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub clamp_hdr_extended_range: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct EncoderConfig {
    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        display_name = "质量预设",
        help = "控制编码器的整体质量预设。仅适用于 Windows AMD AMF、Linux VAAPI (AMD/Intel)。"
    ))]
    pub quality_preset: EncoderQualityPreset,

    #[schema(
        strings(
            display_name = "启用 VBAQ/CAQ",
            help = "在 h264 和 HEVC 上启用基于方差的自适应量化，在 AV1 上启用内容自适应量化"
        ),
        flag = "steamvr-restart"
    )]
    pub enable_vbaq: bool,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(help = r#"CBR: 固定比特率模式。推荐使用此模式。
VBR: 可变比特率模式。不推荐使用，因为它可能会干扰自适应比特率算法。此模式仅在 Windows 上受支持，并且仅适用于 AMD/Nvidia GPU"#))]
    #[schema(flag = "steamvr-restart")]
    pub rate_control_mode: RateControlMode,

    #[schema(strings(
        display_name = "h264: 配置文件",
        help = "尽可能尝试使用此配置文件。可能会增加与各种移动设备的兼容性。仅对 h264 有效。不影响 Windows 上的 NVENC。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub h264_profile: H264Profile,

    #[schema(strings(help = r#"推荐使用 CAVLC 算法。
CABAC 产生更好的压缩效果，但速度明显较慢，并可能导致延迟失控"#))]
    #[schema(flag = "steamvr-restart")]
    pub entropy_coding: EntropyCoding,

    #[schema(strings(
        help = r#"在 CBR 模式下，这确保比特率不会低于指定值。这主要用于调试。"#
    ))]
    #[schema(flag = "steamvr-restart")]
    pub filler_data: bool,

    #[schema(strings(
        display_name = "10 位编码",
        help = "如果客户端没有偏好，则将编码器设置为使用每通道 10 位而不是 8 位。不适用于 Linux 上的 Nvidia"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub use_10bit: Option<bool>,

    #[schema(strings(
        display_name = "编码伽马",
        help = "为了优先处理较暗的像素，但可能会在中色调中增加条带，请设置为 2.2。要让编码器自行决定优先级，请设置为 1.0。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub encoding_gamma: Option<f32>,

    #[schema(strings(display_name = "HDR"))]
    #[schema(flag = "steamvr-restart")]
    pub hdr: HDRConfig,

    #[schema(strings(display_name = "NVIDIA 编码器 (NVENC)"))]
    #[schema(flag = "steamvr-restart")]
    pub nvenc: NvencConfig,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(display_name = "AMD 媒体框架 (AMF)"))]
    #[schema(flag = "steamvr-restart")]
    pub amf: AmfConfig,

    #[schema(strings(display_name = "软件 (CPU) 编码"))]
    pub software: SoftwareEncodingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub enum MediacodecPropType {
    Float,
    Int32,
    Int64,
    String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Debug, PartialEq)]
pub struct MediacodecProperty {
    #[schema(strings(display_name = "类型"))]
    pub ty: MediacodecPropType,
    pub value: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct EncoderLatencyLimiter {
    #[schema(strings(
        help = "允许分配给视频编码的帧间隔百分比"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.3, max = 1.0, step = 0.01)))]
    pub max_saturation_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct DecoderLatencyLimiter {
    #[schema(strings(
        display_name = "最大解码器延迟",
        help = "当解码器延迟超过此阈值时，比特率将降低"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1, max = 50)), suffix = "ms")]
    pub max_decoder_latency_ms: u64,

    #[schema(strings(
        display_name = "延迟超限",
        help = "连续帧数超过阈值以触发比特率降低"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1, max = 100)), suffix = " 帧")]
    pub latency_overstep_frames: usize,

    #[schema(strings(
        help = "控制当解码器延迟超过阈值时比特率降低的程度"
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.5, max = 1.0)))]
    pub latency_overstep_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(gui = "button_group")]
pub enum BitrateMode {
    #[schema(strings(display_name = "恒定"))]
    ConstantMbps(#[schema(gui(slider(min = 5, max = 1000, logarithmic)), suffix = "Mbps")] u64),

    #[schema(collapsible)]
    Adaptive {
        #[schema(strings(
            help = "Percentage of network bandwidth to allocate for video transmission"
        ))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 0.5, max = 5.0, step = 0.01)))]
        saturation_multiplier: f32,

        #[schema(strings(display_name = "最大比特率"))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 1, max = 1000, logarithmic)), suffix = "Mbps")]
        max_throughput_mbps: Switch<u64>,

        #[schema(strings(display_name = "最小比特率"))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 1, max = 100, logarithmic)), suffix = "Mbps")]
        min_throughput_mbps: Switch<u64>,

        #[schema(strings(display_name = "最大网络延迟"))]
        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 1, max = 50)), suffix = "ms")]
        max_network_latency_ms: Switch<u64>,

        #[schema(flag = "real-time")]
        encoder_latency_limiter: Switch<EncoderLatencyLimiter>,

        #[schema(strings(
            help = "Currently there is a bug where the decoder latency keeps rising when above a certain bitrate"
        ))]
        #[schema(flag = "real-time")]
        decoder_latency_limiter: Switch<DecoderLatencyLimiter>,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct BitrateAdaptiveFramerateConfig {
    #[schema(strings(
        display_name = "FPS 重置阈值乘数",
        help = "如果帧率变化超过此因子，则触发参数更新",
    ))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1.0, max = 3.0, step = 0.1)))]
    pub framerate_reset_threshold_multiplier: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct BitrateConfig {
    #[schema(flag = "real-time")]
    pub mode: BitrateMode,

    #[schema(strings(help = "确保无论帧率如何变化，指定的比特率值都能被严格遵守"
    ))]
    #[schema(flag = "real-time")]
    pub adapt_to_framerate: Switch<BitrateAdaptiveFramerateConfig>,

    #[schema(strings(help = "控制计算过程中的平滑度"))]
    pub history_size: usize,

    #[schema(strings(
        help = "启用此选项后，比特率变化后会请求IDR帧。\n此功能仅对AMD显卡有效。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub image_corruption_fix: bool,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Copy, Clone, PartialEq)]
pub enum ClientsideFoveationLevel {
    Low = 1,
    Medium = 2,
    High = 3,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub enum ClientsideFoveationMode {
    Static { level: ClientsideFoveationLevel },
    Dynamic { max_level: ClientsideFoveationLevel },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct ClientsideFoveationConfig {
    pub mode: ClientsideFoveationMode,

    #[schema(strings(display_name = "注视点偏移"))]
    #[schema(gui(slider(min = -45.0, max = 45.0, step = 0.1)), suffix = "°")]
    pub vertical_offset_deg: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(collapsible)]
pub struct FoveatedEncodingConfig {
    #[schema(strings(help = "强制在智能手机客户端上启用"))]
    pub force_enable: bool,

    #[schema(strings(display_name = "中心区域宽度"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_size_x: f32,

    #[schema(strings(display_name = "中心区域高度"))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_size_y: f32,

    #[schema(strings(display_name = "中心偏移X"))]
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_shift_x: f32,

    #[schema(strings(display_name = "中心偏移Y"))]
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub center_shift_y: f32,

    #[schema(strings(display_name = "水平边缘比例"))]
    #[schema(gui(slider(min = 1.0, max = 10.0, step = 1.0)))]
    #[schema(flag = "steamvr-restart")]
    pub edge_ratio_x: f32,

    #[schema(strings(display_name = "垂直边缘比例"))]
    #[schema(gui(slider(min = 1.0, max = 10.0, step = 1.0)))]
    #[schema(flag = "steamvr-restart")]
    pub edge_ratio_y: f32,
}

#[repr(C)]
#[derive(SettingsSchema, Clone, Copy, Serialize, Deserialize, Pod, Zeroable)]
pub struct ColorCorrectionConfig {
    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.001)))]
    #[schema(flag = "steamvr-restart")]
    pub brightness: f32,

    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.001)))]
    #[schema(flag = "steamvr-restart")]
    pub contrast: f32,

    #[schema(gui(slider(min = -1.0, max = 1.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub saturation: f32,

    #[schema(gui(slider(min = 0.0, max = 5.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub gamma: f32,

    #[schema(gui(slider(min = -1.0, max = 5.0, step = 0.01)))]
    #[schema(flag = "steamvr-restart")]
    pub sharpening: f32,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq, Default)]
#[schema(gui = "button_group")]
pub enum CodecType {
    #[default]
    #[schema(strings(display_name = "H.264 (AVC)"))]
    H264 = 0,
    #[schema(strings(display_name = "H.265 (HEVC)"))]
    Hevc = 1,
    #[schema(strings(display_name = "AV1"))]
    AV1 = 2,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[schema(gui = "button_group")]
pub enum H264Profile {
    #[schema(strings(display_name = "高"))]
    High = 0,
    #[schema(strings(display_name = "主"))]
    Main = 1,
    #[schema(strings(display_name = "基线"))]
    Baseline = 2,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct RgbChromaKeyConfig {
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0, max = 255)))]
    pub red: u8,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0, max = 255)))]
    pub green: u8,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0, max = 255)))]
    pub blue: u8,

    #[schema(strings(help = "The threshold is applied per-channel"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 1, max = 255)))]
    pub distance_threshold: u8,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.01, max = 1.0, step = 0.01)))]
    pub feathering: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct HsvChromaKeyConfig {
    #[schema(strings(display_name = "色调起始最大值"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_start_max_deg: f32,

    #[schema(strings(display_name = "色调起始最小值"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_start_min_deg: f32,

    #[schema(strings(display_name = "色调结束最小值"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_end_min_deg: f32,

    #[schema(strings(display_name = "色调结束最大值"), suffix = "°")]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -179.0, max = 539.0, step = 1.0)))]
    pub hue_end_max_deg: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub saturation_start_max: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub saturation_start_min: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub saturation_end_min: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5,step = 0.01)))]
    pub saturation_end_max: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_start_max: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_start_min: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_end_min: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -0.5, max = 1.5, step = 0.01)))]
    pub value_end_max: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
#[schema(gui = "button_group")]
pub enum PassthroughMode {
    Blend {
        #[schema(strings(
        help = "启用此选项将根据每个像素的亮度调整透明度。\n效果类似于AR眼镜。"
    ))]
    #[schema(flag = "real-time")]
    premultiplied_alpha: bool,

        #[schema(flag = "real-time")]
        #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
        threshold: f32,
    },

    #[schema(strings(display_name = "RGB色度键"))]
    RgbChromaKey(#[schema(flag = "real-time")] RgbChromaKeyConfig),

    #[schema(strings(display_name = "HSV色度键"))]
    HsvChromaKey(#[schema(flag = "real-time")] HsvChromaKeyConfig),
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[schema(gui = "button_group")]
pub enum ClientsidePostProcessingSuperSamplingMode {
    Disabled = 0,
    Normal = 1 << 0,
    Quality = 1 << 1,
}

#[repr(u8)]
#[derive(SettingsSchema, Serialize, Deserialize, Debug, Copy, Clone, PartialEq, Eq)]
#[schema(gui = "button_group")]
pub enum ClientsidePostProcessingSharpeningMode {
    Disabled = 0,
    Normal = 1 << 2,
    Quality = 1 << 3,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct ClientsidePostProcessingConfig {
    #[schema(strings(
        help = "减少高对比度边缘的闪烁。\n当输入分辨率高于头显显示分辨率时特别有用"
    ))]
    pub super_sampling: ClientsidePostProcessingSuperSamplingMode,
    #[schema(strings(
        help = "提高高对比度边缘的清晰度并抵消模糊效果。\n当输入分辨率低于头显显示分辨率时特别有用"
    ))]
    pub sharpening: ClientsidePostProcessingSharpeningMode,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Debug)]
pub struct UpscalingConfig {
    #[schema(strings(
        help = "通过使用边缘方向进行放大来提高视觉质量，但会略微降低性能"
    ))]
    pub edge_direction: bool,
    #[schema(gui(slider(min = 1.0, max = 16.0, step = 1.0)))]
    pub edge_threshold: f32,
    #[schema(gui(slider(min = 1.0, max = 2.0, step = 0.01)))]
    pub edge_sharpness: f32,
    #[schema(gui(slider(min = 1.0, max = 3.0, step = 0.01)))]
    #[schema(strings(
        help = "维度分辨率乘数，高值会导致性能问题，特别是在硬件较弱或分辨率较高的情况下"
    ))]
    pub upscale_factor: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct VideoConfig {
    #[schema(flag = "real-time")]
    pub passthrough: Switch<PassthroughMode>,

    pub bitrate: BitrateConfig,

    #[schema(strings(
        help = "HEVC可能会提供更好的视觉保真度，但会增加编码器延迟"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub preferred_codec: CodecType,

    #[schema(strings(
        notice = r"禁用注视点编码可能会导致编码/解码时间显著增加，并出现卡顿甚至崩溃。\n如果您想减少边缘的像素化，请增加中心区域的宽度和高度"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub foveated_encoding: Switch<FoveatedEncodingConfig>,

    #[schema(flag = "steamvr-restart")]
    pub color_correction: Switch<ColorCorrectionConfig>,

    #[schema(
        strings(
            display_name = "最大缓冲",
            help = "增加此值有助于减少卡顿，但会增加延迟"
        ),
        gui(slider(min = 1.0, max = 10.0, step = 0.1, logarithmic)),
        suffix = " 帧"
    )]
    pub max_buffering_frames: f32,

    #[schema(gui(slider(min = 0.50, max = 0.99, step = 0.01)))]
    pub buffering_history_weight: f32,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(
        help = r"此功能仅在Windows上有效。除非您确定VR游戏无法达到目标帧率，否则不应禁用此功能。"
    ))]
    #[schema(flag = "real-time")]
    pub enforce_server_frame_pacing: bool,

    #[schema(flag = "steamvr-restart")]
    pub encoder_config: EncoderConfig,

    #[schema(strings(
        help = "尝试在设备上使用软件解码器。速度较慢，但可以解决损坏的编解码器问题。"
    ))]
    pub force_software_decoder: bool,

    pub mediacodec_extra_options: Vec<(String, MediacodecProperty)>,

    #[schema(strings(
        help = "用于编码和解码的分辨率。相对于单眼视图。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub transcoding_view_resolution: FrameSize,

    #[schema(strings(
        help = "这是SteamVR将用作游戏渲染默认分辨率的设置。相对于单眼视图。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub emulated_headset_view_resolution: FrameSize,

    #[schema(strings(display_name = "首选帧率"))]
    #[schema(gui(slider(min = 60.0, max = 120.0)), suffix = "Hz")]
    #[schema(flag = "steamvr-restart")]
    pub preferred_fps: f32,

    #[cfg_attr(not(target_os = "windows"), schema(flag = "hidden"))]
    #[schema(strings(
        help = "您可能不想更改此项。允许更改ALVR合成器的适配器。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub adapter_index: u32,

    pub clientside_foveation: Switch<ClientsideFoveationConfig>,

    #[schema(strings(
        display_name = "客户端后处理",
        help = "硬件优化算法，适用于Quest和Pico头显"
    ))]
    #[schema(flag = "real-time")]
    pub clientside_post_processing: Switch<ClientsidePostProcessingConfig>,

    #[schema(strings(help = "骁龙游戏超分辨率客户端侧超采样"))]
    pub upscaling: Switch<UpscalingConfig>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(gui = "button_group")]
pub enum CustomAudioDeviceConfig {
    #[schema(strings(display_name = "按名称（子字符串）"))]
    NameSubstring(String),
    #[schema(strings(display_name = "按索引"))]
    Index(usize),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct AudioBufferingConfig {
    #[schema(strings(display_name = "平均缓冲"))]
    #[schema(gui(slider(min = 0, max = 200)), suffix = "ms")]
    pub average_buffering_ms: u64,

    #[schema(strings(display_name = "批处理大小"))]
    #[schema(gui(slider(min = 1, max = 20)), suffix = "ms")]
    pub batch_ms: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct GameAudioConfig {
    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    pub device: Option<CustomAudioDeviceConfig>,

    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    #[schema(strings(display_name = "串流时静音桌面音频"))]
    pub mute_when_streaming: bool,

    pub buffering: AudioBufferingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum MicrophoneDevicesConfig {
    Automatic,
    #[schema(strings(display_name = "虚拟音频线"))]
    VAC,
    #[schema(strings(display_name = "VB音频线"))]
    VBCable,
    #[schema(strings(display_name = "香蕉语音"))]
    VoiceMeeter,
    #[schema(strings(display_name = "香蕉语音辅助"))]
    VoiceMeeterAux,
    #[schema(strings(display_name = "香蕉语音VAIO3"))]
    VoiceMeeterVaio3,
    Custom {
        #[schema(strings(help = "ALVR用于输出麦克风音频的设备"))]
        sink: CustomAudioDeviceConfig,
        #[schema(strings(help = "在SteamVR中设置为默认麦克风的设备"))]
        source: CustomAudioDeviceConfig,
    },
}

// Note: sample rate is a free parameter for microphone, because both server and client supports
// resampling. In contrary, for game audio, the server does not support resampling.
#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct MicrophoneConfig {
    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    pub devices: MicrophoneDevicesConfig,

    pub buffering: AudioBufferingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct AudioConfig {
    #[schema(strings(display_name = "头显扬声器"))]
    pub game_audio: Switch<GameAudioConfig>,

    #[cfg_attr(
        windows,
        schema(strings(
            display_name = "Headset microphone",
            notice = r"To be able to use the microphone on Windows, you need to install Virtual Audio Cable"
        ))
    )]
    #[cfg_attr(not(windows), schema(strings(display_name = "头显麦克风")))]
    pub microphone: Switch<MicrophoneConfig>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum HeadsetEmulationMode {
    #[schema(strings(display_name = "Rift S (头显)"))]
    RiftS,
    #[schema(strings(display_name = "Quest 2 (头显)"))]
    Quest2,
    #[schema(strings(display_name = "Quest Pro (头显)"))]
    QuestPro,
    Vive,
    Custom {
        serial_number: String,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct FaceTrackingSourcesConfig {
    pub eye_tracking_fb: bool,
    pub face_tracking_fb: bool,
    pub eye_expressions_htc: bool,
    pub lip_expressions_htc: bool,
    pub face_tracking_pico: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum FaceTrackingSinkConfig {
    #[schema(strings(display_name = "VRChat 眼部 OSC"))]
    VrchatEyeOsc { port: u16 },
    #[schema(strings(display_name = "VRCFaceTracking (面部追踪)"))]
    VrcFaceTracking,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct FaceTrackingConfig {
    pub sources: FaceTrackingSourcesConfig,
    pub sink: FaceTrackingSinkConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct BodyTrackingSourcesConfig {
    pub body_tracking_fb: Switch<BodyTrackingFBConfig>,
    #[schema(strings(
        help = "建议在Pico头显的Motion Tracker应用设置中将跟踪模式设置为全身跟踪。"
    ))]
    pub body_tracking_bd: Switch<BodyTrackingBDConfig>,
    // todo:
    // pub detached_controllers_as_feet: bool,
    // unfortunately multimodal is incompatible with body tracking. To make this usable we need to
    // at least add support for an android client as 3dof waist tracker.
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
pub struct BodyTrackingFBConfig {
    pub full_body: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq)]
#[schema(gui = "button_group")]
pub enum BodyTrackingBDConfig {
    #[schema(strings(display_name = "身体追踪 (OpenVR)"))]
    BodyTracking {
        #[schema(strings(
            help = "提高跟踪精度，但会增加延迟。"
        ))]
        high_accuracy: bool,
        #[schema(strings(
            help = "如果跟踪器之前未校准，连接到串流器后将开始校准过程。"
        ))]
        prompt_calibration_on_start: bool,
    },
    #[schema(strings(display_name = "对象追踪 (OpenVR)"))]
    ObjectTracking,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum BodyTrackingSinkConfig {
    #[schema(strings(display_name = "模拟 Vive 追踪器"))]
    FakeViveTracker,
    #[schema(strings(display_name = "VRChat 身体 OSC"))]
    VrchatBodyOsc { port: u16 },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct BodyTrackingConfig {
    pub sources: BodyTrackingSourcesConfig,
    pub sink: BodyTrackingSinkConfig,
    #[schema(strings(help = "关闭此项可暂时暂停跟踪。"))]
    #[schema(flag = "real-time")]
    pub tracked: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct VMCConfig {
    pub host: String,
    pub port: u16,
    #[schema(strings(help = "关闭此项可暂时暂停发送数据。"))]
    #[schema(flag = "real-time")]
    pub publish: bool,
    #[schema(flag = "real-time")]
    pub orientation_correction: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, PartialEq, Eq)]
pub enum ControllersEmulationMode {
    #[schema(strings(display_name = "Rift S Touch (控制器)"))]
    RiftSTouch,
    #[schema(strings(display_name = "Quest 2 Touch (控制器)"))]
    Quest2Touch,
    #[schema(strings(display_name = "Quest 3 Touch Plus (控制器)"))]
    Quest3Plus,
    #[schema(strings(display_name = "Quest Pro (头显)"))]
    QuestPro,
    #[schema(strings(display_name = "Pico 4 (头显)"))]
    Pico4,
    #[schema(strings(display_name = "Valve Index (控制器)"))]
    ValveIndex,
    #[schema(strings(display_name = "Vive Wand (控制器)"))]
    ViveWand,
    #[schema(strings(display_name = "Vive 追踪器"))]
    ViveTracker,
    Custom {
        serial_number: String,
        button_set: Vec<String>,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub struct HysteresisThreshold {
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub value: f32,
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub deviation: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub struct BinaryToScalarStates {
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub off: f32,
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub on: f32,
}

// Remaps 0..1 to custom range
#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub struct Range {
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub min: f32,
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub max: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub enum ButtonMappingType {
    Passthrough,
    HysteresisThreshold(HysteresisThreshold),
    BinaryToScalar(BinaryToScalarStates),
    Remap(Range),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ButtonBindingTarget {
    pub destination: String,
    pub mapping_type: ButtonMappingType,
    pub binary_conditions: Vec<String>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct AutomaticButtonMappingConfig {
    pub click_threshold: HysteresisThreshold,
    pub touch_threshold: HysteresisThreshold,
    pub force_threshold: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HandTrackingInteractionConfig {
    #[schema(flag = "real-time")]
    pub only_touch: bool,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "手指尖需要多近才能注册为捏合点击。"
    ))]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)), suffix = "cm")]
    pub pinch_touch_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "手指尖需要多近才能开始注册为捏合触发器拉动。"
    ))]
    #[schema(gui(slider(min = 0.0, max = 2.5, step = 0.025)), suffix = "cm")]
    pub pinch_trigger_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "手指尖需要多靠近手掌才能注册为弯曲点击。"
    ))]
    #[schema(gui(slider(min = 0.0, max = 5.0)), suffix = "cm")]
    pub curl_touch_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "手指尖需要多靠近手掌才能开始注册为触发器拉动。"
    ))]
    #[schema(gui(slider(min = 0.0, max = 10.0)), suffix = "cm")]
    pub curl_trigger_distance: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 100.0)), suffix = "%")]
    pub joystick_deadzone: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -5.0, max = 5.0)), suffix = "cm")]
    pub joystick_offset_horizontal: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = -5.0, max = 5.0)), suffix = "cm")]
    pub joystick_offset_vertical: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "摇杆的运动半径。如果拇指在此范围的2倍内，则可以控制摇杆。"
    ))]
    #[schema(gui(slider(min = 0.0, max = 5.0)), suffix = "cm")]
    pub joystick_range: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "手势必须持续保持多长时间才能激活。"
    ))]
    #[schema(gui(slider(min = 0, max = 1000)), suffix = "ms")]
    pub activation_delay: u32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "手势必须持续释放多长时间才能停用。"
    ))]
    #[schema(gui(slider(min = 0, max = 1000)), suffix = "ms")]
    pub deactivation_delay: u32,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "手势停用后需要多长时间才能再次激活。"
    ))]
    #[schema(gui(slider(min = 0, max = 1000)), suffix = "ms")]
    pub repeat_delay: u32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct HapticsConfig {
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 5.0, step = 0.1)))]
    pub intensity_multiplier: f32,

    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)))]
    pub amplitude_curve: f32,

    #[schema(strings(display_name = "最小持续时间"))]
    #[schema(flag = "real-time")]
    #[schema(gui(slider(min = 0.0, max = 0.1, step = 0.001)), suffix = "s")]
    pub min_duration_s: f32,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HandSkeletonConfig {
    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        help = r"Enabling this will use separate tracker objects with the full skeletal tracking level when hand tracking is detected. This is required for VRChat hand tracking."
    ))]
    pub steamvr_input_2_0: bool,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = r"Predict hand skeleton to make it less floaty. It may make hands too jittery."
    ))]
    pub predict: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
#[schema(collapsible)]
pub struct ControllersConfig {
    #[schema(strings(help = "关闭此项将使控制器显示为关闭状态。"))]
    #[schema(flag = "real-time")]
    pub tracked: bool,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(
        help = "启用此项会将骨骼手部数据(手指跟踪)传递给SteamVR。"
    ))]
    pub hand_skeleton: Switch<HandSkeletonConfig>,

    #[schema(strings(
        help = r"手持控制器时跟踪手部骨骼。这将把手部跟踪频率降低到30Hz。
由于运行时限制，当身体跟踪处于活动状态时，此选项将被忽略。"
    ))]
    pub multimodal_tracking: bool,

    #[schema(flag = "real-time")]
    #[schema(strings(
        help = "启用此项允许使用手势模拟控制器输入。"
    ))]
    pub hand_tracking_interaction: Switch<HandTrackingInteractionConfig>,

    #[schema(strings(
        display_name = "预测",
        help = r"更高的值使控制器跟踪更平滑。
技术上，这是提交给SteamVR的姿态与相应虚拟垂直同步发生之间的时间（以帧为单位）。
目前无法可靠地自动估算。正确的值应为2，但默认为3，以实现更平滑的跟踪，但会略有延迟。"
    ))]
    #[schema(gui(slider(min = 1.0, max = 10.0, logarithmic)), suffix = "frames")]
    pub steamvr_pipeline_frames: f32,

    #[schema(flag = "real-time")]
    pub haptics: Switch<HapticsConfig>,

    #[schema(flag = "steamvr-restart")]
    pub emulation_mode: ControllersEmulationMode,

    #[schema(flag = "steamvr-restart")]
    pub extra_openvr_props: Vec<OpenvrProperty>,

    #[schema(flag = "real-time")]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = 0.0, max = 1.0, step = 0.01)), suffix = "m/s")]
    pub linear_velocity_cutoff: f32,

    #[schema(flag = "real-time")]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = 0.0, max = 100.0, step = 1.0)), suffix = "°/s")]
    pub angular_velocity_cutoff: f32,

    #[schema(flag = "real-time")]
    #[schema(strings(help = "右控制器偏移水平镜像"))]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = -0.5, max = 0.5, step = 0.001)), suffix = "m")]
    pub left_controller_position_offset: [f32; 3],

    #[schema(flag = "real-time")]
    #[schema(strings(help = "右控制器偏移水平镜像"))]
    #[schema(gui(slider(min = -180.0, max = 180.0, step = 1.0)), suffix = "°")]
    pub left_controller_rotation_offset: [f32; 3],

    #[schema(flag = "real-time")]
    #[schema(strings(help = "右控制器偏移水平镜像"))]
    // note: logarithmic scale seems to be glitchy for this control
    #[schema(gui(slider(min = -0.5, max = 0.5, step = 0.001)), suffix = "m")]
    pub left_hand_tracking_position_offset: [f32; 3],

    #[schema(flag = "real-time")]
    #[schema(strings(help = "右控制器偏移水平镜像"))]
    #[schema(gui(slider(min = -180.0, max = 180.0, step = 1.0)), suffix = "°")]
    pub left_hand_tracking_rotation_offset: [f32; 3],

    #[schema(strings(help = "OpenXR风格的路径列表"))]
    pub button_mappings: Option<Vec<(String, Vec<ButtonBindingTarget>)>>,

    pub button_mapping_config: AutomaticButtonMappingConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum PositionRecenteringMode {
    Disabled,
    LocalFloor,
    Local {
        #[schema(gui(slider(min = 0.0, max = 3.0)), suffix = "m")]
        view_height: f32,
    },
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum RotationRecenteringMode {
    Disabled,
    Yaw,
    Tilted,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct HeadsetConfig {
    #[schema(strings(
        help = r#"禁用：游玩空间原点由房间规模的守护系统设置决定。
本地地面：原点在地面上，长按oculus按钮时重置。
本地：原点在长按oculus按钮时重置，并计算为当前头部位置的偏移量。"#
    ))]
    #[schema(flag = "real-time")]
    pub position_recentering_mode: PositionRecenteringMode,

    #[schema(strings(
        help = r#"禁用：游玩空间方向由房间规模的守护系统设置决定。
偏航：长按oculus按钮时，前进方向会重置。
倾斜：长按oculus按钮时，世界会倾斜。这对于躺着使用VR很有用。"#
    ))]
    #[schema(flag = "real-time")]
    pub rotation_recentering_mode: RotationRecenteringMode,

    #[schema(flag = "steamvr-restart")]
    pub controllers: Switch<ControllersConfig>,

    #[schema(flag = "steamvr-restart")]
    pub emulation_mode: HeadsetEmulationMode,

    #[schema(flag = "steamvr-restart")]
    pub extra_openvr_props: Vec<OpenvrProperty>,

    #[schema(flag = "steamvr-restart")]
    pub tracking_ref_only: bool,

    #[schema(flag = "steamvr-restart")]
    pub enable_vive_tracker_proxy: bool,

    pub face_tracking: Switch<FaceTrackingConfig>,

    #[schema(flag = "steamvr-restart")]
    pub body_tracking: Switch<BodyTrackingConfig>,

    #[schema(flag = "steamvr-restart")]
    #[schema(strings(display_name = "VMC (虚拟动作捕捉)"))]
    pub vmc: Switch<VMCConfig>,

    #[schema(strings(
        help = "头部和控制器的最大预测。用于避免加载时过多的抖动。"
    ))]
    #[schema(gui(slider(min = 0, max = 200, step = 5)), suffix = "ms")]
    pub max_prediction_ms: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
#[schema(gui = "button_group")]
pub enum SocketProtocol {
    #[schema(strings(display_name = "用户数据报协议 (UDP)"))]
    Udp,
    #[schema(strings(display_name = "传输控制协议 (TCP)"))]
    Tcp,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct DiscoveryConfig {
    #[cfg_attr(target_os = "linux", schema(flag = "hidden"))]
    #[schema(strings(
        help = "允许不受信任的客户端在未经确认的情况下连接。出于安全原因，不建议这样做。"
    ))]
    pub auto_trust_clients: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum SocketBufferSize {
    Default,
    Maximum,
    Custom(#[schema(suffix = "B")] u32),
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ConnectionConfig {
    #[schema(strings(
        help = r#"UDP：比TCP更快，但稳定性较差。如果您的网络经过良好优化且没有干扰，请尝试此选项。
TCP：比UDP慢，但更稳定。如果您在使用UDP时遇到视频或音频卡顿，请选择此选项。"#
    ))]
    pub stream_protocol: SocketProtocol,

    pub client_discovery: Switch<DiscoveryConfig>,

    #[schema(strings(
        help = r#"ALVR在建立有线连接时应查找哪种发布类型的客户端。"#
    ))]
    pub wired_client_type: ClientFlavor,

    #[schema(strings(
        help = r#"ALVR在建立有线连接时是否应尝试自动启动客户端。"#
    ))]
    pub wired_client_autolaunch: bool,

    #[cfg_attr(
        windows,
        schema(strings(
            help = "如果on_connect.bat与session.json同时存在，它将在头戴式设备连接时运行。环境变量ACTION将设置为`connect`。"
        ))
    )]
    #[cfg_attr(
        not(windows),
        schema(strings(
            help = "如果on_connect.sh与session.json同时存在，它将在头戴式设备连接时运行。环境变量ACTION将设置为`connect`。"
        ))
    )]
    pub enable_on_connect_script: bool,

    #[cfg_attr(
        windows,
        schema(strings(
            help = "如果on_disconnect.bat与session.json同时存在，它将在头戴式设备断开连接时运行。环境变量ACTION将设置为`disconnect`。"
        ))
    )]
    #[cfg_attr(
        not(windows),
        schema(strings(
            help = "如果on_disconnect.sh与session.json同时存在，它将在头戴式设备断开连接时运行。环境变量ACTION将设置为`disconnect`。"
        ))
    )]
    #[schema(flag = "real-time")]
    pub enable_on_disconnect_script: bool,

    #[schema(strings(
        help = "允许跨域浏览器请求远程控制ALVR设置。"
    ))]
    #[schema(flag = "real-time")]
    pub allow_untrusted_http: bool,

    #[schema(strings(
        help = r#"如果客户端、服务器或网络丢弃了一个数据包，则丢弃数据包直到找到一个IDR数据包。"#
    ))]
    pub avoid_video_glitching: bool,

    #[schema(gui(slider(min = 1024, max = 65507, logarithmic)), suffix = "B")]
    pub packet_size: i32,

    pub stream_port: u16,
    pub web_server_port: u16,
    pub osc_local_port: u16,

    #[schema(strings(display_name = "串流器发送缓冲区大小"))]
    pub server_send_buffer_bytes: SocketBufferSize,

    #[schema(strings(display_name = "串流器接收缓冲区大小"))]
    pub server_recv_buffer_bytes: SocketBufferSize,

    #[schema(strings(display_name = "客户端发送缓冲区大小"))]
    pub client_send_buffer_bytes: SocketBufferSize,

    #[schema(strings(display_name = "客户端接收缓冲区大小"))]
    pub client_recv_buffer_bytes: SocketBufferSize,

    #[schema(strings(
        help = r#"如果服务器无法将视频数据包推送到网络，则会丢弃它们。
这可能发生在TCP上。在这种情况下，会请求一个IDR帧。"#
    ))]
    pub max_queued_server_video_frames: usize,

    #[schema(suffix = " frames")]
    pub statistics_history_size: usize,

    #[schema(strings(display_name = "最小IDR间隔"))]
    #[schema(flag = "steamvr-restart")]
    #[schema(gui(slider(min = 5, max = 1000, step = 5)), suffix = "ms")]
    pub minimum_idr_interval_ms: u64,

    pub dscp: Option<DscpTos>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
#[repr(u8)]
#[schema(gui = "button_group")]
pub enum DropProbability {
    Low = 0x01,
    Medium = 0x10,
    High = 0x11,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone, Copy)]
pub enum DscpTos {
    BestEffort,

    ClassSelector(#[schema(gui(slider(min = 1, max = 7)))] u8),

    AssuredForwarding {
        #[schema(gui(slider(min = 1, max = 4)))]
        class: u8,
        drop_probability: DropProbability,
    },

    ExpeditedForwarding,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct RawEventsConfig {
    #[schema(flag = "real-time")]
    pub hide_spammy_events: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct LoggingConfig {
    #[schema(strings(help = "通知提示教您如何使用ALVR"))]
    pub show_notification_tip: bool,

    #[schema(strings(help = "这仅适用于某些错误或警告消息。"))]
    #[schema(flag = "steamvr-restart")]
    pub prefer_backtrace: bool,

    #[schema(flag = "real-time")]
    pub notification_level: LogSeverity,

    pub client_log_report_level: Switch<LogSeverity>,

    #[schema(flag = "real-time")]
    pub show_raw_events: Switch<RawEventsConfig>,

    #[schema(strings(help = "将日志写入session_log.txt文件。"))]
    pub log_to_disk: bool,

    #[schema(flag = "real-time")]
    pub log_tracking: bool,

    #[schema(flag = "real-time")]
    pub log_button_presses: bool,

    #[schema(flag = "real-time")]
    pub log_haptics: bool,

    #[cfg_attr(not(debug_assertions), schema(flag = "hidden"))]
    #[schema(strings(help = "这些设置启用额外的垃圾日志用于调试目的。"))]
    pub debug_groups: DebugGroupsConfig,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct SteamvrLauncher {
    #[schema(strings(display_name = "通过仪表板打开和关闭SteamVR"))]
    pub open_close_steamvr_with_dashboard: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct RollingVideoFilesConfig {
    #[schema(strings(display_name = "持续时间"))]
    #[schema(suffix = "s")]
    pub duration_s: u64,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct CaptureConfig {
    #[schema(strings(display_name = "客户端连接时开始视频录制"))]
    pub startup_video_recording: bool,

    pub rolling_video_files: Switch<RollingVideoFilesConfig>,

    #[schema(flag = "steamvr-restart")]
    pub capture_frame_dir: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct Patches {
    #[schema(strings(
        help = "SteamVR中的异步计算目前已损坏，请保持禁用。仅用于测试。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub linux_async_compute: bool,
    #[schema(strings(
        help = "异步重投影仅在您始终能达到至少一半刷新率的情况下才有效。"
    ))]
    #[schema(flag = "steamvr-restart")]
    pub linux_async_reprojection: bool,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct NewVersionPopupConfig {
    pub hide_while_version: String,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct ExtraConfig {
    pub steamvr_launcher: SteamvrLauncher,
    pub capture: CaptureConfig,
    pub logging: LoggingConfig,
    #[cfg_attr(not(target_os = "linux"), schema(flag = "hidden"))]
    pub patches: Patches,

    #[schema(
        strings(help = "Linear and angular velocity multiplier for debug purposes.
It does not update in real time.")
    )]
    pub velocities_multiplier: f32,

    pub open_setup_wizard: bool,
    pub new_version_popup: Switch<NewVersionPopupConfig>,
}

#[derive(SettingsSchema, Serialize, Deserialize, Clone)]
pub struct Settings {
    pub video: VideoConfig,
    pub audio: AudioConfig,
    pub headset: HeadsetConfig,
    pub connection: ConnectionConfig,
    pub extra: ExtraConfig,
}

pub fn session_settings_default() -> SettingsDefault {
    let view_resolution = FrameSizeDefault {
        variant: FrameSizeDefaultVariant::Absolute,
        Scale: 1.0,
        Absolute: FrameSizeAbsoluteDefault {
            width: 2144,
            height: OptionalDefault {
                set: false,
                content: 1072,
            },
        },
    };
    let default_custom_audio_device = CustomAudioDeviceConfigDefault {
        NameSubstring: "".into(),
        Index: 0,
        variant: CustomAudioDeviceConfigDefaultVariant::NameSubstring,
    };
    let default_custom_openvr_props = VectorDefault {
        gui_collapsed: true,
        element: OpenvrPropertyDefault {
            key: OpenvrPropKeyDefault {
                variant: OpenvrPropKeyDefaultVariant::TrackingSystemNameString,
            },
            value: "".into(),
        },
        content: vec![],
    };
    let socket_buffer = SocketBufferSizeDefault {
        Custom: 100000,
        variant: SocketBufferSizeDefaultVariant::Maximum,
    };

    SettingsDefault {
        video: VideoConfigDefault {
            passthrough: SwitchDefault {
                enabled: false,
                content: PassthroughModeDefault {
                    variant: PassthroughModeDefaultVariant::Blend,
                    Blend: PassthroughModeBlendDefault {
                        premultiplied_alpha: true,
                        threshold: 0.5,
                    },
                    RgbChromaKey: RgbChromaKeyConfigDefault {
                        red: 0,
                        green: 255,
                        blue: 0,
                        distance_threshold: 85,
                        feathering: 0.05,
                    },
                    HsvChromaKey: HsvChromaKeyConfigDefault {
                        hue_start_max_deg: 70.0,
                        hue_start_min_deg: 80.0,
                        hue_end_min_deg: 160.0,
                        hue_end_max_deg: 170.0,
                        saturation_start_max: 0.2,
                        saturation_start_min: 0.3,
                        saturation_end_min: 1.0,
                        saturation_end_max: 1.1,
                        value_start_max: 0.0,
                        value_start_min: 0.1,
                        value_end_min: 1.0,
                        value_end_max: 1.1,
                    },
                },
            },
            clientside_post_processing: SwitchDefault {
                enabled: false,
                content: ClientsidePostProcessingConfigDefault {
                    super_sampling: ClientsidePostProcessingSuperSamplingModeDefault {
                        variant: ClientsidePostProcessingSuperSamplingModeDefaultVariant::Quality,
                    },
                    sharpening: ClientsidePostProcessingSharpeningModeDefault {
                        variant: ClientsidePostProcessingSharpeningModeDefaultVariant::Quality,
                    },
                },
            },
            upscaling: SwitchDefault {
                enabled: false,
                content: UpscalingConfigDefault {
                    edge_direction: true,
                    edge_threshold: 4.0,
                    edge_sharpness: 2.0,
                    upscale_factor: 1.5,
                },
            },
            adapter_index: 0,
            transcoding_view_resolution: view_resolution.clone(),
            emulated_headset_view_resolution: view_resolution,
            preferred_fps: 72.,
            max_buffering_frames: 2.0,
            buffering_history_weight: 0.90,
            enforce_server_frame_pacing: true,
            bitrate: BitrateConfigDefault {
                gui_collapsed: false,
                mode: BitrateModeDefault {
                    ConstantMbps: 30,
                    Adaptive: BitrateModeAdaptiveDefault {
                        gui_collapsed: true,
                        saturation_multiplier: 0.95,
                        max_throughput_mbps: SwitchDefault {
                            enabled: false,
                            content: 100,
                        },
                        min_throughput_mbps: SwitchDefault {
                            enabled: false,
                            content: 5,
                        },
                        max_network_latency_ms: SwitchDefault {
                            enabled: false,
                            content: 8,
                        },
                        encoder_latency_limiter: SwitchDefault {
                            enabled: true,
                            content: EncoderLatencyLimiterDefault {
                                max_saturation_multiplier: 0.9,
                            },
                        },
                        decoder_latency_limiter: SwitchDefault {
                            enabled: true,
                            content: DecoderLatencyLimiterDefault {
                                gui_collapsed: false,
                                max_decoder_latency_ms: 30,
                                latency_overstep_frames: 90,
                                latency_overstep_multiplier: 0.99,
                            },
                        },
                    },
                    variant: BitrateModeDefaultVariant::ConstantMbps,
                },
                adapt_to_framerate: SwitchDefault {
                    enabled: false,
                    content: BitrateAdaptiveFramerateConfigDefault {
                        framerate_reset_threshold_multiplier: 2.0,
                    },
                },
                history_size: 256,
                image_corruption_fix: false,
            },
            preferred_codec: CodecTypeDefault {
                variant: CodecTypeDefaultVariant::H264,
            },
            encoder_config: EncoderConfigDefault {
                gui_collapsed: true,
                rate_control_mode: RateControlModeDefault {
                    variant: RateControlModeDefaultVariant::Cbr,
                },
                filler_data: false,
                h264_profile: H264ProfileDefault {
                    variant: H264ProfileDefaultVariant::High,
                },
                entropy_coding: EntropyCodingDefault {
                    variant: EntropyCodingDefaultVariant::Cavlc,
                },
                use_10bit: OptionalDefault {
                    set: false,
                    content: false,
                },
                encoding_gamma: OptionalDefault {
                    set: false,
                    content: 1.0,
                },
                hdr: HDRConfigDefault {
                    gui_collapsed: true,
                    enable: OptionalDefault {
                        set: false,
                        content: false,
                    },
                    force_hdr_srgb_correction: false,
                    clamp_hdr_extended_range: false,
                },
                nvenc: NvencConfigDefault {
                    gui_collapsed: true,
                    quality_preset: EncoderQualityPresetNvidiaDefault {
                        variant: EncoderQualityPresetNvidiaDefaultVariant::P1,
                    },
                    tuning_preset: NvencTuningPresetDefault {
                        variant: NvencTuningPresetDefaultVariant::LowLatency,
                    },
                    multi_pass: NvencMultiPassDefault {
                        variant: NvencMultiPassDefaultVariant::QuarterResolution,
                    },
                    adaptive_quantization_mode: NvencAdaptiveQuantizationModeDefault {
                        variant: NvencAdaptiveQuantizationModeDefaultVariant::Spatial,
                    },
                    low_delay_key_frame_scale: -1,
                    refresh_rate: -1,
                    enable_intra_refresh: false,
                    intra_refresh_period: -1,
                    intra_refresh_count: -1,
                    max_num_ref_frames: -1,
                    gop_length: -1,
                    p_frame_strategy: -1,
                    rate_control_mode: -1,
                    rc_buffer_size: -1,
                    rc_initial_delay: -1,
                    rc_max_bitrate: -1,
                    rc_average_bitrate: -1,
                    enable_weighted_prediction: false,
                },
                quality_preset: EncoderQualityPresetDefault {
                    variant: EncoderQualityPresetDefaultVariant::Speed,
                },
                enable_vbaq: false,
                amf: AmfConfigDefault {
                    gui_collapsed: true,
                    enable_pre_analysis: false,
                    enable_hmqb: false,
                    use_preproc: false,
                    preproc_sigma: 4,
                    preproc_tor: 7,
                },
                software: SoftwareEncodingConfigDefault {
                    gui_collapsed: true,
                    force_software_encoding: false,
                    thread_count: 0,
                },
            },
            mediacodec_extra_options: {
                fn int32_default(int32: i32) -> MediacodecPropertyDefault {
                    MediacodecPropertyDefault {
                        ty: MediacodecPropTypeDefault {
                            variant: MediacodecPropTypeDefaultVariant::Int32,
                        },
                        value: int32.to_string(),
                    }
                }
                DictionaryDefault {
                    gui_collapsed: true,
                    key: "".into(),
                    value: int32_default(0),
                    content: vec![
                        ("operating-rate".into(), int32_default(i32::MAX)),
                        ("priority".into(), int32_default(0)),
                        // low-latency: only applicable on API level 30. Quest 1 and 2 might not be
                        // cabable, since they are on level 29.
                        // ("low-latency".into(), int32_default(1)), // Android smartphones crashes enabling this feature (https://github.com/PhoneVR-Developers/alvr-cardboard/issues/5)
                        (
                            "vendor.qti-ext-dec-low-latency.enable".into(),
                            int32_default(1),
                        ),
                    ],
                }
            },
            foveated_encoding: SwitchDefault {
                enabled: true,
                content: FoveatedEncodingConfigDefault {
                    gui_collapsed: true,
                    force_enable: false,
                    center_size_x: 0.45,
                    center_size_y: 0.4,
                    center_shift_x: 0.4,
                    center_shift_y: 0.1,
                    edge_ratio_x: 4.,
                    edge_ratio_y: 5.,
                },
            },
            clientside_foveation: SwitchDefault {
                enabled: false,
                content: ClientsideFoveationConfigDefault {
                    mode: ClientsideFoveationModeDefault {
                        Static: ClientsideFoveationModeStaticDefault {
                            level: ClientsideFoveationLevelDefault {
                                variant: ClientsideFoveationLevelDefaultVariant::High,
                            },
                        },
                        Dynamic: ClientsideFoveationModeDynamicDefault {
                            max_level: ClientsideFoveationLevelDefault {
                                variant: ClientsideFoveationLevelDefaultVariant::High,
                            },
                        },
                        variant: ClientsideFoveationModeDefaultVariant::Dynamic,
                    },
                    vertical_offset_deg: 0.0,
                },
            },
            force_software_decoder: false,
            color_correction: SwitchDefault {
                enabled: false,
                content: ColorCorrectionConfigDefault {
                    brightness: 0.,
                    contrast: 0.,
                    saturation: 0.5,
                    gamma: 1.,
                    sharpening: 0.5,
                },
            },
        },
        audio: AudioConfigDefault {
            game_audio: SwitchDefault {
                enabled: true,
                content: GameAudioConfigDefault {
                    gui_collapsed: true,
                    device: OptionalDefault {
                        set: false,
                        content: default_custom_audio_device.clone(),
                    },
                    mute_when_streaming: true,
                    buffering: AudioBufferingConfigDefault {
                        gui_collapsed: true,
                        average_buffering_ms: 50,
                        batch_ms: 10,
                    },
                },
            },
            microphone: SwitchDefault {
                enabled: cfg!(target_os = "linux"),
                content: MicrophoneConfigDefault {
                    gui_collapsed: true,
                    devices: MicrophoneDevicesConfigDefault {
                        Custom: MicrophoneDevicesConfigCustomDefault {
                            source: default_custom_audio_device.clone(),
                            sink: default_custom_audio_device,
                        },
                        variant: MicrophoneDevicesConfigDefaultVariant::Automatic,
                    },
                    buffering: AudioBufferingConfigDefault {
                        gui_collapsed: true,
                        average_buffering_ms: 50,
                        batch_ms: 10,
                    },
                },
            },
        },
        headset: HeadsetConfigDefault {
            emulation_mode: HeadsetEmulationModeDefault {
                Custom: HeadsetEmulationModeCustomDefault {
                    serial_number: "Unknown".into(),
                },
                variant: HeadsetEmulationModeDefaultVariant::Quest2,
            },
            extra_openvr_props: default_custom_openvr_props.clone(),
            tracking_ref_only: false,
            enable_vive_tracker_proxy: false,
            face_tracking: SwitchDefault {
                enabled: false,
                content: FaceTrackingConfigDefault {
                    gui_collapsed: true,
                    sources: FaceTrackingSourcesConfigDefault {
                        eye_tracking_fb: true,
                        face_tracking_fb: true,
                        eye_expressions_htc: true,
                        lip_expressions_htc: true,
                        face_tracking_pico: true,
                    },
                    sink: FaceTrackingSinkConfigDefault {
                        VrchatEyeOsc: FaceTrackingSinkConfigVrchatEyeOscDefault { port: 9000 },
                        variant: FaceTrackingSinkConfigDefaultVariant::VrchatEyeOsc,
                    },
                },
            },
            body_tracking: SwitchDefault {
                enabled: false,
                content: BodyTrackingConfigDefault {
                    gui_collapsed: true,
                    sources: BodyTrackingSourcesConfigDefault {
                        body_tracking_fb: SwitchDefault {
                            enabled: true,
                            content: BodyTrackingFBConfigDefault { full_body: true },
                        },
                        body_tracking_bd: SwitchDefault {
                            enabled: true,
                            content: BodyTrackingBDConfigDefault {
                                BodyTracking: BodyTrackingBDConfigBodyTrackingDefault {
                                    high_accuracy: true,
                                    prompt_calibration_on_start: true,
                                },
                                variant: BodyTrackingBDConfigDefaultVariant::BodyTracking,
                            },
                        },
                    },
                    sink: BodyTrackingSinkConfigDefault {
                        VrchatBodyOsc: BodyTrackingSinkConfigVrchatBodyOscDefault { port: 9000 },
                        variant: BodyTrackingSinkConfigDefaultVariant::FakeViveTracker,
                    },
                    tracked: true,
                },
            },
            vmc: SwitchDefault {
                enabled: false,
                content: VMCConfigDefault {
                    gui_collapsed: true,
                    host: "127.0.0.1".into(),
                    port: 39539,
                    publish: true,
                    orientation_correction: true,
                },
            },
            controllers: SwitchDefault {
                enabled: true,
                content: ControllersConfigDefault {
                    gui_collapsed: false,
                    tracked: true,
                    hand_skeleton: SwitchDefault {
                        enabled: true,
                        content: HandSkeletonConfigDefault {
                            steamvr_input_2_0: true,
                            predict: false,
                        },
                    },
                    multimodal_tracking: false,
                    emulation_mode: ControllersEmulationModeDefault {
                        Custom: ControllersEmulationModeCustomDefault {
                            serial_number: "ALVR Controller".into(),
                            button_set: VectorDefault {
                                gui_collapsed: false,
                                element: "/user/hand/left/input/a/click".into(),
                                content: vec![],
                            },
                        },
                        variant: ControllersEmulationModeDefaultVariant::Quest2Touch,
                    },
                    extra_openvr_props: default_custom_openvr_props,
                    button_mappings: OptionalDefault {
                        set: false,
                        content: DictionaryDefault {
                            gui_collapsed: false,
                            key: "/user/hand/left/input/a/click".into(),
                            value: VectorDefault {
                                gui_collapsed: false,
                                element: ButtonBindingTargetDefault {
                                    destination: "/user/hand/left/input/a/click".into(),
                                    mapping_type: ButtonMappingTypeDefault {
                                        HysteresisThreshold: HysteresisThresholdDefault {
                                            value: 0.5,
                                            deviation: 0.05,
                                        },
                                        BinaryToScalar: BinaryToScalarStatesDefault {
                                            off: 0.0,
                                            on: 1.0,
                                        },
                                        Remap: RangeDefault { min: 0.0, max: 1.0 },
                                        variant: ButtonMappingTypeDefaultVariant::Passthrough,
                                    },
                                    binary_conditions: VectorDefault {
                                        gui_collapsed: true,
                                        element: "/user/hand/left/input/trigger/touch".into(),
                                        content: vec![],
                                    },
                                },
                                content: vec![],
                            },
                            content: vec![],
                        },
                    },
                    button_mapping_config: AutomaticButtonMappingConfigDefault {
                        gui_collapsed: true,
                        click_threshold: HysteresisThresholdDefault {
                            value: 0.5,
                            deviation: 0.05,
                        },
                        touch_threshold: HysteresisThresholdDefault {
                            value: 0.1,
                            deviation: 0.05,
                        },
                        force_threshold: 0.8,
                    },
                    hand_tracking_interaction: SwitchDefault {
                        enabled: false,
                        content: HandTrackingInteractionConfigDefault {
                            only_touch: false,
                            pinch_touch_distance: 0.0,
                            pinch_trigger_distance: 0.25,
                            curl_touch_distance: 2.0,
                            curl_trigger_distance: 2.5,
                            joystick_deadzone: 40.0,
                            joystick_offset_horizontal: 0.0,
                            joystick_offset_vertical: 0.0,
                            joystick_range: 1.0,
                            repeat_delay: 100,
                            activation_delay: 50,
                            deactivation_delay: 100,
                        },
                    },
                    steamvr_pipeline_frames: 2.1,
                    linear_velocity_cutoff: 0.05,
                    angular_velocity_cutoff: 10.0,
                    left_controller_position_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.0, 0.0, -0.11],
                    },
                    left_controller_rotation_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.0; 3],
                    },
                    left_hand_tracking_position_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.04, -0.02, -0.13],
                    },
                    left_hand_tracking_rotation_offset: ArrayDefault {
                        gui_collapsed: true,
                        content: [0.0, -45.0, -90.0],
                    },
                    haptics: SwitchDefault {
                        enabled: true,
                        content: HapticsConfigDefault {
                            gui_collapsed: true,
                            intensity_multiplier: 1.0,
                            amplitude_curve: 1.0,
                            min_duration_s: 0.01,
                        },
                    },
                },
            },
            position_recentering_mode: PositionRecenteringModeDefault {
                Local: PositionRecenteringModeLocalDefault { view_height: 1.5 },
                variant: PositionRecenteringModeDefaultVariant::LocalFloor,
            },
            rotation_recentering_mode: RotationRecenteringModeDefault {
                variant: RotationRecenteringModeDefaultVariant::Yaw,
            },
            max_prediction_ms: 100,
        },
        connection: ConnectionConfigDefault {
            stream_protocol: SocketProtocolDefault {
                variant: SocketProtocolDefaultVariant::Udp,
            },
            client_discovery: SwitchDefault {
                enabled: true,
                content: DiscoveryConfigDefault {
                    auto_trust_clients: cfg!(debug_assertions),
                },
            },
            wired_client_type: ClientFlavorDefault {
                Custom: "alvr.client".to_owned(),
                variant: if alvr_common::is_stable() {
                    ClientFlavorDefaultVariant::Store
                } else {
                    ClientFlavorDefaultVariant::Github
                },
            },
            wired_client_autolaunch: true,
            web_server_port: 8082,
            stream_port: 9944,
            osc_local_port: 9942,
            dscp: OptionalDefault {
                set: false,
                content: DscpTosDefault {
                    ClassSelector: 7,
                    AssuredForwarding: DscpTosAssuredForwardingDefault {
                        class: 4,
                        drop_probability: DropProbabilityDefault {
                            variant: DropProbabilityDefaultVariant::Low,
                        },
                    },
                    variant: DscpTosDefaultVariant::ExpeditedForwarding,
                },
            },
            server_send_buffer_bytes: socket_buffer.clone(),
            server_recv_buffer_bytes: socket_buffer.clone(),
            client_send_buffer_bytes: socket_buffer.clone(),
            client_recv_buffer_bytes: socket_buffer,
            max_queued_server_video_frames: 1024,
            avoid_video_glitching: false,
            minimum_idr_interval_ms: 100,
            enable_on_connect_script: false,
            enable_on_disconnect_script: false,
            allow_untrusted_http: false,
            packet_size: 1400,
            statistics_history_size: 256,
        },
        extra: ExtraConfigDefault {
            logging: LoggingConfigDefault {
                client_log_report_level: SwitchDefault {
                    enabled: true,
                    content: LogSeverityDefault {
                        variant: LogSeverityDefaultVariant::Error,
                    },
                },
                log_to_disk: cfg!(debug_assertions),
                log_button_presses: false,
                log_tracking: false,
                log_haptics: false,
                notification_level: LogSeverityDefault {
                    variant: if cfg!(debug_assertions) {
                        LogSeverityDefaultVariant::Info
                    } else {
                        LogSeverityDefaultVariant::Warning
                    },
                },
                show_raw_events: SwitchDefault {
                    enabled: false,
                    content: RawEventsConfigDefault {
                        hide_spammy_events: false,
                    },
                },
                prefer_backtrace: false,
                show_notification_tip: true,
                debug_groups: DebugGroupsConfigDefault {
                    server_impl: false,
                    client_impl: false,
                    server_core: false,
                    client_core: false,
                    connection: false,
                    sockets: false,
                    server_gfx: false,
                    client_gfx: false,
                    encoder: false,
                    decoder: false,
                },
            },
            steamvr_launcher: SteamvrLauncherDefault {
                open_close_steamvr_with_dashboard: false,
            },
            capture: CaptureConfigDefault {
                startup_video_recording: false,
                rolling_video_files: SwitchDefault {
                    enabled: false,
                    content: RollingVideoFilesConfigDefault { duration_s: 5 },
                },
                capture_frame_dir: if !cfg!(target_os = "linux") {
                    "/tmp".into()
                } else {
                    "".into()
                },
            },
            patches: PatchesDefault {
                linux_async_compute: false,
                linux_async_reprojection: false,
            },
            velocities_multiplier: 1.0,
            open_setup_wizard: alvr_common::is_stable() || alvr_common::is_nightly(),
            new_version_popup: SwitchDefault {
                enabled: alvr_common::is_stable(),
                content: NewVersionPopupConfigDefault {
                    hide_while_version: ALVR_VERSION.to_string(),
                },
            },
        },
    }
}
