1.9.131:

 * Do not wait for fill whole buffer on read. It's usefull for read network stream (thanks to Brian Vincent)

1.8.131:

 * Update lz4 to v131
 * Fix incorrect break that could cause reading after a frame ends (thanks to Brian Vincent)
 * Fix typo in Cargo.toml

1.7.129:

 * Autopublish rustdoc
 * Remove libc type publishing

1.6.129:

 * Update lz4 to r129
 * Add tests
 * Rustup: 1.0.0-beta