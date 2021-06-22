#![no_std]

use libtock::result::TockResult;
use libtock::timer::Duration;
use libtock::println;
#[allow(unused_imports)]
use libtock::ism_radio;

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
    drivers.ism_radio.standby()?;

    // Radio configuration ==============================
    // - disable AES
    modify(&drivers.ism_radio,
           &timer,
           ism_radio::register::PacketConfig2,
           ism_radio::register::mask::PacketConfig2_AESOn,
           0).await?;

    // - use fixed-length packets
    modify(&drivers.ism_radio,
           &timer,
           ism_radio::register::PacketConfig1,
           ism_radio::register::mask::PacketConfig1_PacketFormat,
           0).await?;

    // - set payload length to 64
    write(&drivers.ism_radio,
          &timer,
          ism_radio::register::PayloadLength,
          64).await?;

    timer.sleep(Duration::from_ms(1000)).await?;

    // Enumerate registers and show their values.
    // println!("enumerating registers...");
    // for addr in 0x27u8..=0x42 {
    //     let val = read(&drivers.ism_radio, &timer, addr).await?;
    //     println!("{:02X} | {:02X} | {:08b}", addr, val, val);
    // }

    println!("Setup complete.");

    loop {
        drivers.ism_radio.sample_fill(0b10010110, 64)?;
        timer.sleep(Duration::from_ms(100)).await?;

        led1.on()?;
        drivers.ism_radio.transmit()?;
        timer.sleep(Duration::from_ms(20)).await?;
        //let mode = read(&drivers.ism_radio, &timer, ism_radio::register::OpMode).await?;

        let mut irq2 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags2).await?;
        // if irq2 & 1 << 3 == 0 {
        //     println!("Waiting for packet to be sent.");
        // }
        while irq2 & 1 << 3 == 0 {
            timer.sleep(Duration::from_ms(30)).await?;
            irq2 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags2).await?;
        }
        led1.off()?;

        drivers.ism_radio.standby()?;
        timer.sleep(Duration::from_ms(250)).await?;
        //let mode = read(&drivers.ism_radio, &timer, ism_radio::register::OpMode).await?;

        let mut irq1 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags1).await?;
        timer.sleep(Duration::from_ms(50)).await?;

        // if irq1 & 1 << 7 == 0 {
        //     println!("Waiting to enter standby mode.");
        // }
        while irq1 & 1 << 7 == 0 {
            irq1 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags1).await?;
            timer.sleep(Duration::from_ms(50)).await?;
        }

        timer.sleep(Duration::from_ms(500)).await?;
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

#[allow(unused)]
async fn modify<'a>(radio: &libtock::ism_radio::IsmRadioDriver,
                    timer: &libtock::timer::ParallelSleepDriver<'a>,
                    address: u8,
                    mask: u8,
                    mut value: u8) -> TockResult<usize> {
    let mut m_shift = mask;
    while (m_shift & 1) == 0 {
        m_shift = m_shift >> 1;
        value = value << 1;
    }

    let current_value = read(radio, timer, address).await? as u8;
    let new_value = (current_value & !mask) | value;
    println!("Modifying 0x{:02X}: {:08b} -> {:08b}", address, current_value, new_value);

    radio.write(address, new_value)
}
