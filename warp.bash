# A wrapper around the rust binary

function warp {
  declare -A commands
  commands["list"]=1
  commands["add"]=1
  for bin in /usr/bin/warp /usr/local/bin/warp; do
    if [ -f ${bin} ]; then
      if [ -n commands[${1}] ]; then
        echo $(${bin} ${*})
      else
        ${bin} {*}
      fi
    fi
  done
}

