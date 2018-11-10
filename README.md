## TL;DR

Using bright, you can safely toggle your display's brightness with `setuid` privileges via keybindings in your DE of choice.

## Background

Setting display brightness (for Intel-based graphics) on Linux is actually pretty straight-forward:

```bash
# echo $BRIGHTNESS > /sys/class/backlight/intel_backlight/brightness
```

The issue is that it requires elevated privileges. You can't put that into a root-owned script and set the `setuid` flag, either, and for [good reason](https://unix.stackexchange.com/questions/364/allow-setuid-on-shell-scripts).

This project aims to solve that by:

* encapsulating the logic in a binary that provides no external means of configuration (save for reading `actual/max_brightness`, which is controlled by the kernel)
* exposing toggle functionality that considers current and maximum device brightness

