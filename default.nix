{ lib, stdenv, fetchFromGitHub, rustPlatform }:
rustPlatform.buildRustPackage rec {
  pname = "group";
  version = "1.2.0";

  src = fetchFromGitHub {
    owner = "jaschutte";
    repo = "group";
    rev = "v${version}";
    sha256 = "sha256-pzaFiEza+pXp0xQhTzM9a4e393vFhUKRBCYOZysv5aU=";
  };
  cargoSha256 = "sha256-lk/RKzNCx7Q1HJAkcyGuC6zvOo04a4OcZtyw8KLhC2g=";
}
