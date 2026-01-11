class Apppass < Formula
  desc "Generate secure passwords for your applications."
  homepage "https://github.com/stescobedo92/apppass"
  url "https://github.com/stescobedo92/apppass/archive/refs/tags/v1.0.1.tar.gz"
  sha256 "11915381500f2c0adc3b9a893027eb90cc37598fd2e639dec5d5921a8e7d9df5"
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

    system "cargo", "install", *std_cargo_args(args: args)
  end

  test do
    system "#{bin}/apppass", "--version"
  end
end
