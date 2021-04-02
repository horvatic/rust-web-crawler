mod crawler;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let args: Vec<String> = std::env::args().collect();
	if args.len() != 3 {
		println!("args: seed limit");
	}

	let seed = &args[1];
	let limit = args[2].parse::<usize>().unwrap();
	
	return crawler::start_crawl(seed, limit);
}
