#![no_std]

use libtock::result::TockResult;
use libtock::timer::Duration;
use libtock::println;

#[libtock::main]
async fn main() -> TockResult<()> {
    let mut drivers = libtock::retrieve_drivers()?;

    drivers.console.create_console();

    let led_driver = drivers.leds.init_driver()?;
    let led0 = led_driver.get(0)?;
    let led1 = led_driver.get(1)?;

    let mut timer = drivers.timer.create_timer_driver();
    let timer = timer.activate()?;

    led0.off()?;
    led1.off()?;

    drivers.ism_radio.reset()?;
    timer.sleep(Duration::from_ms(100)).await?;

    // Recommended settings
    let settings = [
        (0x18, 0x88),
        (0x19, 0x55),
        (0x1a, 0x8B),
        (0x29, 0xe4),
        (0x6f, 0x30)
    ];

    for setting in settings.iter() {
        while drivers.ism_radio.status()? != 0 {
            timer.sleep(Duration::from_ms(50)).await?;
            println!("still busy...");
        }

        timer.sleep(Duration::from_ms(100)).await?;
        println!("setting {}", setting.0);
        drivers.ism_radio.write(setting.0, setting.1)?;
    }

    println!("setup complete\n");

    loop {
        println!("to transmit");
        let r = drivers.ism_radio.transmit()?;
        println!("return code: {}", r);
        println!("current status: {}", drivers.ism_radio.status()?);
        led0.on()?;
        led1.off()?;
        timer.sleep(Duration::from_ms(500)).await?;

        println!("\n");

        println!("to standby");
        let r = drivers.ism_radio.standby()?;
        println!("return code: {}", r);
        led0.off()?;
        led1.on()?;
        timer.sleep(Duration::from_ms(2000)).await?;

        println!("sample fill");
        let r = drivers.ism_radio.sample_fill()?;
        println!("return code: {}", r);
        timer.sleep(Duration::from_ms(500)).await?;
    }
}
