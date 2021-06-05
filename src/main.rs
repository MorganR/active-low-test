use embedded_hal::digital::OutputPin;
use linux_embedded_hal::gpio_cdev::{Chip, LineRequestFlags};
use linux_embedded_hal::sysfs_gpio::Direction;
use linux_embedded_hal::CdevPin;
use linux_embedded_hal::SysfsPin;
use std::error::Error;

fn blink_twice<TError>(
    led_a: &mut dyn OutputPin<Error = TError>,
    led_b: &mut dyn OutputPin<Error = TError>,
) -> Result<(), TError>
where
    TError: Error,
{
    for _ in 0..2 {
        println!("Turning LEDs on.");
        led_a.try_set_high()?;
        led_b.try_set_high()?;

        std::thread::sleep(std::time::Duration::from_millis(2000));

        println!("Turning LEDs off.");
        led_a.try_set_low()?;
        led_b.try_set_low()?;

        std::thread::sleep(std::time::Duration::from_millis(2000));
    }
    Ok(())
}

fn main() -> Result<(), Box<dyn Error>> {
    println!("Hello, world!");

    let mut active_low = SysfsPin::new(17);
    if !active_low.is_exported() {
        active_low.export()?;
    }
    active_low.set_direction(Direction::Out)?;
    active_low.set_active_low(true)?;
    let mut active_high = SysfsPin::new(27);
    if !active_high.is_exported() {
        active_high.export()?;
    }
    active_low.set_direction(Direction::Out)?;
    active_high.set_active_low(false)?;

    println!("Blinking SysfsPins twice:");
    blink_twice(&mut active_low, &mut active_high)?;

    active_low.unexport()?;
    active_high.unexport()?;

    let mut chip = Chip::new("/dev/gpiochip0")?;
    let mut active_low = CdevPin::new(chip.get_line(17)?.request(
        LineRequestFlags::OUTPUT | LineRequestFlags::ACTIVE_LOW,
        0,
        "blink-led-low",
    )?)?;
    let mut active_high = CdevPin::new(chip.get_line(27)?.request(
        LineRequestFlags::OUTPUT,
        0,
        "blink-led-high",
    )?)?;

    println!("Blinking CdevPins twice:");
    blink_twice(&mut active_low, &mut active_high)?;

    Ok(())
}
