{
  description = "smitty development environment";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs =
    {
      nixpkgs,
      flake-utils,
      rust-overlay,
      ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = (import nixpkgs) {
          inherit system overlays;
        };

        buildInputs = with pkgs; [
          openssl
          pkg-config
        ];

        nativeBuildInputs = [ ];
      in
      {
        devShells.default = pkgs.mkShell {
          nativeBuildInputs =
            with pkgs;
            [
              jq
              git
              act
              just
              nixd
              nixfmt-rfc-style
              (rust-bin.stable."1.88.0".default.override {
                extensions = [
                  "rust-src"
                  "rust-analyzer"
                ];
              })

            ]
            ++ buildInputs
            ++ nativeBuildInputs;
        };
      }
    );
}
