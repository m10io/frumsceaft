use std::{
    collections::HashMap,
    path::{Path, PathBuf},
};

use anyhow::{anyhow, bail};
use clap::{Parser, Subcommand};
use colored::Colorize;
use defmt_decoder::{DecodeError, Frame, Locations, StreamDecoder};
use log::Level;
use petgraph::{
    graph::DiGraph,
    visit::{Topo, Walker},
};
use probe_rs::{config::MemoryRegion, Core, MemoryInterface as _, Session};
use probe_rs_cli_util::common_options::{CargoOptions, FlashOptions, ProbeOptions};
use probe_rs_cli_util::Artifact;
use probe_rs_rtt::{Rtt, ScanRegion, UpChannel};
use probe_run::{
    backtrace::{self, Outcome},
    canary::Canary,
    cortexm, dep,
    elf::Elf,
    target_info::TargetInfo,
};
use serde::Deserialize;
use signal_hook::consts::signal;
use std::{
    collections::BTreeSet,
    io::{self, Write},
    process::Command,
    str::FromStr,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
    time::Duration,
};

const TIMEOUT: Duration = Duration::from_secs(2);

fn main() -> anyhow::Result<()> {
    let mut args = Args::parse();
    let mut config = config::Config::default();
    if let Some(config_file) = args.config {
        config.merge(config::File::from(Path::new(&config_file)).required(false))?;
    }
    config.merge(config::File::from(Path::new("config.toml")).required(false))?;
    let mut config: Config = config.try_into()?;
    if let Some(chip) = args.probe.chip {
        let chip = Chip::from_str(&chip).map_err(|_| anyhow!("{} not supported", chip))?;
        config.chip = chip;
    }
    let image_found = match &args.cmd {
        Cmd::Run { image } | Cmd::Debug { image } => config.images.get(image.as_str()).is_some(),
        Cmd::Flash => true,
    };
    if !image_found {
        bail!("image not found")
    }
    let is_secure = config.images.iter().any(|(_, i)| i.secure);
    args.probe.chip = Some(config.chip.chip_name().to_string());
    if is_secure {
        config.chip.enable_trustzone()?;
        config.chip.wipe_chip()?;
    }
    defmt_decoder::log::init_logger(false, false, move |metadata| {
        if defmt_decoder::log::is_defmt_frame(metadata) {
            true // We want to display *all* defmt frames.
        } else {
            metadata.target().starts_with("fc-tool") && metadata.level() <= Level::Info
        }
    });

    let mut build_graph: DiGraph<_, _, u32> = DiGraph::default();
    for (name, image) in config.images.iter() {
        let a = build_graph.add_node(name.clone());
        for dep in &image.dependencies {
            let b = build_graph.add_node(dep.clone());
            build_graph.add_edge(a, b, 1);
        }
    }
    std::fs::create_dir_all(PathBuf::from("./target"))?;
    let target_path = std::fs::canonicalize(PathBuf::from("./target"))?;
    let mut artifacts: HashMap<String, Artifact> = HashMap::new();
    let nodes = Topo::new(&build_graph)
        .iter(&build_graph)
        .map(|node| {
            build_graph
                .node_weight(node)
                .ok_or_else(|| anyhow!("missing node weight"))
        })
        .collect::<Result<BTreeSet<_>, anyhow::Error>>()?;
    for node in nodes.iter().rev() {
        let image = config
            .images
            .get(node.as_str())
            .ok_or_else(|| anyhow!("image not found"))?;
        let deps = image
            .dependencies
            .iter()
            .filter(|d| artifacts.get(d.as_str()).is_some())
            .map(|a| target_path.join(a));
        println!("{} {}", "Building".green().bold(), node.green().bold());
        let node_path = target_path.join(node);
        std::fs::create_dir_all(node_path.clone())?;
        let artifact = image.build(node_path, deps, args.release)?;
        artifacts.insert(node.to_string(), artifact);
    }
    let mut sess = args.probe.simple_attach()?;
    let flash_opts = FlashOptions {
        disable_double_buffering: false,
        version: false,
        list_chips: false,
        list_probes: false,
        disable_progressbars: false,
        reset_halt: false,
        log: None,
        restore_unwritten: false,
        flash_layout_output_path: None,
        elf: None,
        work_dir: None,
        cargo_options: CargoOptions::default(),
        probe_options: args.probe,
    };
    for name in nodes.iter().rev() {
        println!("{} {}", "Flashing".green().bold(), name);
        let artifact = artifacts.get(name.as_str()).unwrap();
        let flash_loader = flash_opts
            .probe_options
            .build_flashloader(&mut sess, artifact.path())?;
        probe_rs_cli_util::flash::run_flash_download(
            &mut sess,
            artifact.path(),
            &flash_opts,
            flash_loader,
            false,
        )?;
    }

    for name in nodes.iter().rev() {
        match args.cmd {
            Cmd::Run { image: ref n } if name.as_str() == n => {
                let artifact = artifacts.get(name.as_str()).unwrap();
                let elf_bytes = std::fs::read(artifact.path())?;
                let elf = &Elf::parse(&elf_bytes)?;
                let chip_name = config.chip.chip_name();
                let target_info = TargetInfo::new(chip_name, elf)?;
                run_artifact(&target_info, &mut sess, elf)?;
            }
            Cmd::Debug { image: ref n } if name.as_str() == n => {}
            _ => {}
        }
    }
    Ok(())
}

