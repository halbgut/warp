# A wrapper around the rust binary

zshrc=~/.zshrc
bashrc=~/.bashrc
if [ -f ${zshrc} ]; then
  rcfile=${zshrc}
elif [ -f ${bashrc} ]; then
  rcfile=${bashrc}
fi

cat << '__EOF__' >> ${rcfile}

function warp {
  for bin in /usr/bin/warp /usr/local/bin/warp; do
    if [ -f ${bin} ]; then
      if [[ ${1} == "list" || ${1} == "add" ]]; then
        ${bin} ${*}
      else
        result=$(${bin} ${1})
        if [[ ${result} != "No warps found" ]]; then
          cd $(${bin} ${1})
        else
          echo ${result}
        fi
      fi
    fi
  done
}

__EOF__

echo "source ${rcfile}"

