use crate::{TouchEvent, TouchEventType, Touchscreen};
use embedded_graphics_core::{
    prelude::{DrawTarget, OriginDimensions, Size},
    Pixel,
};
mod xpt2046;
use xpt2046::Xpt2046;

pub struct Xpt2046Screen<
    TS: embedded_hal::spi::SpiDevice,
    S: DrawTarget + OriginDimensions,
    C: Fn((u16, u16)) -> Option<(i32, i32)>,
> {
    screen: S,
    xpt_2046: Xpt2046<TS>,
    last_touch: Option<(i32, i32)>,
    calibration: C,
}

impl<
        TS: embedded_hal::spi::SpiDevice,
        S: DrawTarget + OriginDimensions,
        C: Fn((u16, u16)) -> Option<(i32, i32)>,
    > Xpt2046Screen<TS, S, C>
{
    pub fn new(screen: S, touch_spi_device: TS, calibration: C) -> Self {
        Self {
            screen,
            xpt_2046: Xpt2046::new(touch_spi_device),
            last_touch: None,
            calibration,
        }
    }
}

impl<
        TS: embedded_hal::spi::SpiDevice,
        S: DrawTarget + OriginDimensions,
        C: Fn((u16, u16)) -> Option<(i32, i32)>,
    > DrawTarget for Xpt2046Screen<TS, S, C>
{
    type Color = S::Color;
    type Error = S::Error;

    fn draw_iter<I: IntoIterator<Item = Pixel<<Self as DrawTarget>::Color>>>(
        &mut self,
        i: I,
    ) -> Result<(), <Self as DrawTarget>::Error> {
        self.screen.draw_iter(i)
    }
}

impl<
        TS: embedded_hal::spi::SpiDevice,
        S: DrawTarget + OriginDimensions,
        C: Fn((u16, u16)) -> Option<(i32, i32)>,
    > OriginDimensions for Xpt2046Screen<TS, S, C>
{
    fn size(&self) -> Size {
        self.screen.size()
    }
}

impl<
        TS: embedded_hal::spi::SpiDevice,
        S: DrawTarget + OriginDimensions,
        C: Fn((u16, u16)) -> Option<(i32, i32)>,
    > Touchscreen for Xpt2046Screen<TS, S, C>
{
    type TouchError = <TS as embedded_hal::spi::ErrorType>::Error;

    fn get_touch_event(&mut self) -> Result<Option<TouchEvent>, Self::TouchError> {
        match (self.calibration)(self.xpt_2046.get()?) {
            Some((x, y)) => {
                let result = Some(TouchEvent {
                    x,
                    y,
                    r#type: if self.last_touch.is_some() {
                        TouchEventType::Move
                    } else {
                        TouchEventType::Start
                    },
                });
                self.last_touch = Some((x, y));

                Ok(result)
            }
            None => {
                if let Some((last_x, last_y)) = self.last_touch {
                    self.last_touch = None;

                    Ok(Some(TouchEvent {
                        x: last_x,
                        y: last_y,
                        r#type: TouchEventType::End,
                    }))
                } else {
                    Ok(None)
                }
            }
        }
    }
}

pub fn default_calibration((x, y): (u16, u16)) -> Option<(i32, i32)> {
    if x < 250 || y < 230 || x > 4000 || y > 3900 {
        return None;
    }

    // rough but fast
    Some((
        ((x - 250).wrapping_shr(6) * 9).into(),
        ((y - 230).wrapping_shr(6) * 6).into(),
    ))
}

#[cfg(test)]
mod test {
    extern crate std;

    #[test]
    fn test_convert() {
        assert_eq!(super::default_calibration((250, 230)), Some((0, 0)));
        assert_eq!(super::default_calibration((3920, 3850)), Some((513, 336)));
    }

    #[test]
    fn test_convert_out_of_range() {
        assert_eq!(super::default_calibration((200, 200)), None);
        assert_eq!(super::default_calibration((4000, 4000)), None);
    }
}
