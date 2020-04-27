// MakAir
//
// Copyright: 2020, Makers For Life
// License: Public Domain License

use conrod_core::color::Color;

pub const RUNTIME_VERSION: &str = env!("CARGO_PKG_VERSION");

pub const DISPLAY_WINDOW_SIZE_WIDTH: u32 = 800;
pub const DISPLAY_WINDOW_SIZE_HEIGHT: u32 = 480;

pub const DISPLAY_GRAPH_OFFSET_WIDTH: u32 = 42;
pub const DISPLAY_GRAPH_OFFSET_HEIGHT: u32 = 230;

pub const BRANDING_WIDTH: u32 = 121;
pub const BRANDING_HEIGHT: u32 = 45;
pub const BRANDING_IMAGE_MARGIN_TOP: f64 = 22.0;
pub const BRANDING_IMAGE_MARGIN_LEFT: f64 = 26.0;
pub const BRANDING_TEXT_MARGIN_TOP: f64 = 52.0;
pub const BRANDING_TEXT_MARGIN_LEFT: f64 = 82.0;

pub const DISPLAY_ALARM_CONTAINER_WIDTH: f64 = 274.0;
pub const DISPLAY_ALARM_CONTAINER_HEIGHT: f64 = 66.0;
pub const DISPLAY_ALARM_CONTAINER_PADDING_LEFT: f64 = 12.5;
pub const DISPLAY_ALARM_CONTAINER_COLOR: Color =
    Color::Rgba(42.0 / 255.0, 42.0 / 255.0, 42.0 / 255.0, 1.0);

pub const DISPLAY_ALARM_MESSAGE_WIDTH: f64 = 110.0;
pub const DISPLAY_ALARM_MESSAGE_HEIGHT: f64 = 17.5;

pub const DISPLAY_ALARM_CODE_WIDTH: f64 = 22.5;
pub const DISPLAY_ALARM_CODE_HEIGHT: f64 = DISPLAY_ALARM_MESSAGE_HEIGHT;

pub const DISPLAY_MAX_ALARMS: usize = 2;

pub const DISPLAY_ROUNDED_RECTANGLES_ROUND: f64 = 2.5;

pub const DISPLAY_STOPPED_MESSAGE_CONTAINER_WIDTH: f64 = 320.0;
pub const DISPLAY_STOPPED_MESSAGE_CONTAINER_HEIGHT: f64 = 65.0;
pub const DISPLAY_STOPPED_MESSAGE_PADDING: f64 = 15.0;

pub const TELEMETRY_POINTS_PER_SECOND: usize = 10 * 100;
pub const TELEMETRY_WIDGET_SPACING_FROM_BOTTOM: f64 = 18.0;
pub const TELEMETRY_WIDGET_SIZE_WIDTH: f64 = 116.0;
pub const TELEMETRY_WIDGET_SIZE_HEIGHT: f64 = 72.0;
pub const TELEMETRY_WIDGET_SIZE_SPACING: f64 = 14.0;

pub const GRAPH_DRAW_SPACING_FROM_BOTTOM: f64 = 120.0;
pub const GRAPH_DRAW_SECONDS: usize = 9;
pub const GRAPH_DRAW_RANGE_LOW: i32 = 0;
pub const GRAPH_DRAW_RANGE_HIGH: i32 = 90;
pub const GRAPH_DRAW_MARGIN_TOP: u32 = 0;
pub const GRAPH_DRAW_MARGIN_BOTTOM: u32 = 10;
pub const GRAPH_DRAW_MARGIN_LEFT: u32 = 0;
pub const GRAPH_DRAW_MARGIN_RIGHT: u32 = 0;
pub const GRAPH_DRAW_LINE_SIZE: u32 = 2;
pub const GRAPH_DRAW_LABEL_WIDTH: u32 = 28;
pub const GRAPH_NUMBER_OF_POINTS: usize = GRAPH_DRAW_SECONDS * TELEMETRY_POINTS_PER_SECOND;
