# Warp

This lets you bookmark directories and then warp to them using a shortname.

## Installation

If you're using bash, you can use this to install it:

```bash
cat << __EOF__ >> ~/.bashrc

function warp {
  declare -A commands
  commands["list"]=1
  commands["add"]=1
  for bin in /usr/bin/warp /usr/local/bin/warp; do
    if [ -f ${bin} ]; then
      if [[ ${commands[${1}]} == 1 ]]; then
        ${bin} ${*}
      else
        cd $(${bin} ${*})
      fi
    fi
  done
}

__EOF__
```

## Usage

```bash
cd some/location/
warp add bookmarkname
```

```bash
warp bookmarkname
```

