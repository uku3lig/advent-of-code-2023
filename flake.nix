{
  inputs = {
    nixpkgs.url = "nixpkgs";

    flake-parts = {
      url = "github:hercules-ci/flake-parts";
      inputs.nixpkgs-lib.follows = "nixpkgs";
    };

    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs.nixpkgs.follows = "nixpkgs";
    };
  };

  outputs = {
    nixpkgs,
    flake-parts,
    rust-overlay,
    ...
  } @ inputs:
    flake-parts.lib.mkFlake {inherit inputs;} {
      systems = ["x86_64-linux" "aarch64-linux" "x86_64-darwin" "aarch64-darwin"];

      perSystem = {
        lib,
        system,
        pkgs,
        ...
      }: {
        _module.args.pkgs = import nixpkgs {
          inherit system;
          overlays = [(import rust-overlay)];
        };

        devShells.default = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [pkg-config];

          buildInputs = with pkgs; [
            (rust-bin.stable.latest.default.override {
              extensions = ["rust-analyzer" "rust-src"];
            })
            openssl
          ];

          packages = with pkgs; [nil];

          LD_LIBRARY_PATH = lib.makeLibraryPath [ pkgs.openssl ];
        };

        formatter = pkgs.alejandra;
      };
    };
}
