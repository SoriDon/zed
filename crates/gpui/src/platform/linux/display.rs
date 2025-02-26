use crate::{Bounds, DisplayId, GlobalPixels, PlatformDisplay, Size};
use anyhow::Result;
use uuid::Uuid;

#[derive(Debug)]
pub(crate) struct LinuxDisplay {
    x_screen_index: i32,
    bounds: Bounds<GlobalPixels>,
    uuid: Uuid,
}

impl LinuxDisplay {
    pub(crate) fn new(xc: &xcb::Connection, x_screen_index: i32) -> Self {
        let screen = xc.get_setup().roots().nth(x_screen_index as usize).unwrap();
        Self {
            x_screen_index,
            bounds: Bounds {
                origin: Default::default(),
                size: Size {
                    width: GlobalPixels(screen.width_in_pixels() as f32),
                    height: GlobalPixels(screen.height_in_pixels() as f32),
                },
            },
            uuid: Uuid::from_bytes([0; 16]),
        }
    }
}

impl PlatformDisplay for LinuxDisplay {
    fn id(&self) -> DisplayId {
        DisplayId(self.x_screen_index as u32)
    }

    fn uuid(&self) -> Result<Uuid> {
        Ok(self.uuid)
    }

    fn bounds(&self) -> Bounds<GlobalPixels> {
        self.bounds
    }
}
