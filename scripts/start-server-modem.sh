cd ..
mkdir logs
socat -d -d tcp-l:20001,fork,reuseaddr system:"scripts/server-modem.sh"
