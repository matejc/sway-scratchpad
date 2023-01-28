# sway-scratchpad

Convert a command to a scratchpad, and toggle visibility

## Install

```
cargo install sway-scratchpad
```

## Usage

```
Usage: sway-scratchpad [OPTIONS] --command <COMMAND> --mark <MARK>

Options:
  -s, --sock <SOCK>            Sway/i3 socket path [default: /run/user/1000/sway-ipc.1000.2278443.sock]
  -c, --command <COMMAND>      Execute command
  -a, --arguments <ARGUMENTS>  Execute command with this arguments
      --width <WIDTH>          Width of scratchpad in percent [default: 95]
      --height <HEIGHT>        Height of scratchpad in percent [default: 90]
  -m, --mark <MARK>            Mark the container (executed command) with with this value
  -h, --help                   Print help
  -V, --version                Print version
```


## Example config

Put this in you sway config (`~/.config/sway/config`)

```
bindsym F12 exec sway-scratchpad --command kitty --mark terminal

for_window [con_mark="SCRATCHPAD_terminal"] border pixel 1
```
