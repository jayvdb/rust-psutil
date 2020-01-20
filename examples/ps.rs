use std::thread;
use std::time::Duration;

use psutil::process::processes;

// TODO: update to actually match the output of `ps aux`

fn main() {
	let processes = processes().unwrap();

	let block_time = Duration::from_millis(1000);
	thread::sleep(block_time);

	println!(
		"{:>6} {:>4} {:>4} {:.100}",
		"PID", "%CPU", "%MEM", "COMMAND"
	);
	for p in processes {
		let mut p = p.unwrap();

		println!(
			"{:>6} {:>2.1} {:>2.1} {:.100}",
			p.pid(),
			p.cpu_percent().unwrap(),
			p.memory_percent().unwrap(),
			p.cmdline()
				.unwrap()
				.unwrap_or_else(|| format!("[{}]", p.name().unwrap())),
		);
	}
}
