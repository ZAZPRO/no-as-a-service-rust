{
  mkShell,
  rustc,
  rustfmt,
  clippy,
  cargo,
  rust-analyzer,
}:
mkShell {
  packages = [
    rustc
    rustfmt
    clippy
    cargo
    rust-analyzer
  ];
}
