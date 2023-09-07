# Documentation: https://docs.brew.sh/Formula-Cookbook
#                https://rubydoc.brew.sh/Formula
# PLEASE REMOVE ALL GENERATED COMMENTS BEFORE SUBMITTING YOUR PULL REQUEST!
class Prismaviz < Formula
  desc "Visualise Prisma schema in your terminal"
  homepage "https://github.com/Viijay-Kr/prisma-visualise"
  url "https://github.com/Viijay-Kr/prisma-visualise/releases/download/v0.0.1/prismaviz-mac.tar.gz"
  sha256 "9afb2dd2fd99a9297504cce3d7a4c2c77c2bd22293538bd16ba8d59792621069"
  version "0.0.1"

  def install
    bin.install "prismaviz"
  end
end