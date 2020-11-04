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

    println!("Energy account is available: {}", drivers.energy_account.is_available()?);
    println!("Current value: {}", drivers.energy_account.total_usage()?);

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

    // Enumerate registers and show their values.
    // println!("enumerating registers...");
    // for addr in 0x3eu8..=0x42 {
    //     let val = read(&drivers.ism_radio, &timer, addr).await?;
    //     println!("{:02X} | {:02X} | {:08b}", addr, val, val);
    // }

    // for (addr, val) in &settings {
    //     write(&drivers.ism_radio, &timer, *addr, *val).await?;
    // }

    // println!("enumerating registers...");
    // for addr in 0x3eu8..=0x42 {
    //     let val = read(&drivers.ism_radio, &timer, addr).await?;
    //     println!("{:02X} | {:02X} | {:08b}", addr, val, val);
    // }

    // for (addr, _val) in settings.iter() {
    //     write(&drivers.ism_radio, &timer, *addr, *val).await?;

    //     let x = read(&drivers.ism_radio, &timer, *addr).await?;
    //     println!("{:x}: {:x}", *addr, x);
    // }

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
        println!("Standby");
        drivers.ism_radio.standby()?;
        timer.sleep(Duration::from_ms(1000)).await?;
        let mode = read(&drivers.ism_radio, &timer, ism_radio::register::OpMode).await?;
        println!("Mode:      {:08b}", mode);

        let mut irq1 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags1).await?;
        println!("IRQFlags1: {:08b}", irq1);
        let irq2 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags2).await?;
        println!("IRQFlags2: {:08b}", irq2);
        timer.sleep(Duration::from_ms(3000)).await?;

        while irq1 & 1 << 7 == 0 {
            println!("    mode not ready...");
            irq1 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags1).await?;
            timer.sleep(Duration::from_ms(250)).await?;
        }

        for _i in 0..10 {
            println!("Energy: {}", drivers.energy_account.total_usage()?);
            timer.sleep(Duration::from_ms(150)).await?;
        }

        println!("");

        println!("Fill FIFO");
        drivers.ism_radio.sample_fill(0b10010110, 64)?;
        timer.sleep(Duration::from_ms(1000)).await?;

        let irq1 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags1).await?;
        println!("IRQFlags1: {:08b}", irq1);
        let irq2 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags2).await?;
        println!("IRQFlags2: {:08b}", irq2);
        timer.sleep(Duration::from_ms(2000)).await?;

        println!("");

        println!("Transmit");
        drivers.ism_radio.transmit()?;
        timer.sleep(Duration::from_ms(1000)).await?;
        let mode = read(&drivers.ism_radio, &timer, ism_radio::register::OpMode).await?;
        println!("Mode:      {:08b}", mode);

        let mut irq1 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags1).await?;
        println!("IRQFlags1: {:08b}", irq1);
        let irq2 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags2).await?;
        println!("IRQFlags2: {:08b}", irq2);
        timer.sleep(Duration::from_ms(3000)).await?;

        while irq1 & 1 << 7 == 0 {
            println!("    mode not ready...");
            irq1 = read(&drivers.ism_radio, &timer, ism_radio::register::IRQFlags1).await?;
            timer.sleep(Duration::from_ms(250)).await?;
        }

        for _i in 0..10 {
            println!("Energy: {}", drivers.energy_account.total_usage()?);
            timer.sleep(Duration::from_ms(150)).await?;
        }

        println!("");
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
