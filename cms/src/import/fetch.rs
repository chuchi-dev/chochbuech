//! Fetches the one page a member asked for, and nothing else: http(s)
//! on the usual ports, only public addresses (the cms must not become a
//! way into the server's own network), a few redirects, a time and size
//! limit, only html.

use std::net::{IpAddr, Ipv4Addr, Ipv6Addr, SocketAddr};
use std::time::Duration;

use reqwest::header;
use reqwest::redirect::Policy;
use url::{Host, Url};

pub const USER_AGENT: &str =
	"chochbuech.com recipe import (+https://chochbuech.com)";
const MAX_REDIRECTS: usize = 3;
const TIMEOUT: Duration = Duration::from_secs(10);
pub const MAX_BYTES: usize = 2 * 1024 * 1024;

/// A fetched page: where it ended up after the redirects, its html.
#[derive(Debug, Clone)]
pub struct Page {
	pub url: Url,
	pub html: String,
}

/// Where the pages come from, the web or the tests' fixtures.
#[async_trait::async_trait]
pub trait Pages: Send + Sync {
	async fn fetch(&self, url: Url) -> Result<Page, String>;
}

pub struct Web;

#[async_trait::async_trait]
impl Pages for Web {
	async fn fetch(&self, url: Url) -> Result<Page, String> {
		match tokio::time::timeout(TIMEOUT, fetch(url)).await {
			Ok(r) => r,
			Err(_) => Err("The page took too long to answer.".into()),
		}
	}
}

/// The url a member typed, if it may be fetched at all.
pub fn parse_url(s: &str) -> Result<Url, String> {
	let url = Url::parse(s.trim()).map_err(|_| {
		"This is not a valid address of a web page.".to_string()
	})?;
	check_url(&url)?;
	Ok(url)
}

fn check_url(url: &Url) -> Result<(), String> {
	if !matches!(url.scheme(), "http" | "https") {
		return Err("Only http and https addresses can be imported.".into());
	}
	if !url.username().is_empty() || url.password().is_some() {
		return Err("Addresses with a login can't be imported.".into());
	}
	let ip: Option<IpAddr> = match url.host() {
		None => {
			return Err("This is not a valid address of a web page.".into());
		}
		Some(Host::Ipv4(ip)) => Some(ip.into()),
		Some(Host::Ipv6(ip)) => Some(ip.into()),
		Some(Host::Domain(_)) => None,
	};
	if ip.is_some_and(|ip| !is_public(ip)) {
		return Err("This address is not on the public internet.".into());
	}
	// the ports of web sites, nothing else on a server
	if url.port().is_some_and(|p| p != 80 && p != 443) {
		return Err("Only pages on the usual ports can be imported.".into());
	}
	Ok(())
}

/// The public addresses of the url's host; a host with any other
/// address is refused, like one that resolves to the server itself.
async fn public_addrs(url: &Url) -> Result<Vec<SocketAddr>, String> {
	let port = url.port_or_known_default().unwrap_or(443);
	let ips: Vec<IpAddr> = match url.host() {
		Some(Host::Ipv4(ip)) => vec![ip.into()],
		Some(Host::Ipv6(ip)) => vec![ip.into()],
		Some(Host::Domain(d)) => tokio::net::lookup_host((d, port))
			.await
			.map_err(|_| "This web site could not be found.".to_string())?
			.map(|a| a.ip())
			.collect(),
		None => vec![],
	};
	if ips.is_empty() {
		return Err("This web site could not be found.".into());
	}
	if ips.iter().any(|ip| !is_public(*ip)) {
		return Err("This address is not on the public internet.".into());
	}
	Ok(ips
		.into_iter()
		.map(|ip| SocketAddr::new(ip, port))
		.collect())
}

async fn fetch(mut url: Url) -> Result<Page, String> {
	for _ in 0..=MAX_REDIRECTS {
		check_url(&url)?;
		let addrs = public_addrs(&url).await?;
		// connect to the addresses just checked, not to what another
		// lookup might answer
		let mut client = reqwest::Client::builder()
			.redirect(Policy::none())
			.no_proxy()
			.timeout(TIMEOUT)
			.user_agent(USER_AGENT);
		if let Some(Host::Domain(d)) = url.host() {
			client = client.resolve_to_addrs(d, &addrs);
		}
		let client = client.build().map_err(|e| e.to_string())?;
		let mut resp = client
			.get(url.clone())
			.header(header::ACCEPT, "text/html,application/xhtml+xml")
			.send()
			.await
			.map_err(|_| "The page could not be loaded.".to_string())?;

		if resp.status().is_redirection() {
			let to = resp
				.headers()
				.get(header::LOCATION)
				.and_then(|l| l.to_str().ok())
				.and_then(|l| url.join(l).ok())
				.ok_or("The page could not be loaded.")?;
			url = to;
			continue;
		}
		if !resp.status().is_success() {
			return Err(format!(
				"The page answered with an error ({}).",
				resp.status().as_u16()
			));
		}
		let ty = resp
			.headers()
			.get(header::CONTENT_TYPE)
			.and_then(|t| t.to_str().ok())
			.unwrap_or("")
			.to_ascii_lowercase();
		if !ty.starts_with("text/html") && !ty.starts_with("application/xhtml")
		{
			return Err("This address is not a web page.".into());
		}
		if resp.content_length().is_some_and(|l| l > MAX_BYTES as u64) {
			return Err("The page is too big.".into());
		}
		let mut body = Vec::new();
		while let Some(chunk) = resp
			.chunk()
			.await
			.map_err(|_| "The page could not be loaded.".to_string())?
		{
			body.extend_from_slice(&chunk);
			if body.len() > MAX_BYTES {
				return Err("The page is too big.".into());
			}
		}
		let html = String::from_utf8_lossy(&body).into_owned();
		return Ok(Page { url, html });
	}
	Err("The page redirects too often.".into())
}

/// Whether an address is on the public internet: not the server itself,
/// not a private, link-local, shared, documentation or reserved one.
pub fn is_public(ip: IpAddr) -> bool {
	match ip {
		IpAddr::V4(ip) => is_public_v4(ip),
		IpAddr::V6(ip) => is_public_v6(ip),
	}
}

fn is_public_v4(ip: Ipv4Addr) -> bool {
	let [a, b, c, _] = ip.octets();
	!(ip.is_unspecified()
		|| ip.is_loopback()
		|| ip.is_private()
		|| ip.is_link_local()
		|| ip.is_broadcast()
		|| ip.is_documentation()
		|| ip.is_multicast()
		|| a == 0
		// shared address space (carrier-grade NAT)
		|| (a == 100 && (64..128).contains(&b))
		// IETF protocol assignments
		|| (a == 192 && b == 0 && c == 0)
		// benchmarking
		|| (a == 198 && (18..20).contains(&b))
		// reserved
		|| a >= 240)
}

fn is_public_v6(ip: Ipv6Addr) -> bool {
	// an IPv4 address in IPv6 clothes is judged as IPv4
	if let Some(v4) = ip.to_ipv4_mapped() {
		return is_public_v4(v4);
	}
	let s = ip.segments();
	// only global unicast (2000::/3) is public
	if s[0] & 0xe000 != 0x2000 {
		return false;
	}
	// documentation, Teredo, 6to4 (which wraps any IPv4) and ORCHID
	!(s[0] == 0x2001 && s[1] == 0x0db8
		|| s[0] == 0x2001 && s[1] == 0
		|| s[0] == 0x2002
		|| s[0] == 0x2001 && (0x10..0x30).contains(&s[1]))
}
