class Mhost < Formula
  desc "Flatten nested file formats like json, toml, yaml into single lines with full path to all values."
  url "https://github.com/lukaspustina/fcat/archive/v0.0.1.tar.gz"
  homepage "https://fcat.pustina.de"
  sha256 ""
  license any_of: ["MIT", "Apache-2.0"]
  head "https://github.com/lukaspustina/fcat.git"

  depends_on "rust" => :build

  def install
    system "cargo", "install", *std_cargo_args

    out_dir = Dir["target/release/build/fcat-*/out"].first
    # man1.install "#{out_dir}/man.1"
    bash_completion.install "#{out_dir}/fcat.bash"
    fish_completion.install "#{out_dir}/fcat.fish"
    zsh_completion.install "#{out_dir}/_fcat"
  end

  test do
    system "#{bin}/fcat", "#{src_dir}/Cargo.toml"
  end
end
