{
  description = "Small exercises to get you used to reading and writing Rust code";

  inputs = {
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
    flake-utils.url = "github:numtide/flake-utils";
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
  };

  outputs = { self, flake-utils, nixpkgs, ... }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages.${system};

        cargoBuildInputs = with pkgs; lib.optionals stdenv.isDarwin [
          darwin.apple_sdk.frameworks.CoreServices
        ];

        # Specify the version of Rust here
        myRustVersion = "1.74.0";

        rustlings =
          pkgs.rustPlatform.buildRustPackage {
            name = "rustlings";
            version = "5.6.1";

            buildInputs = cargoBuildInputs;
            nativeBuildInputs = [pkgs.git];

            src = with pkgs.lib; cleanSourceWith {
              src = self;
              filter = path: type:
                let
                  baseName = builtins.baseNameOf (toString path);
                  path' = builtins.replaceStrings [ "${self}/" ] [ "" ] path;
                  inDirectory = directory: hasPrefix directory path';
                in
                inDirectory "src" ||
                inDirectory "tests" ||
                hasPrefix "Cargo" baseName ||
                baseName == "info.toml";
            };

            cargoLock.lockFile = ./Cargo.lock;

            # Override the rustc version
            rustc = "${myRustVersion}";
          };
      in
      {
        devShell = pkgs.mkShell {
          RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";

          buildInputs = with pkgs; [
            cargo
            rustc
            rust-analyzer
            rustlings
            rustfmt
            clippy
          ] ++ cargoBuildInputs;
        };
        apps = let
          rustlings-app = {
            type = "app";
            program = "${rustlings}/bin/rustlings";
          };
        in {
          default = rustlings-app;
          rustlings = rustlings-app;
        };
        packages = {
          inherit rustlings;
          default = rustlings;
        };
      });
}