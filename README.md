# Whython-7

Requires `link.exe` (MSVC) and `nasm.exe` to be in `PATH`. `main.why` will be compiled
into the binary found in `out.exe`.

When on Linux, `gcc` will be used for linking and `wine` for execution (both of which need to be in `PATH`).
`nasm.exe` is still required.

Designed to work on Windows linking with `kernel32.lib` for system calls.
