{
  inputs = {
    rust-overlay = {
      url = "github:oxalica/rust-overlay";
      inputs = { nixpkgs.follows = "nixpkgs"; };
    };
  };

  outputs = { nixpkgs, rust-overlay, ... }:
    let
      overlays = [ (import rust-overlay) ];

      # Supported systems
      systems = [
        "x86_64-linux" # 64-bit Intel/AMD Linux
        "aarch64-linux" # 64-bit ARM Linux
        "x86_64-darwin" # 64-bit Intel macOS
        "aarch64-darwin" # 64-bit ARM macOS
      ];

      # Helper to provide system-specific attributes
      eachSystem = f:
        nixpkgs.lib.genAttrs systems
        (system: f { pkgs = import nixpkgs { inherit system overlays; }; });

    in {
      packages = eachSystem
        ({ pkgs }: { default = pkgs.callPackage ./default.nix { }; });
      devShells =
        eachSystem ({ pkgs }: { default = pkgs.callPackage ./shell.nix { }; });
    };
}
