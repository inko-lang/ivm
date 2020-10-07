#!/usr/bin/env ruby
# rubocop: disable all
# frozen_string_literal: true

require 'English'

new_version = ARGV[0]

if !new_version || new_version.empty?
  abort 'You must specify a new version as the first argument'
end

old_toml = File.read('Cargo.toml')
new_toml = old_toml.gsub(
  /version.+# VERSION/,
  %(version = "#{new_version}" # VERSION)
)

File.open('Cargo.toml', 'w') do |handle|
  handle.write(new_toml)
end

# Make sure that Cargo.lock is also updated
output = `cargo check 2>&1`

unless $CHILD_STATUS.success?
  abort "Failed to update Cargo.lock files: #{output}"
end
