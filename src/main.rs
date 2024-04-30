mod bus;
mod clock;
mod cpu;
mod device;
mod mbc;
mod mmu;
mod serial;

use clap::Parser;
use clock::Clock;
use cpu::CPU;
use mmu::MMU;
use serial::Serial;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(version, author, about, long_about = None)]
struct Args {
    /// Path of rom file, with extension `.gb` or `.gbc`.
    rom_path: PathBuf,
}

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let args = Args::parse();

    let mut clock = Clock::new();

    let mbc = mbc::new_mbc(args.rom_path).await?;
    let serial = Serial::new();
    let mmu = MMU::new(mbc, serial).await?;
    let cpu = CPU::new(clock.syncer(), mmu);

    tokio::spawn(cpu.run());

    clock.run().await;

    Ok(())
}
