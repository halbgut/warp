# Warp

This lets you bookmark directories and then warp to them using the given name.

## Installation

If you're using bash, you can use this to install it:

```bash
git clone https://github.com/Kriegslustig/warp.git
$(warp/install.bash)
sudo cp warp/target/release/warp /usr/bin/
rm -r warp
```

### Mac OS > 10.10

Copying the binary to /usr/bin doesn't work on Mac OS El Capitan (10.11). This is due to the System Integrity Protection feature. It protects some directories from beeing writen to. To circumvent this, it's best to install [Brew](http://brew.sh/). If you aren't using Brew already, it could be easier to create a directory to put your custom binaries in. Maybe something like `/Users/yourusername/bin`. You'd need to run the following commands in order for this to work.

```bash
mkdir ~/bin
cp warp/warp ~/bin/
echo 'export '$PATH="$PATH:~/bin"' >> ~/.bashrc
```

If you're using something like ZSH you'll need to modify the last line.

## Usage

```bash
cd some/location/
warp add bookmarkname
```

```bash
warp bookmarkname
```

```bash
warp list
```

