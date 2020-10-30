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
    let settings: [(u8, u8); 1] = [
        (0x01, 0x00),

        // (0x18, 0x88),
        // (0x19, 0x55),
        // (0x1a, 0x8B),
        // (0x29, 0xe4),
        // (0x6f, 0x30)
    ];

    drivers.ism_radio.sleep()?;

    for (addr, _) in settings.iter() {
        // while drivers.ism_radio.status()? != 0 {
        //     timer.sleep(Duration::from_ms(25)).await?;
        // }

        // drivers.ism_radio.read(*addr)?;
        // while drivers.ism_radio.status()? != 0 {
        //     timer.sleep(Duration::from_ms(25)).await?;
        // }
        // println!("{:x}: {:8b}", *addr, drivers.ism_radio.get_read()?);
        let x = read(&drivers.ism_radio, &timer, *addr).await?;
        println!("{:x}: {:8b}", *addr, x);
    }

    // for setting in settings.iter() {
    //     while drivers.ism_radio.status()? != 0 {
    //         timer.sleep(Duration::from_ms(50)).await?;
    //         // println!("still busy...");
    //     }

    //     println!("setting {:x} to {:x}", setting.0, setting.1);
    //     drivers.ism_radio.write(setting.0, setting.1)?;
    //     timer.sleep(Duration::from_ms(50)).await?;
    // }

    println!("setup complete\n");

    loop {
        // let r = drivers.ism_radio.standby()?;
        // println!("  - Return: {}", r);
        // led0.off()?;
        // led1.on()?;
        // timer.sleep(Duration::from_ms(1000)).await?;

        // println!("Transmit");
        // let r = drivers.ism_radio.transmit()?;
        // println!("  - Status: {}", drivers.ism_radio.status()?);
        // println!("  - Return: {}", r);
        // led0.on()?;
        // led1.off()?;
        // timer.sleep(Duration::from_ms(1000)).await?;

        // println!("Fill FIFO");
        // let r = drivers.ism_radio.sample_fill()?;
        // println!("  - Status: {}", drivers.ism_radio.status()?);
        // println!("  - Return: {}", r);
        // timer.sleep(Duration::from_ms(1000)).await?;

        // println!("\n");
    }
}

#[allow(unused)]
async fn write<'a>(radio: &libtock::ism_radio::IsmRadioDriver,
               timer: &libtock::timer::ParallelSleepDriver<'a>,
               address: u8, value: u8) -> TockResult<usize> {
    while radio.status()? != 0 {
        timer.sleep(Duration::from_ms(25 as usize)).await?;
    }

    radio.write(address, value)
}

#[allow(unused)]
async fn read<'a>(radio: &libtock::ism_radio::IsmRadioDriver,
              timer: &libtock::timer::ParallelSleepDriver<'a>,
              address: u8) -> TockResult<usize> {
    while radio.status()? != 0 {
        timer.sleep(Duration::from_ms(25 as usize)).await?;
    }

    radio.read(address)?;

    while radio.status()? != 0 {
        timer.sleep(Duration::from_ms(25 as usize)).await?;
    }

    radio.get_read()
}
