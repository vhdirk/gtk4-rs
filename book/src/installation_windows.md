# Windows

When preparing your windows machine you have to decide between using the **MSVC toolchain** or  the **GNU toolchain**.

In both cases you will have to install the rust toolchain via [rustup](https://rustup.rs/).


## MSVC toolchain

### Visual Studio

Install Visual Studio from https://visualstudio.microsoft.com/vs/.
Make sure to check the box "Desktop development with C++" during the installation process.

<div style="text-align:center"><img src="img/vs-install.png" /></div>

### Set PKG_CONFIG_PATH environment variable

1. Go to settings -> Search and open `Advanced system settings` -> click on `Environment variables`
2. Under `User variables` click on `New` and add:

- Variable name: `PKG_CONFIG_PATH`
- Variable value: `C:\gnome\lib\pkgconfig;C:\gstreamer\1.0\msvc_x86_64\lib\pkgconfig`



### GStreamer

1. Install gstreamer from [gstreamer.freedesktop.org](https://gstreamer.freedesktop.org/download/) using both runtime and development installers.
You might need to select right-click -> Properties -> Security: "Unblock" after download.

2. Go to settings -> Search and open `Advanced system settings` -> click on `Environment variables`
3. Select `Path` -> Click on `Edit` -> Add the following three entries:

```
C:\gstreamer\1.0\msvc_x86_64\include
C:\gstreamer\1.0\msvc_x86_64\bin
C:\gstreamer\1.0\msvc_x86_64\lib
```


### Dependencies to install via the command line

Then, make sure that you have `git`, `python`, `pkg-config`, `ninja` and `meson` in your `PATH`.
One way to do that is by requesting the version of the program in your terminal and check if it outputs an error:

```powershell
git --version
python --version
pkg-config --version
ninja --version
meson --version
```

If one of them is missing install the package manager `scoop` by executing the following in the powershell terminal:

```powershell
iwr -useb get.scoop.sh | iex
```

Then install the missing dependencies with

```powershell
scoop install git python pkg-config ninja
```

If `meson` was missing as well, install it next with

```powershell
pip install meson
```

### Compile and install GTK4

From the Windows start menu, search for `x64 Native Tools Command Prompt for VS 2019`.
That will open a terminal configured to use MSVC x64 tools.
From there run the following commands:

```powershell
cd /
git clone https://gitlab.gnome.org/GNOME/gtk.git --depth 1
cd gtk
meson setup builddir -Dbuild-tests=false --prefix C:/gnome
meson install -C builddir
```

## GNU toolchain


### Dependencies

1. Install the rust toolchain via [rustup](https://rustup.rs/)
2. Install MSYS2 from [www.msys2.org](https://www.msys2.org/) 

From the Windows start menu, search for `MSYS2 MinGW 64-bit`.
That will open a terminal configured to use MinGW x64 tools.

There execute the following commands to install `GTK 4`, `pkgconf` and `gcc`.

```sh
pacman -S mingw-w64-x86_64-gtk4 mingw-w64-x86_64-pkgconf mingw-w64-x86_64-gcc
```


### Update Path environment variable

1. Go to settings -> Search and open `Advanced system settings` -> click on `Environment variables`
2. Select `Path` -> Click on `Edit` -> Add the following three entries:
 
```
C:\msys64\mingw64\include
C:\msys64\mingw64\bin
C:\msys64\mingw64\lib
```

### Setup the `windows-gnu` toolchain for Rust

The default toolchain on windows is `stable-msvc`.
To switch to `stable-gnu`, run the following commands from your terminal:

1. `rustup toolchain install stable-gnu`
2. `rustup default stable-gnu`

Please note that this command might change in the future.
If it does not work anymore please open an issue [here](https://github.com/gtk-rs/gtk4-rs/issues/new/choose).

### Install GTK

Run the following command from your **MSYS2 terminal**:




