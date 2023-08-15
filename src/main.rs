// Feature ideas:
// `forestry sync` || `forestry update`
//   - `rustup self update && rustup update`
//   - `brew update && brew upgrade`
//   - `sudo yum update`
//   - `bun upgrade`
//   - `cargo install-update -a`
//   - `choco upgrade all`
// `forestry --fix`
// should probably make `update-icons.sh` part of this so that I don't have to run them separately
// `git-update-config`

// let checks = vec![
// 	path::InPath::pathsd("$HOME/.cargo"),
// 	path::InPath::pathsd("$HOME/.go"),
// 	path::InPath::pathsd("/opt/homebrew/bin"), // if macOS
// 	// cargo, specifically
// 	cargo::Installed::crate("bai"),
// 	cargo::Installed::crate("betty"),
// 	cargo::Installed::crate("cargo-update"),
// 	cargo::Installed::crate("kc"),
// 	cargo::Installed::crate("kirbo"),
// 	cargo::Installed::crate("mkscratch"),
// 	// system
// 	system::Installed::binary("node"),
// 	// brew or system
// 	system::Installed::binary("bash"),
// 	system::Installed::binary("bat"),
// 	system::Installed::binary("cmake"),
// 	system::Installed::binary("curl"),
// 	system::Installed::binary("dash"),
// 	system::Installed::binary("deno"),   // if personal || deno
// 	system::Installed::binary("elixir"), // if personal || gleam
// 	system::Installed::binary("erlang"), // if personal || gleam
// 	system::Installed::binary("fish"),
// 	system::Installed::binary("gcc"),
// 	system::Installed::binary("gh"),
// 	system::Installed::binary("git-lfs"),
// 	system::Installed::binary("gleam"), // if personal || gleam
// 	system::Installed::binary("go"),
// 	system::Installed::binary("gum"),
// 	system::Installed::binary("hx"),
// 	system::Installed::binary("htop"),
// 	system::Installed::binary("imagemagick"),
// 	system::Installed::binary("julia"),
// 	system::Installed::binary("lua"), // if personal || lua
// 	system::Installed::binary("nushell"),
// 	system::Installed::binary("pkg-config"),
// 	system::Installed::binary("rebar3"), // if personal || gleam
// 	system::Installed::binary("sqlite3"),
// 	system::Installed::binary("tree"),
// 	system::Installed::binary("watchexec"),
// 	system::Installed::binary("wget"),
// 	system::Installed::binary("zig"), // if personal || zig
// 	// brew, specifically
// 	brew::Installed::formula("font-cascadia-code"),
// 	brew::Installed::formula("font-cascadia-code-pl"),
// 	brew::Installed::formula("font-jetbrains-mono"),
// 	brew::Installed::formula("font-outfit"),
// 	brew::Installed::formula("font-source-code-pro"),
// 	brew::Installed::formula("font-victor-mono"),
// 	brew::Installed::formula("appcleaner"),
// 	brew::Installed::formula("bbedit"),
// 	brew::Installed::formula("rectangle"),
// 	// TODO: make sure fish config is loaded
// 	// TODO: make sure that VSCodium config is loaded, and installed
// 	// TODO: make sure that helix config is loaded, and installed
// 	// TODO: make sure that Zed config is loaded, if installed
// 	// TODO: make sure that bai config is loaded (and varies based on personal?)
// 	// TODO: make sure that 1Password SSH agent is enabled and configured
// 	//   - ~/.config/1Password/ssh/agent.toml
// 	//   - check `ssh-add -l`
// 	// TODO: make sure that git code signing is enabled and using the appropriate key
// 	// TODO: make sure that git.sr.ht, github.com, gitlab.com, and forest.mckayla.cloud are in known_hosts
// 	// TODO: make sure that $GOPATH is set
// 	// TODO: make sure that gopls, staticcheck, dlv, gofumpt are installed if go
// 	// TODO: make sure that rustup toolchain stable && rustup toolchain nightly
// ];

use std::collections::HashSet;
use std::env;
use std::ffi::OsString;
use std::fs;
use std::os::unix::fs::PermissionsExt;

fn main() {
	list_all_of_the_binaries_in_path();
	let (passed, fixed, failed) = do_all_the_things();
	println!("✅  {:>3} checks passed!", passed);
	if fixed > 0 {
		println!("    {:>3} fixes applied!", fixed);
	}
	if failed > 0 {
		println!("☣️   {:>3} checks failed!", failed);
	}
}

fn list_all_of_the_binaries_in_path() {
	let paths = env::var_os("PATH").unwrap();
	let mut binaries = HashSet::new();

	for path in env::split_paths(&paths) {
		let Ok(files) = fs::read_dir(&path) else {
			continue;
		};
		println!("{}", path.display());
		let binaries_found = files
			.flatten()
			.filter(|file| {
				let Ok(metadata) = fs::metadata(file.path()) else { return false };
				metadata.is_file() && metadata.permissions().mode() & 0o111 > 0
			})
			.map(|file| file.file_name());
		binaries.extend(binaries_found);
	}

	let check = |name| {
		println!(
			"does path contain {}? {}",
			name,
			binaries.contains(&OsString::from(name))
		)
	};

	check("bai");
	check("kc");
	check("go");
	check("gofumpt");
	check("gopls");

	// println!("{:?}", binaries);
}

fn do_all_the_things() -> (u32, u32, u32) {
	// `::context<Brew>()` for storing fix state
	(0, 0, 0)
}
