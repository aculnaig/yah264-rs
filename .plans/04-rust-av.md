# 04 — rust-av integration

## Outcomes

- A `av-yah264` crate that implements the agreed rust-av encoder integration point.
- Clear format, timestamp, packet, configuration, and error mappings.

## Tasks

1. Pin the rust-av revision and implement the codec crate's `Encoder` trait.
2. Adapt the safe `yah264` wrapper; do not add a separate encoder implementation.
3. Map rust-av 8-bit YUV420P frame representations and timestamps to encoder inputs without format conversion.
4. Copy emitted NAL bytes into rust-av's owned `Packet.data` vector and map typed errors to rust-av's error type.
5. Define handling for unsupported formats, conversion, flush/end-of-stream, and encoder reconfiguration.
6. Add rustdoc examples and unit tests for the adapter's own mapping logic; leave end-to-end integration tests for a later phase.

## Acceptance

- The crate builds against the selected rust-av version.
- Supported mappings and unsupported cases are documented.
- Packet, timestamp, and error behavior follow rust-av conventions.
