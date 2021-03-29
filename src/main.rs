fn main() -> Result<(), Box<dyn std::error::Error>> {
	let seed = "https://docs.rs/reqwest/0.11.2/reqwest/blocking/index.html";
	return crawl(&seed)
}

fn crawl(seed: &str) -> Result<(), Box<dyn std::error::Error>> {
	let mut html = reqwest::blocking::get(seed)?.text()?;
	let href_offset = 6;
	let mut done = false;

	while !done {
		let pos = html.find("href=");
		match pos {
			Some(x) => {
				html = remove_scanned_html(x + href_offset, &html).to_string();
				let end_pos = find_uri(&html);
				if end_pos != 0 {
					html = remove_scanned_html(end_pos, &html).to_string();
				}
			},
			None => done = true,
		}
	}
	Ok(())
}

fn find_uri(html: &str) -> usize {
	let pos = html.find("\"");
	match pos {
		Some(x) => { 
			println!("{:#?}", &html[..x]);
			return x;
		},
		None => return 0,
	}
}

fn remove_scanned_html(pos: usize, html: &str) -> &str {
	return &html[pos..];
}
