{rustPlatform}:
rustPlatform.buildRustPackage {
  pname = "no-as-a-service-rust";
  version = "1.0.0";

  src = ../.;

  cargoHash = "sha256-ay25V1hFuxTiwJuR/+7Asb7ZEh5AD/DFzAno0cMwWL0=";
}
