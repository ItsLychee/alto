{ rustPlatform }:
rustPlatform.buildRustPackage {
  pname = "alto";
  version = "0-unstable-2024-03-11";
  src = ./.;
  cargoLock.lockFile = ./Cargo.lock;
  meta.mainProgram = "alto";
}
