{
  pkgs,
  lib,
  config,
  inputs,
  ...
}:

{
  packages = with pkgs; [
    pkgsCross.avr.buildPackages.gcc
    avrdude
    ravedude
  ];

  languages.rust = {
    enable = true;
    channel = "nightly";
    # version = "2025-04-27";
    components = [
      "rustc"
      "cargo"
      "clippy"
      "rustfmt"
      "rust-analyzer"
      "rust-src"
    ];
  };
}
