#[cfg(windows)]
extern crate winres;

#[cfg(windows)]
fn os_spec() {
    // Metadata for Windows executables.
    let res = winres::WindowsResource::new();
    res.compile().unwrap();
}

#[cfg(unix)]
fn os_spec() {}

fn main() {
    println!("cargo:rerun-if-changed=.");
    os_spec();
}