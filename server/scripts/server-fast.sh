unixtime=$(date +%s)
python3 neu-ulm.py --user=0 --page=6502 2> logs/$unixtime.log | tee logs/$unixtime.cept
