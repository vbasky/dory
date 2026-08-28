# Rolling nightly channel pointer.
#
# THIS FILE IS AUTO-UPDATED by .github/workflows/nightly.yml on the `nightly`
# git ref immediately before the nightly tag is force-moved.  On `main` it is
# only a seed — the hashes below are placeholders and the file will NOT fetch
# successfully from main.
#
# Consume nightly via the pinned ref:
#
#   nix run github:vbasky/dory/nightly#dory-nightly
#
# Do NOT pin to main for this package. The `nightly` ref always contains
# hashes that match the artifacts published in the rolling nightly release.
#
# From-source nightly (no hash needed):
#
#   nix run github:vbasky/dory/nightly#dory-source
{
  version = "nightly";

  artifacts = {
    "x86_64-linux" = {
      url = "https://github.com/vbasky/dory/releases/download/nightly/dory-linux-amd64.tar.gz";
      hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
    };
    "aarch64-linux" = {
      url = "https://github.com/vbasky/dory/releases/download/nightly/dory-linux-arm64.tar.gz";
      hash = "sha256-AAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAAA=";
    };
  };
}
