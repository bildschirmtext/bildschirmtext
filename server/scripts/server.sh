unixtime=$(date +%s)
python3 neu-ulm.py 2> logs/$unixtime.log | tee logs/$unixtime.cept
