{
  inputs = {
    naersk = {
      url = "github:nix-community/naersk";
      inputs.nixpkgs.follows = "nixpkgs";
    };
    flake-compat-ci.url = "github:hercules-ci/flake-compat-ci";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, naersk, flake-compat, flake-compat-ci, }:
    let
      supportedSystems = [ "x86_64-linux" "i686-linux" "aarch64-linux" ];
      genSystems = nixpkgs.lib.genAttrs supportedSystems;
      pkgsFor = nixpkgs.legacyPackages;
      buildEnvVars = pkgs: {
        PKG_CONFIG_PATH =
          "${pkgs.openssl.dev}/lib/pkgconfig;${pkgs.postgresql.dev}/lib/pkgconfig";
      };
    in {
      packages = genSystems (system: rec {
        lunatree = naersk.lib.${system}.buildPackage {
          pname = "lunatree";
          root = ./.;
          nativeBuildInputs = with pkgsFor.${system}; [ pkgconfig openssl clang mold ];
        };
        default = lunatree;
      });

      devShells = genSystems (system: {
        default = with pkgsFor.${system};
          mkShell ({
            packages = [ zsh diesel rustc cargo openssl pkgconfig clang mold ];
            shellHook = ''
              test -f ~/.zshrc && exec zsh
            '';
          } // buildEnvVars pkgsFor.${system});
      });

      ciNix = flake-compat-ci.lib.recurseIntoFlakeWith { flake = self; };
    };
}
