{
  description = "Rust development environment with local PostgreSQL";

  inputs = {
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    flake-utils.url = "github:numtide/flake-utils";
  };

  outputs = { self, nixpkgs, flake-utils }:
    flake-utils.lib.eachDefaultSystem (system:
      let
        pkgs = import nixpkgs {
          inherit system;
        };
      in
      {
        devShells.default = pkgs.mkShell {
          packages = with pkgs; [
            bacon
            cargo
            cargo-nextest
            clippy
            openssl
            pkg-config
            postgresql_16
            rust-analyzer
            rustc
            rustfmt
          ];

          shellHook = ''
            export PROJECT_ROOT="$(pwd)"
            export PGHOST="$PROJECT_ROOT/.postgres"
            export PGDATA="$PROJECT_ROOT/.postgres/data"
            export PGPORT="5432"
            export PGUSER="aiinvite"
            export PGPASSWORD="aiinvite"
            export PGDATABASE="aiinvite_dev"
            export DATABASE_URL="postgresql://$PGUSER:$PGPASSWORD@127.0.0.1:$PGPORT/$PGDATABASE"

            mkdir -p "$PROJECT_ROOT/.postgres"

            echo
            echo "Rust + PostgreSQL environment ready"
            echo "DATABASE_URL=$DATABASE_URL"
            echo "Use: make db-init && make db-start"
            echo
          '';
        };
      });
}
