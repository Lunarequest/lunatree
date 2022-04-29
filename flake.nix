{
  inputs = {
    flake-utils.url = "github:numtide/flake-utils";
    naersk.url = "github:nix-community/naersk";
    flake-compat-ci.url = "github:hercules-ci/flake-compat-ci";
    flake-compat = {
      url = "github:edolstra/flake-compat";
      flake = false;
    };
  };

  outputs = { self, nixpkgs, flake-utils,flake-compat, flake-compat-ci, naersk }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = nixpkgs.legacyPackages."${system}";
        naersk-lib = naersk.lib."${system}";
        buildEnvVars = {
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
        };
      in rec {
        # `nix build`
        packages.lunatree = naersk-lib.buildPackage {
          pname = "lunatree";
          root = ./.;
          nativeBuildInputs = [ pkgs.pkgconfig pkgs.openssl ];
        };
        defaultPackage = packages.lunatree;

        # `nix run`
        apps.hello-world =
          flake-utils.lib.mkApp { drv = packages.lunatree; };
        defaultApp = apps.coudflareupdated;

        # `nix develop`
        devShell = pkgs.mkShell {
          nativeBuildInputs = with pkgs; [ rustc cargo openssl diesel-cli pkgconfig ];
          shellHook = ''
            test -f ~/.zshrc && exec zsh
          '';
        } // buildEnvVars;

        ciNix = flake-compat-ci.lib.recurseIntoFlakeWith { flake = self; }
          // buildEnvVars;
      });
}