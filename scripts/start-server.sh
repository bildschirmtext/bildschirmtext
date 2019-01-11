cd ..
mkdir logs
socat -d -d tcp-l:20000,fork,reuseaddr system:"scripts/server.sh"
