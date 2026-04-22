{
  description = "bevy flake";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-unstable";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs =
    { nixpkgs
    , rust-overlay
    , flake-utils
    , ...
    }:
    flake-utils.lib.eachDefaultSystem (
      system:
      let
        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs { inherit system overlays; };

        # ── Dimensions ────────────────────────────────────────────────────────
        profiles = [
          {
            suffix = "";
            flag = "";
            label = "debug";
          }
          {
            suffix = "r";
            flag = "--release";
            label = "release";
          }
          {
            suffix = "o";
            flag = "--profile release_optimized";
            label = "optimized";
          }
        ];

        natives = [
          {
            suffix = "";
            env = "";
            label = "";
          }
          {
            suffix = "_native";
            env = ''RUSTFLAGS="-C target-cpu=native" '';
            label = "native CPU";
          }
        ];

        perfs = [
          {
            suffix = "";
            bin = "";
            label = "";
          }
          {
            suffix = "_perf";
            bin = "mangohud ";
            label = "perf monitor";
          }
        ];

        # ── Script generator ──────────────────────────────────────────────────
        mkScript =
          p: n: perf:
          let
            name = "run${p.suffix}${n.suffix}${perf.suffix}";
            desc = pkgs.lib.concatStringsSep ", " (
              pkgs.lib.filter (s: s != "") [
                p.label
                n.label
                perf.label
              ]
            );
          in
          {
            pkg = pkgs.writeShellScriptBin name "${n.env}${perf.bin}cargo run ${p.flag}";
            name = name;
            desc = desc;
          };

        # Flatten: perf last (outermost loop = perf)
        allScripts = pkgs.lib.flatten (
          map (perf: map (n: map (p: mkScript p n perf) profiles) natives) perfs
        );

        customAliases = map (s: s.pkg) allScripts;

        # ── Help table for shellHook ──────────────────────────────────────────
        helpLines = pkgs.lib.concatMapStrings
          (
            s: ''printf "    %-22s %s\n" "${s.name}" "${s.desc}"'' + "\n"
          )
          allScripts;

      in
      {
        devShells.default =
          with pkgs;
          mkShell {
            packages = customAliases;

            buildInputs = [
              (rust-bin.stable.latest.default.override { extensions = [ "rust-src" ]; })
              pkg-config
              mangohud
              fontforge
            ]
            ++ lib.optionals (lib.strings.hasInfix "linux" system) [
              alsa-lib
              vulkan-loader
              vulkan-tools
              wayland
              libx11
              libxcursor
              libxi
              libxrandr
              libxkbcommon
              udev
            ];

            RUST_SRC_PATH = "${pkgs.rust.packages.stable.rustPlatform.rustLibSrc}";
            LD_LIBRARY_PATH = lib.makeLibraryPath [
              vulkan-loader
              wayland
              libx11
              libxi
              libxcursor
              libxkbcommon
            ];

            shellHook = ''
              echo -e "\033[1;33mRUN COMMANDS\033[0m"
              ${helpLines}
              echo -e "\033[1;34m━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━\033[0m"
            '';
          };
      }
    );
}
