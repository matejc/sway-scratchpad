use ksway::{Client, ipc_command};
use serde_json::{from_str, Value};
use crate::Args;

const WIDTH: &str = "width";
const HEIGHT: &str = "height";

#[derive(Debug, PartialEq)]
pub struct WindowSize {
    width: Size,
    height: Size,
}

impl WindowSize {
    pub fn new(args: Args) -> Self {
        let width = match args.width_px {
            0 => Size::Percent(args.width),
            _ => Size::Pixel(args.width_px),
        };

        let height = match args.height_px {
            0 => Size::Percent(args.height),
            _ => Size::Pixel(args.height_px),
        };

        Self { width, height }
    }

    pub fn get_sizes (&self, client: &mut Client) -> (u64, u64) {
        let mut focused_output: Option<Value> = None;

        let w = self.get_size(WIDTH, client, &mut focused_output);
        let h = self.get_size(HEIGHT, client, &mut focused_output);

        (w, h)
    }

    fn get_size(
        &self, measurement: &str,
        client: &mut Client,
        focused_output: &mut Option<Value>,
    ) -> u64 {
        match self.get_value(measurement) {
            Size::Pixel(v) => v,
            Size::Percent(v) => {
                Self::get_focused_output(client, focused_output);
                let size = focused_output.clone().unwrap()["rect"][measurement].as_u64().unwrap();
                size * v / 100
            },
        }
    }

    fn get_focused_output(client: &mut Client, focused_output: &mut Option<Value>) {
        match focused_output {
            Some(_) => (),
            None => {
                let outputs: Vec<Value> = from_str(&String::from_utf8_lossy(&client.ipc(ipc_command::get_outputs()).unwrap())).unwrap();
                let result: Value = outputs.into_iter().filter(|o| o["focused"].as_bool().unwrap()).nth(0).unwrap();
                *focused_output = Some(result);
            },
        }
    }

    fn get_value(&self, measurement: &str) -> Size {
        match measurement {
            WIDTH => self.width,
            HEIGHT => self.height,
            _ => panic!("undefined measurement"),
        }
    }

}




#[derive(Clone, Copy, Debug, PartialEq)]
enum Size {
    Percent(u64),
    Pixel(u64),
}


