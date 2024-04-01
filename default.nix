{ lib, stdenv, fetchFromGitHub, rustPlatform }:
rustPlatform.buildRustPackage rec {
  pname = "group";
  version = "1.1.0";

  src = fetchFromGitHub {
    owner = "jaschutte";
    repo = "group";
    rev = "v${version}";
    sha256 = "sha256-A+O2NjPMBHw0OPm5S6mo8Pt1NMSP0+RB1mowaX4Oyb8=";
  };
  cargoSha256 = "sha256-Dir6/iJ1H2mFQTLS8OTRJ1bOE+Ztp0iNADGaatscqMs=";
}
