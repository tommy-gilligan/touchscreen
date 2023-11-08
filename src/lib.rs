#![no_std]
use embedded_graphics_core::prelude::{DrawTarget, OriginDimensions};

#[derive(Debug, PartialEq, Eq)]
pub enum TouchEventType {
    Start,
    Move,
    End,
}

#[derive(Debug, PartialEq, Eq)]
pub struct TouchEvent {
    pub x: i32,
    pub y: i32,
    pub r#type: TouchEventType,
}

pub trait Touchscreen: DrawTarget + OriginDimensions {
    type TouchError;

    fn get_touch_event(&mut self) -> Result<Option<TouchEvent>, Self::TouchError>;
}

#[cfg(feature = "red-screen")]
pub mod red_screen;

#[cfg(feature = "web-screen")]
pub mod web_screen;

#[cfg(feature = "sdl-screen")]
pub mod sdl_screen;
