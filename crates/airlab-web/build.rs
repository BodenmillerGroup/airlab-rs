fn main() {
    let _ = std::io::Write::write_all(
        &mut std::io::stdout(),
        b"cargo:rerun-if-changed=migrations\n",
    );
}