fn run_artifact(target_info: &TargetInfo, sess: &mut Session, elf: &Elf) -> anyhow::Result<()> {
    let canary = Canary::install(sess, target_info, elf, false)?;
    start_program(sess, elf)?;

    let current_dir = &std::env::current_dir()?;

    let memory_map = sess.target().memory_map.clone();
    let mut core = sess.core(0)?;

    let halted_due_to_signal = extract_and_print_logs(elf, &mut core, &memory_map, current_dir)?;

    print_separator()?;

    let canary_touched = canary
        .map(|canary| canary.touched(&mut core, elf))
        .transpose()?
        .unwrap_or(false);

    let panic_present = canary_touched || halted_due_to_signal;

    let mut backtrace_settings = backtrace::Settings {
        current_dir,
        backtrace_limit: 50,
        backtrace: backtrace::BacktraceOptions::Auto,
        panic_present,
        shorten_paths: false,
        include_addresses: false,
    };

    let mut outcome = backtrace::print(
        &mut core,
        elf,
        &target_info.active_ram_region,
        &mut backtrace_settings,
    )?;

    // if general outcome was OK but the user ctrl-c'ed, that overrides our outcome
    // (TODO refactor this to be less bumpy)
    if halted_due_to_signal && outcome == Outcome::Ok {
        outcome = Outcome::CtrlC
    }

    core.reset_and_halt(TIMEOUT)?;

    outcome.log();

    Ok(())
}

#[derive(Parser, Debug)]
#[clap(about, version, author)]
struct Args {
    #[clap(flatten)]
    probe: ProbeOptions,
    #[clap(short, long)]
    config: Option<String>,
    #[clap(subcommand)]
    cmd: Cmd,
    #[clap(short, long, default_value = "./target")]
    target_path: PathBuf,
    #[clap(long)]
    release: bool,
}

#[derive(Subcommand, Debug, PartialEq)]
enum Cmd {
    Run { image: String },
    Debug { image: String },
    Flash,
}

#[derive(Debug, Deserialize)]
struct Config {
    chip: Chip,
    images: HashMap<String, Image>,
}

#[derive(Debug, Deserialize)]
enum Chip {
    STM32L562QEIxQ,
    NRF5340,
}

impl Chip {
    fn enable_trustzone(&self) -> anyhow::Result<()> {
        if let Chip::STM32L562QEIxQ = self {
            Command::new("STM32_Programmer_CLI")
                .arg("-c port=SWD")
                .arg("-ob")
                .arg("TZEN=1")
                .arg("SECWM2_PSTRT=1")
                .arg("SECWM2_PEND=0")
                .output()?;
        }
        Ok(())
    }

    fn wipe_chip(&self) -> anyhow::Result<()> {
        if let Chip::STM32L562QEIxQ = self {
            let output = Command::new("STM32_Programmer_CLI")
                .arg("-c port=SWD")
                .arg("-e")
                .arg("all")
                .output()?;
            if !output.status.success() {
                let error = String::from_utf8(output.stderr)?;
                return Err(anyhow!("wipe error: {}", error));
            }
            println!("{}", "Wiped Successfully".bold().green());
        }
        Ok(())
    }

    fn chip_name(&self) -> &'static str {
        match self {
            Chip::STM32L562QEIxQ => "STM32L562QEIxQ",
            Chip::NRF5340 => "nRF5340_xxAA",
        }
    }
}

