# Tokio Locking Example

This is a minimal reproduction of an issue involving the Tokio runtime and the latest Linux kernel versions.
It only seems to occur in very specific scenarios:
- It only occurs on certain 6.14.x Kernels (most of my testing was done on 6.14.4)
- Running under Linux (I can confirm Arch Linux is affected. Other distros running newer kernels may also be affected)
- Using `tokio::sync::Mutex`/`tokio::sync::RwLock`'s within many tasks.
- Most/all tasks must have calls to `tokio::time::sleep`.

This is not a deadlock (at least not in the traditional sense) since this is recoverable.
By sending certains signals to the process, it wakes up and resumes all tasks.

## What Happens

```
cargo run --release
```

- Eventually the program halts, and all tasks stop
- One task enters a spinlock (100% CPU usage)
- All other tasks have 0% CPU usage

## Recovery Examples

The program recovers under these situations:
- An API call (if the program is listening on a port)
- Specifically _detaching_ `gdb` (Attaching does not wake it)
- Others? (waiting on confirmation for whether it is awoken by Unix signals e.g. SIGURG)

## Other Notes

- Running an additional single non-sleeping task appears to cause the issue to occur much more frequently. This task also runs at 100% CPU during the freeze.
- It occurs on both Intel and AMD CPUs
- It _seems_ that machines with 2 threads or less are unable to reproduce it.
- Machines with more CPU cores/threads take longer to reproduce it.
- Removing the call to `tokio::time::sleep` prevents the freeze from ever happening.
- I didn't check older versions of Tokio but I can confirm it occurs on `1.45.0` and on `1.1.1`.

Backtrace during frozen state is available here: https://gist.github.com/macladson/88a4bcf51e5e2630dd2183f53a4ff4b9
