# onelinecharts

This is a very tiny rust program to produce (as of now) one-line charts, mainly intended for CLI output or for use within the tmux status bar.
This program was mostly inspired by [rainbarf](https://github.com/creaktive/rainbarf), but I just wanted to show some arbitrary data as a chart.

## Example usage

`./onelinecharts -- 1 50 90 100 150 80 -80`

This shows 7 bars, of which the fifth (150) and last one (-80) are above the default maximum (100) and minimum (0) respectively. Consequently, they are shown as a red full bar and a red 'empty' bar.

For usage within tmux status line you should pass the flag `--tmux`.
This is an example of part of my tmux status string.

`...  #(tail -n12 $XDG_RUNTIME_DIR/.cpu_hist | xargs olc --tmux --) ...`

Note that I have a service that logs cpu usage to the file `$XDG_RUNTIME_DIR/.cpu_hist` every ten seconds, i.e., my tmux status bar shows the CPU usage history of the last 2 minutes.

## FAQ

**Why are you using a light theme terminal?**

Please don't judge :)

**Can I pipe data into onelinecharts?**

Not yet, any clues how to do it elegantly with clap are welcome. For now you can make use of xargs as shown in the tmux example.
