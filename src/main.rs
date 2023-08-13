fn main() {
	let (passed, fixed, failed) = do_all_the_things();
	println!("✅  {:>3} checks passed!", passed);
	if fixed > 0 {
		println!("    {:>3} fixes applied!", fixed);
	}
	if failed > 0 {
		println!("☣️   {:>3} checks failed!", failed);
	}
}

fn do_all_the_things() -> (u32, u32, u32) {
	(0, 0, 0)
}
