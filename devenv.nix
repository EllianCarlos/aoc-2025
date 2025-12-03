{ pkgs, lib, config, inputs, ... }:

{
  # 1. Enable Rust with "batteries included"
  languages.rust = {
    enable = true;
    # specific channel (stable is usually fine for AoC, nightly if you want experimental features)
    channel = "stable"; 
    components = [ "rustc" "cargo" "clippy" "rustfmt" "rust-analyzer" ];
    
    # Use the Mold linker (much faster compilation times for iterative coding)
    mold.enable = true; 
  };

  # 2. System dependencies (often needed for crates like 'reqwest' or 'openssl')
  packages = [ 
    pkgs.openssl
    pkgs.pkg-config 
    pkgs.bacon # Optional: A background rust code checker (highly recommended)
    pkgs.aoc-cli
    pkgs.just
    pkgs.secretspec
  ];

  # 4. Automate Environment Variables
  env.RUST_BACKTRACE = "1";
  env.AOC_SESSION = config.secretspec.secrets.AOC_SESSION or "";
}
