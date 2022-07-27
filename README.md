# onelinecharts

This is a very tiny rust program to produce (as of now) one-line charts, mainly intended for CLI output or for use within the tmux status bar. My main motivation to have an "arbitrary" version of [rainbarf](https://github.com/creaktive/rainbarf)

## Example usage

`./onelinecharts -- 1 50 90 100 150 80 -80`

This shows 7 bars, of which the fifth (150) and last one (-80) are above the default maximum (100) and minimum (0) respectively. Consequently, they are shown as a red full bar and a red 'empty' bar.

For usage within tmux status line you should pass the flag `--tmux`.
This is an example of part of my tmux status string.

`...  #(tail -n12 $XDG_RUNTIME_DIR/.cpu_hist | xargs olc --tmux --) ...`

Note that I have a service that logs cpu usage to the file `$XDG_RUNTIME_DIR/.cpu_hist` every ten seconds, i.e., my tmux status bar shows the CPU usage history of the last 2 minutes.

