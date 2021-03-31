extern crate html_escape;

fn main() -> Result<(), Box<dyn std::error::Error>> {
	let seed = "https://docs.rs/reqwest/0.11.2/reqwest/blocking/index.html";
	let host = "https://docs.rs/";
	return crawl(&host, &seed, &seed)
}

fn crawl(host: &str, page: &str, seed: &str) -> Result<(), Box<dyn std::error::Error>> {
	let mut html = reqwest::blocking::get(seed)?.text()?;
	let href_offset = 9;
	let mut done = false;

	while !done {
		let pos = html.find("<a href=");
		match pos {
			Some(x) => {
				html = remove_scanned_html(x + href_offset, &html).to_string();
				let end_pos = find_uri(host, page, &html);
				if end_pos != 0 {
					html = remove_scanned_html(end_pos, &html).to_string();
				}
			},
			None => done = true,
		}
	}
	Ok(())
}

fn find_uri(host: &str, page: &str, html: &str) -> usize {
	let pos = html.find(|c: char| (c == '\"') || (c == '\''));
	match pos {
		Some(x) => { 
			println!("{:#?}", clean_html(host, page, &html[..x]));
			return x;
		},
		None => return 0,
	}
}

fn remove_scanned_html(pos: usize, html: &str) -> &str {
	return &html[pos..];
}

fn clean_html(host: &str, page: &str, html: &str) -> String {
	let mut clean = String::from(html_escape::decode_html_entities(html));
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
