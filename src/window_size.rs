use ksway::{Client, ipc_command};
use serde_json::{from_str, Value};
use crate::Args;

const WIDTH: &'static str = "width";
const HEIGHT: &'static str = "height";

pub struct WindowSize {
    width: Size,
    height: Size,
}

impl<'a> WindowSize {
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

    pub fn get_sizes (&mut self, client: &mut Client) -> (u64, u64) {
        let mut focused_output: Option<Value> = None;

        let w = self.get_size(WIDTH, client, &mut focused_output);
        let h = self.get_size(HEIGHT, client, &mut focused_output);
        (w, h)
    }

    fn get_size(
        &mut self, measurement: &str,
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




#[derive(Clone, Copy)]
enum Size {
    Percent(u64),
    Pixel(u64),
}


fn window_center(client: &mut Client, width_percent: u64, height_percent: u64) -> String {
    let outputs: Vec<Value> = from_str(&String::from_utf8_lossy(&client.ipc(ipc_command::get_outputs()).unwrap())).unwrap();
    let focused_output: Value = outputs.into_iter().filter(|o| o["focused"].as_bool().unwrap()).nth(0).unwrap();

    let width = focused_output["rect"]["width"].as_u64().unwrap();
    let height = focused_output["rect"]["height"].as_u64().unwrap();

    let w = width * width_percent / 100;
    let h = height * height_percent / 100;

    return String::from(format!("resize set {w} px {h} px, move position center"));
}
