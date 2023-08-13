use std::process::Command;

mod setup;

const EXE: &str = "./build/release/forestry";

#[test]
fn hello() {
	setup::before();

	let result = Command::new(EXE).output().unwrap();
	assert!(result.status.success());
	let stdout = String::from_utf8_lossy(&result.stdout);

	assert!(stdout.contains("checks passed"));
}
