unixtime=$(date +%s)
python3 neu-ulm.py --baud=1200 2> logs/$unixtime.log | tee logs/$unixtime.cept
