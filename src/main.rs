mod window_size;

use std::{env::var, thread, time::Duration};

use async_process::{Child, Command};
use clap::Parser;
use ksway::{Client, IpcEvent, command, criteria::{con_id, con_mark}, ipc_command};
use serde_json::{from_str, Value};
use window_size::WindowSize;

const MARK_PREFIX: &str = "SCRATCHPAD_";


/// Execute commands and set mark on their Sway container for further use
#[derive(Parser)]
#[command(author, version, about, long_about = None)]
pub struct Args {
   /// Sway/i3 socket path
   #[arg(short, long, default_value_t = var("SWAYSOCK").unwrap())]
   sock: String,

   /// Execute command with arguments
   #[arg(short, long, default_value = None, use_value_delimiter = true, value_delimiter = ' ')]
   command: Vec<String>,

   /// Width of scratchpad in percent
   #[arg(long, default_value_t = 95)]
   width: u64,

   /// Height of scratchpad in percent
   #[arg(long, default_value_t = 90)]
   height: u64,

   /// Width of scratchpad in pixels
   #[arg(long, default_value_t = 0)]
   width_px: u64,

   /// Height of scratchpad in pixels
   #[arg(long, default_value_t = 0)]
   height_px: u64,

   /// Mark the container (executed command) with with this value
   #[arg(short, long)]
   mark: String,
}

fn is_running(child: &mut Child) -> bool {
    return match child.try_status().unwrap() {
        None => true,
        Some(_) => false,
    }
}

fn exec(client: &mut Client, mark: String, command: String, arguments: Vec<String>, window_size: WindowSize) {
    let mut child: Child;
    if arguments.len() == 0 {
        child = Command::new(command).spawn().unwrap();
    } else {
        child = Command::new(command).args(arguments).spawn().unwrap();
    }
    let child_pid = child.id().to_owned();

    let window_center = window_center(client, window_size);

    let rx = client.subscribe(vec![IpcEvent::Window, IpcEvent::Tick]).unwrap();
    loop {
        if ! is_running(&mut child) {
            break;
        }
        while let Ok((payload_type, payload)) = rx.try_recv() {
            match payload_type {
                IpcEvent::Window => {
                    let value: Value = from_str(&String::from_utf8_lossy(&payload)).unwrap();
                    if value["container"]["pid"] == child_pid {
                        let id: u32 = value["container"]["id"].as_u64().unwrap() as u32;
                        client.run(command::raw(format!("mark {mark}, move scratchpad, focus")).with_criteria(vec![con_id(id)])).unwrap();
                        thread::sleep(Duration::from_millis(50));
                        client.run(command::raw(format!("{window_center}")).with_criteria(vec![con_id(id)])).unwrap();
                        return;
                    }
                },
                _ => {}
            }
        }
        client.poll().unwrap();
    }
}

fn switch(client: &mut Client, mark: String, window_size: WindowSize) {
    let window_center = window_center(client, window_size);
    client.run(command::raw(format!("move scratchpad, focus")).with_criteria(vec![con_mark(mark.to_owned())])).unwrap();
    thread::sleep(Duration::from_millis(50));
    client.run(command::raw(format!("{window_center}")).with_criteria(vec![con_mark(mark)])).unwrap();
}

fn hide(client: &mut Client, mark: String) {
    client.run(command::raw("move scratchpad").with_criteria(vec![con_mark(mark)])).unwrap();
}

fn get_mark_container(containers: &Vec<Value>, mark: String) -> Result<&Value, String> {
    let container = containers.into_iter().filter(|c| c["marks"].as_array().unwrap().into_iter().any(|m| String::from(m.as_str().unwrap()) == mark)).nth(0);
    return match container {
        Some(c) => Ok(c),
        None => Err("Container Not Found".to_string()),
    };
}

fn find_edges(tree_data: &Value) -> Vec<Value> {
    let nodes = tree_data["nodes"].as_array().unwrap();
    let floating_nodes = tree_data["floating_nodes"].as_array().unwrap();
    if nodes.len() == 0 && floating_nodes.len() == 0 {
        return Vec::from([tree_data.to_owned()]);
    } else {
        let mut array_data: Vec<Value> = Vec::new();
        for node in nodes {
            array_data.append(&mut find_edges(node));
        }
        for node in floating_nodes {
            array_data.append(&mut find_edges(node));
        }
        return array_data;
    }
}

fn window_center(client: &mut Client, window_size: WindowSize) -> String {

    let (w, h) = window_size.get_sizes(client);

    return String::from(format!("resize set {w} px {h} px, move position center"));
}

fn main() {
    let args: Args = Args::parse();
    let mark = format!("{}{}", MARK_PREFIX, args.mark);

    let argv: &mut Vec<String> = &mut args.command.to_owned();
    let command: String = argv[0].to_owned();
    let arguments: Vec<String> = argv[1..].to_vec();

    let mut client = Client::connect_to_path(args.sock.to_owned()).unwrap();
    let tree_data: Value = from_str(&String::from_utf8_lossy(&client.ipc(ipc_command::get_tree()).unwrap())).unwrap();
    let containers = find_edges(&tree_data);
    let marked = get_mark_container(&containers, mark.to_owned());
    let window_size = WindowSize::new(args);
    match marked {
        Err(_) => exec(&mut client, mark, command, arguments, window_size),
        Ok(c) if c["focused"].as_bool().unwrap() => hide(&mut client, mark),
        Ok(c) if !c["focused"].as_bool().unwrap() => switch(&mut client, mark, window_size),
        _ => {}
    }
}
