extern crate html_escape;

struct UriStore {
	uris: std::collections::HashMap<String, usize>,
}

pub fn start_crawl(seed: &str, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
	let mut store = UriStore { uris: std::collections::HashMap::new() };
	let r = crawl(seed, seed, &mut store, 0, limit);
	match r {
		Err(error) => println!("error: {}", error),
		_ => (),
	};

	for (uri, count) in &store.uris {
		    println!("{} \"{}\"", uri, count);
	}
	
	return Ok(());
}

fn crawl(host: &str, seed: &str, store: &mut UriStore, current_depth: usize, limit: usize) -> Result<(), Box<dyn std::error::Error>> {
	if store.uris.contains_key(&seed.to_string()) {
		let seed_count = store.uris.get(&seed.to_string()).unwrap() + 1;
		store.uris.insert(seed.to_string(), seed_count);
		return Ok(());
	}

	store.uris.insert(seed.to_string(), 1);
	if seed.contains("#") || current_depth >= limit || !seed.starts_with("http") {
		return Ok(());
	}

	let mut html = reqwest::blocking::get(seed)?.text()?;
	let href_offset = 9;
	let mut done = false;

	while !done {
		let pos = html.find("<a href=");
		match pos {
			Some(x) => {
				html = remove_scanned_html(x + href_offset, &html).to_string();
				let (end_pos, clean) = find_uri(host, seed, &html);
				if end_pos != 0 {
					html = remove_scanned_html(end_pos, &html).to_string();
				}
				let r = crawl(host, &clean, store, current_depth + 1, limit); 
				match r {
					Err(error) => println!("error: {}", error),
					_ => (),
				};
			},
			None => done = true,
		}
	}
	Ok(())
}

fn find_uri(host: &str, page: &str, html: &str) -> (usize, String) {
	let pos = html.find(|c: char| (c == '\"') || (c == '\''));
	match pos {
		Some(x) => { 
			let clean = clean_html(host, page, &html[..x]);
			return (x, clean);
		},
		None => return (0, "".to_string()),
	}
}

fn remove_scanned_html(pos: usize, html: &str) -> &str {
	return &html[pos..];
}

fn clean_html(host: &str, page: &str, html: &str) -> String {
	let mut clean = String::from(html_escape::decode_html_entities(html));
	if clean.len() == 0 {
		return clean;
	}
	clean = match &clean[0..1] {
		"/"   => clean.replacen("/", host, 1),
		"#"   => page.to_owned() + &clean,
		"." => {
			let c = &clean.matches("../").count();
			let clean_paths: Vec<&str> = clean.split('/').collect();
			let short_page: Vec<&str> = page.split('/').collect();
			let mut clean_uri = "".to_string();

			for i in 0..short_page.len()-c {
				if short_page[i] == "http:" || short_page[i] == "https:" {
					clean_uri.push_str(&(short_page[i].to_owned() + "//"));
				} else if short_page[i] == "" {
					continue;
				} else {
					clean_uri.push_str(&(short_page[i].to_owned() + "/"));
				}
			}
			for i in c.to_owned()+1..clean_paths.len() {
				if clean_paths[i] == "" {
				} else {
					clean_uri.push_str(&(clean_paths[i].to_owned() + "/"));
				}
			}
			return clean_uri.trim_end_matches('/').to_string();
		},
		_     => clean,
	};
	return clean;
}
