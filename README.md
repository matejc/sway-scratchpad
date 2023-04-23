# sway-scratchpad

Convert a command to a scratchpad, and toggle visibility


## Install

```
cargo install sway-scratchpad
```


## Usage

```
Usage: sway-scratchpad [OPTIONS] --mark <MARK>

Options:
  -s, --sock <SOCK>            Sway/i3 socket path [default: /run/user/1000/sway-ipc.1000.2107.sock]
  -c, --command <COMMAND>      Execute command with arguments
      --width <WIDTH>          Width of scratchpad in percent [default: 95]
      --height <HEIGHT>        Height of scratchpad in percent [default: 90]
      --width-px <WIDTH_PX>    Width of scratchpad in pixels [default: 0]
      --height-px <HEIGHT_PX>  Height of scratchpad in pixels [default: 0]
  -m, --mark <MARK>            Mark the container (executed command) with with this value
  -h, --help                   Print help
  -V, --version                Print version
```


## Example config

Put this in you sway config (`~/.config/sway/config`)

```
bindsym F12 exec sway-scratchpad --command "kitty -d /home/user/projects" --mark terminal

for_window [con_mark="SCRATCHPAD_terminal"] border pixel 1
```
