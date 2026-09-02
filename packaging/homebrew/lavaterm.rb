class Lavaterm < Formula
  desc "Terminal-native ambient lava lamp and metaball visualizer"
  homepage "https://github.com/githubuser2777/ZenLavaTerm"
  url "https://github.com/githubuser2777/ZenLavaTerm/archive/refs/tags/v1.0.0.tar.gz"
  sha256 "6d2b6de1db92f55514f244831d6882fed3687620dcec229185dc21455e8f92ff"
  license "MIT"
  head "https://github.com/githubuser2777/ZenLavaTerm.git", branch: "main"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args
  end

  test do
    output = shell_output("#{bin}/lavaterm --snapshot")
    assert_predicate output, :present?
  end
end
