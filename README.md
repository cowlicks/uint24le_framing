# ⚠️  WARNING 🚧 API unstable ⚒️  and still in development 👷

This small crate takes `AsyncRead` / `AsyncWrite` IO and turns it into a `Stream` and `Sink`.
It prefixes messages with a little endian unsigned 24 bit integers.
Used to frame messages between Hypercore peers.
