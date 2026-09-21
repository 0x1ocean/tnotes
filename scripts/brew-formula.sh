#!/usr/bin/env sh
# Print the Homebrew formula for a released version, reading the .sha256 files of the
# GitHub release. Usage: scripts/brew-formula.sh 1.3.0 > Formula/tnotes.rb
set -eu
v=$1
base="https://github.com/0x1ocean/tnotes/releases/download/v$v"
sum() { curl -sfL "$base/tnotes-$v-$1.tar.gz.sha256" | cut -d' ' -f1; }
mac_arm=$(sum aarch64-apple-darwin)
mac_x86=$(sum x86_64-apple-darwin)
linux_arm=$(sum aarch64-unknown-linux-gnu)
linux_x86=$(sum x86_64-unknown-linux-gnu)
cat <<EOF
class Tnotes < Formula
  desc "Terminal notes: Markdown files with folders, #tags, [[links]], a mouse-first TUI and a JSON CLI"
  homepage "https://github.com/0x1ocean/tnotes"
  version "$v"
  license "MIT"

  on_macos do
    on_arm do
      url "$base/tnotes-$v-aarch64-apple-darwin.tar.gz"
      sha256 "$mac_arm"
    end
    on_intel do
      url "$base/tnotes-$v-x86_64-apple-darwin.tar.gz"
      sha256 "$mac_x86"
    end
  end

  on_linux do
    on_arm do
      url "$base/tnotes-$v-aarch64-unknown-linux-gnu.tar.gz"
      sha256 "$linux_arm"
    end
    on_intel do
      url "$base/tnotes-$v-x86_64-unknown-linux-gnu.tar.gz"
      sha256 "$linux_x86"
    end
  end

  def install
    bin.install "tnotes"
  end

  test do
    assert_match version.to_s, shell_output("#{bin}/tnotes --version")
    system bin/"tnotes", "--dir", testpath/"vault", "new", "Brew test"
    assert_match "Brew test", shell_output("#{bin}/tnotes --dir #{testpath}/vault ls")
  end
end
EOF
