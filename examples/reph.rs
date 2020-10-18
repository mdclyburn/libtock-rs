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
    timer.sleep(Duration::from_ms(500)).await?;

    loop {
        println!("to receive");
        let r = drivers.ism_radio.receive()?;
        println!("{}", r);
        led0.on()?;
        led1.off()?;
        timer.sleep(Duration::from_ms(2000)).await?;

        println!("to sleep");
        drivers.ism_radio.sleep()?;
        led0.off()?;
        led1.on()?;
        timer.sleep(Duration::from_ms(2000)).await?;
    }
}
