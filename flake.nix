{
	description = "Build a cargo project";

	inputs = {
		nixpkgs.url = "github:NixOS/nixpkgs/nixpkgs-unstable";

		crane = {
			url = "github:ipetkov/crane";
			inputs.nixpkgs.follows = "nixpkgs";
		};

		flake-utils = {
			url = "github:numtide/flake-utils";
		};

		rust-overlay = {
			url = "github:oxalica/rust-overlay";
			inputs = {
				nixpkgs.follows = "nixpkgs";
				flake-utils.follows = "flake-utils";
			};
		};
	};

	outputs = { self, nixpkgs, crane, flake-utils, rust-overlay, ... }:
		flake-utils.lib.eachDefaultSystem (system:
			let
				pkgs = import nixpkgs {
					inherit system;
					overlays = [ (import rust-overlay) ];
				};

				craneLib = (crane.mkLib pkgs).overrideToolchain pkgs.rust-bin.stable.latest.default;
				src = self;

				# Build *just* the cargo dependencies, so we can reuse
				# all of that work (e.g. via cachix) when running in CI
				cargoArtifacts = craneLib.buildDepsOnly {
					inherit src;
				};

				# Build the actual crate itself, reusing the dependency
				# artifacts from above.
				wake = craneLib.buildPackage {
					inherit cargoArtifacts src;
				};
			in
			{
				checks = {
					# Build the crate as part of `nix flake check` for convenience
					inherit wake;

					# Run clippy (and deny all warnings) on the crate source,
					# again, resuing the dependency artifacts from above.
					#
					# Note that this is done as a separate derivation so that
					# we can block the CI if there are issues here, but not
					# prevent downstream consumers from building our crate by itself.
					wake-clippy = craneLib.cargoClippy {
						inherit cargoArtifacts src;
						cargoClippyExtraArgs = "-- --deny warnings";
					};

					# Check formatting
					wake-fmt = craneLib.cargoFmt {
						inherit src;
					};
				};

				defaultPackage = wake;
				packages.wake = wake;

				apps.my-app = flake-utils.lib.mkApp {
					drv = wake;
				};
				defaultApp = self.apps.${system}.my-app;

				devShell = craneLib.devShell {
					checks = self.checks.${system};
				};
			});
}
