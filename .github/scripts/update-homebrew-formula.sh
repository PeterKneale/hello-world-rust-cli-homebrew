#!/usr/bin/env bash
# Regenerate the Homebrew formula for a hello-world-rust-cli-homebrew release.
#
# Usage: update-homebrew-formula.sh <version> <source-repo> <formula-path>
#   <version>      release version without the leading v, e.g. 0.1.0
#   <source-repo>  owner/repo holding the release assets, e.g. PeterKneale/hello-world-rust-cli-homebrew
#   <formula-path> path to the formula in a checked-out copy of the tap
#
# Reads the four published <asset>.sha256 files from the release and writes a
# fresh prebuilt-binary formula. Requires the gh CLI (GH_TOKEN in CI).
set -euo pipefail

version="${1:?version required}"
repo="${2:?source repo required}"
formula="${3:?formula path required}"

bin="hello-world-rust-cli-homebrew"

tmp="$(mktemp -d)"
trap 'rm -rf "$tmp"' EXIT

gh release download "v${version}" --repo "$repo" --dir "$tmp" --pattern "${bin}-*.sha256"

sha() { awk '{print $1}' "$tmp/${bin}-${version}-$1.sha256"; }
macos_arm64="$(sha macos-arm64)"
macos_x86_64="$(sha macos-x86_64)"
linux_x86_64="$(sha linux-x86_64)"
linux_arm64="$(sha linux-arm64)"

base="https://github.com/${repo}/releases/download/v${version}"

cat > "$formula" <<EOF
class HelloWorldRustCliHomebrew < Formula
  desc "Hello world CLI"
  homepage "https://github.com/${repo}"
  version "${version}"
  license "MIT"

  on_macos do
    on_arm do
      url "${base}/${bin}-${version}-macos-arm64.tar.gz"
      sha256 "${macos_arm64}"
    end
    on_intel do
      url "${base}/${bin}-${version}-macos-x86_64.tar.gz"
      sha256 "${macos_x86_64}"
    end
  end

  on_linux do
    on_intel do
      url "${base}/${bin}-${version}-linux-x86_64.tar.gz"
      sha256 "${linux_x86_64}"
    end
    on_arm do
      url "${base}/${bin}-${version}-linux-arm64.tar.gz"
      sha256 "${linux_arm64}"
    end
  end

  def install
    bin.install "${bin}"
  end

  test do
    assert_match "Hello", shell_output("#{bin}/${bin}")
  end
end
EOF

echo "Wrote ${formula} for v${version}"