impl std::str::FromStr for Chip {
    type Err = ();
    fn from_str(s: &str) -> Result<Chip, ()> {
        match s {
            "STM32L562QEIxQ" => Ok(Chip::STM32L562QEIxQ),
            "nRF5340_xxAA" => Ok(Chip::NRF5340),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Image {
    secure: bool,
    path: PathBuf,
    dependencies: Vec<String>,
}

impl Image {
    fn build(
        &self,
        lib_dir: PathBuf,
        dependencies: impl Iterator<Item = PathBuf>,
        release: bool,
    ) -> anyhow::Result<Artifact> {
        let mut args = vec![
            "-Zunstable-options".to_string(),
            "--config".to_string(),
            format!("env.FC_LIB_DIR={:?}", lib_dir),
        ];
        if release {
            args.push("--release".to_string());
        }
        for dep in dependencies {
            args.push("--config".to_string());
            args.push(format!(
                "target.thumbv8m.main-none-eabihf.rustflags=[\"-L{}\"]",
                dep.to_str().unwrap()
            ));
        }
        let artifact = probe_rs_cli_util::build_artifact(&self.path, &args)?;
        Ok(artifact)
    }
}

fn start_program(sess: &mut Session, elf: &Elf) -> anyhow::Result<()> {
    let mut core = sess.core(0)?;

    log::debug!("starting device");

    if let Some(rtt_buffer_address) = elf.rtt_buffer_address() {
        set_rtt_to_blocking(&mut core, elf.main_fn_address(), rtt_buffer_address)?
    }

    core.set_hw_breakpoint(cortexm::clear_thumb_bit(elf.vector_table.hard_fault))?;
    core.run()?;

    Ok(())
}

/// Set rtt to blocking mode
fn set_rtt_to_blocking(
    core: &mut Core,
    main_fn_address: u32,
    rtt_buffer_address: u32,
) -> anyhow::Result<()> {
    // set and wait for a hardware breakpoint at the beginning of `fn main()`
    core.set_hw_breakpoint(main_fn_address)?;
    core.run()?;
    core.wait_for_core_halted(Duration::from_secs(5))?;

    // calculate address of up-channel-flags inside the rtt control block
    const OFFSET: u32 = 44;
    let rtt_buffer_address = rtt_buffer_address + OFFSET;

    // read flags
    let channel_flags = &mut [0];
    core.read_32(rtt_buffer_address, channel_flags)?;
    // modify flags to blocking
    const MODE_MASK: u32 = 0b11;
    const MODE_BLOCK_IF_FULL: u32 = 0b10;
    let modified_channel_flags = (channel_flags[0] & !MODE_MASK) | MODE_BLOCK_IF_FULL;
    // write flags back
    core.write_word_32(rtt_buffer_address, modified_channel_flags)?;

    // clear the breakpoint we set before
    core.clear_hw_breakpoint(main_fn_address)?;

    Ok(())
}

fn extract_and_print_logs(
    elf: &Elf,
    core: &mut probe_rs::Core,
    memory_map: &[MemoryRegion],
    current_dir: &Path,
) -> anyhow::Result<bool> {
    let exit = Arc::new(AtomicBool::new(false));
    let sig_id = signal_hook::flag::register(signal::SIGINT, exit.clone())?;

    let mut logging_channel = if let Some(address) = elf.rtt_buffer_address() {
        Some(setup_logging_channel(address, core, memory_map)?)
    } else {
        eprintln!("RTT logs not available; blocking until the device halts..");
        None
    };

    let use_defmt = logging_channel
        .as_ref()
        .map_or(false, |channel| channel.name() == Some("defmt"));

    if use_defmt && elf.defmt_table.is_none() {
        bail!("\"defmt\" RTT channel is in use, but the firmware binary contains no defmt data");
    }

    let mut decoder_and_encoding = if use_defmt {
        elf.defmt_table
            .as_ref()
            .map(|table| (table.new_stream_decoder(), table.encoding()))
    } else {
        None
    };

    print_separator()?;

    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    let mut read_buf = [0; 1024];
    let mut was_halted = false;
    while !exit.load(Ordering::Relaxed) {
        if let Some(logging_channel) = &mut logging_channel {
            let num_bytes_read = match logging_channel.read(core, &mut read_buf) {
                Ok(n) => n,
                Err(e) => {
                    eprintln!("RTT error: {}", e);
                    break;
                }
            };

            if num_bytes_read != 0 {
                match decoder_and_encoding.as_mut() {
                    Some((stream_decoder, encoding)) => {
                        stream_decoder.received(&read_buf[..num_bytes_read]);
                        decode_and_print_defmt_logs(
                            &mut **stream_decoder,
                            elf.defmt_locations.as_ref(),
                            current_dir,
                            false, //opts.shorten_paths,
                            encoding.can_recover(),
                        )?;
                    }

                    _ => {
                        stdout.write_all(&read_buf[..num_bytes_read])?;
                        stdout.flush()?;
                    }
                }
            }
        }

        let is_halted = core.core_halted()?;

        if is_halted && was_halted {
            break;
        }
        was_halted = is_halted;
    }

    drop(stdout);

    signal_hook::low_level::unregister(sig_id);
    signal_hook::flag::register_conditional_default(signal::SIGINT, exit.clone())?;

    // TODO refactor: a printing fucntion shouldn't stop the MC as a side effect
    // Ctrl-C was pressed; stop the microcontroller.
    if exit.load(Ordering::Relaxed) {
        core.halt(TIMEOUT)?;
    }

    let halted_due_to_signal = exit.load(Ordering::Relaxed);

    Ok(halted_due_to_signal)
}

fn decode_and_print_defmt_logs(
    stream_decoder: &mut dyn StreamDecoder,
    locations: Option<&Locations>,
    current_dir: &Path,
    shorten_paths: bool,
    encoding_can_recover: bool,
) -> anyhow::Result<()> {
    loop {
        match stream_decoder.decode() {
            Ok(frame) => forward_to_logger(&frame, locations, current_dir, shorten_paths),
            Err(DecodeError::UnexpectedEof) => break,
            Err(DecodeError::Malformed) => match encoding_can_recover {
                // if recovery is impossible, abort
                false => return Err(DecodeError::Malformed.into()),
                // if recovery is possible, skip the current frame and continue with new data
                true => continue,
            },
        }
    }

    Ok(())
}

fn forward_to_logger(
    frame: &Frame,
    locations: Option<&Locations>,
    current_dir: &Path,
    shorten_paths: bool,
) {
    let (file, line, mod_path) = location_info(frame, locations, current_dir, shorten_paths);
    defmt_decoder::log::log_defmt(frame, file.as_deref(), line, mod_path.as_deref());
}

fn location_info(
    frame: &Frame,
    locations: Option<&Locations>,
    current_dir: &Path,
    shorten_paths: bool,
) -> (Option<String>, Option<u32>, Option<String>) {
    locations
        .map(|locations| &locations[&frame.index()])
        .map(|location| {
            let path = if let Ok(relpath) = location.file.strip_prefix(&current_dir) {
                relpath.display().to_string()
            } else {
                let dep_path = dep::Path::from_std_path(&location.file);
                match shorten_paths {
                    true => dep_path.format_short(),
                    false => dep_path.format_highlight(),
                }
            };
            (
                Some(path),
                Some(location.line as u32),
                Some(location.module.clone()),
            )
        })
        .unwrap_or((None, None, None))
}

fn setup_logging_channel(
    rtt_buffer_address: u32,
    core: &mut probe_rs::Core,
    memory_map: &[MemoryRegion],
) -> anyhow::Result<UpChannel> {
    const NUM_RETRIES: usize = 10; // picked at random, increase if necessary

    let scan_region = ScanRegion::Exact(rtt_buffer_address);
    for _ in 0..NUM_RETRIES {
        match Rtt::attach_region(core, memory_map, &scan_region) {
            Ok(mut rtt) => {
                log::debug!("Successfully attached RTT");

                let channel = rtt
                    .up_channels()
                    .take(0)
                    .ok_or_else(|| anyhow!("RTT up channel 0 not found"))?;

                return Ok(channel);
            }

            Err(probe_rs_rtt::Error::ControlBlockNotFound) => {
                log::trace!("Could not attach because the target's RTT control block isn't initialized (yet). retrying");
            }

            Err(e) => {
                return Err(anyhow!(e));
            }
        }
    }

    log::error!("Max number of RTT attach retries exceeded.");
    Err(anyhow!(probe_rs_rtt::Error::ControlBlockNotFound))
}

/// Print a line to separate different execution stages.
fn print_separator() -> io::Result<()> {
    writeln!(io::stderr(), "{}", "─".repeat(80).dimmed())
}
