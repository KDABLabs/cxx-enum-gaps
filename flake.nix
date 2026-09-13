# SPDX-FileCopyrightText: 2026 Klarälvdalens Datakonsult AB, a KDAB Group company <info@kdab.com>
# SPDX-FileContributor: Nicolas Qiu Guichard <nicolas.guichard@kdab.com>
#
# SPDX-License-Identifier: MIT OR Apache-2.0
{
  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";
    crane.url = "github:ipetkov/crane";
  };

  outputs = {
    self,
    nixpkgs,
    crane,
  }: (
    let
      system = "x86_64-linux";
      pkgs = import nixpkgs {
        inherit system;
      };

      craneLib = crane.mkLib pkgs;

      src = craneLib.cleanCargoSource ./.;
      cargoArtifacts = craneLib.buildDepsOnly {
        inherit src;
      };
      enum-gaps = craneLib.buildPackage {
        inherit src cargoArtifacts;
      };
    in {
      packages.${system} = {
        inherit enum-gaps;
        default = enum-gaps;
      };

      devShells.${system}.default = pkgs.mkShell {
        inputsFrom = [enum-gaps];

        packages = with pkgs; [
          rust-analyzer
          rr
          rustfmt
          clippy
          reuse
          cargo-expand
        ];
      };

      checks.${system} = {
        inherit enum-gaps;

        clippy = craneLib.cargoClippy {
          inherit src cargoArtifacts;
          cargoClippyExtraArgs = "--all-targets -- --deny warnings";
        };

        fmt = craneLib.cargoFmt {
          inherit src;
        };

        toml-fmt = craneLib.taploFmt {
          src = pkgs.lib.sources.sourceFilesBySuffices src [".toml"];
        };

        reuse = pkgs.runCommand "check-reuse" {} ''
          cd ${self}
          ${pkgs.reuse}/bin/reuse lint
          touch $out
        '';

        nix-fmt = pkgs.runCommand "check-nix-fmt" {} ''
          ${self.formatter.${system}}/bin/${self.formatter.${system}.NIX_MAIN_PROGRAM} -c ${self}/flake.nix
          touch $out
        '';
      };

      formatter.${system} = pkgs.alejandra;
    }
  );
}
