use dkregistry::v2::Client;
use regex::Regex;

fn get_image_components(raw_image: &str) -> (&str, &str, &str) {
    let re = Regex::new(r"(?:([a-z0-9.-]+)/)?([a-z-]+/[a-z-]+):?([a-z0-9.-]+)?").unwrap();
    let caps = re.captures(raw_image).unwrap();

    let registry = caps.get(1).map_or("registry-1.docker.io", |m| m.as_str());
    let image = caps.get(2).map(|m| m.as_str()).unwrap();
    let tag = caps.get(3).map_or("latest", |m| m.as_str());

    return (registry, image, tag);
}

async fn get_digest<'a>(
    registry: &'a str,
    image: &'a str,
    tag: &'a str,
) -> Option<String> {
    let client = Client::configure().registry(registry).build().unwrap();
    let login_scope = format!("repository:{}:pull", image);
    let dclient = client.authenticate(&[&login_scope]).await.unwrap();
    return dclient.get_manifestref(image, tag).await.unwrap();
}

#[tokio::main]
async fn main() {
    let (registry, image, tag) = get_image_components("acockburn/appdaemon:4.2.1");
    println!("registry: {}", registry);
    println!("image: {}", image);
    println!("tag: {}", tag);
    let digest = get_digest(registry, image, tag).await.unwrap();
    println!("digest: {}", digest);
}
