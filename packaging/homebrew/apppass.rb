class Apppass < Formula
  desc "Generate secure passwords for your applications."
  homepage "https://github.com/stescobedo92/apppass"
  url "https://github.com/stescobedo92/apppass/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "ddeef950f2e12fc83e3d1633ff911ff5db2e768e1c2d7fa44097520f0a9fe0b0"
  license "MIT"

  depends_on "rust" => :build

  option "without-tui", "Build without TUI support"
  option "without-console", "Build without CLI console support"

  def install
    args = []
    if build.without? "tui"
      args << "--no-default-features" << "--features" << "console"
    elsif build.without? "console"
      args << "--no-default-features" << "--features" << "tui"
    end

    system "cargo", "install", *std_cargo_args, *args
  end

  test do
    system "#{bin}/apppass", "--version"
  end
end
