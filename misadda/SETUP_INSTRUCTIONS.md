# Rust Setup Instructions for Windows

It appears your environment is missing the necessary C++ build tools required to compile Rust applications.

## The Problem
- **Error:** `program not found: dlltool.exe`
- **Cause:** You are trying to compile using the GNU toolchain, but the MinGW build tools (which contain `dlltool`, `gcc`, etc.) are not in your PATH.
- **Alternative:** You technically have the MSVC toolchain installed via Rustup, but you are likely missing the actual Visual Studio C++ Build Tools (linker, compiler).

## The Solution (Recommended)

The most robust way to develop Rust on Windows is using the MSVC toolchain.

1.  **Download Visual Studio Build Tools**:
    *   Go to: [https://visualstudio.microsoft.com/visual-cpp-build-tools/](https://visualstudio.microsoft.com/visual-cpp-build-tools/)
    *   Download the installer.

2.  **Install the C++ Workload**:
    *   Run the installer.
    *   Select the **"Desktop development with C++"** workload.
    *   Ensure the following are selected in the installation details:
        *   `MSVC v143 - VS 2022 C++ x64/x86 build tools` (or similar version)
        *   `Windows 11 SDK` (or Windows 10 SDK)
    *   Click **Install**.

3.  **Verify Configuration**:
    I have already switched your default Rust toolchain to MSVC for you. Once the installation is complete, open a new terminal and run:

    ```powershell
    cargo run
    ```

    It should now compile successfully.

## Alternative Solution (MinGW)

If you prefer to verify your pure GNU setup (not recommended unless you have specific needs):
1.  Download and install **MinGW-w64** (e.g., via MSYS2 or execution installer).
2.  Add the `bin` folder of your MinGW installation to your System PATH environment variable.
3.  Switch back to the GNU toolchain: `rustup default stable-x86_64-pc-windows-gnu`.
