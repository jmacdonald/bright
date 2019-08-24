use std::fs::File;
use std::io::{Read, Write};
use std::path::Path;

fn main() {
    // Find and set a brightness greater than the current and quit.
    for interval in &intervals() {
        if *interval > current_brightness() {
            return set_brightness(interval);
        }
    }

    // We're already at max brightness; circle back to the minimum.
    set_brightness(&intervals()[0]);
}

fn intervals() -> [usize; 4] {
    let max = max_brightness();

    [max/25, max/10, max/4, max]
}

fn current_brightness() -> usize {
    let mut current_brightness = String::new();
    File::open(Path::new("/sys/class/backlight/intel_backlight/actual_brightness"))
        .expect("Couldn't read current brightness.")
        .read_to_string(&mut current_brightness);

    current_brightness.lines().next().unwrap().parse().expect(
        &format!("Couldn't parse current brightness value: {}", current_brightness)
    )
}

fn max_brightness() -> usize {
    let mut max_brightness = String::new();
    File::open(Path::new("/sys/class/backlight/intel_backlight/max_brightness"))
        .expect("Couldn't read max brightness.")
        .read_to_string(&mut max_brightness);

    max_brightness.lines().next().unwrap().parse().expect(
        &format!("Couldn't parse max brightness value: {}", max_brightness)
    )
}

fn set_brightness(brightness: &usize) {
    let mut device_brightness = File::create(
      Path::new("/sys/class/backlight/intel_backlight/brightness")
    ).unwrap();

    device_brightness.write_all(format!("{}", brightness).as_bytes()).unwrap();
}
