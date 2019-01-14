unixtime=$(date +%s)
python3 neu-ulm.py --modem --baud=1200 2> logs/$unixtime-modem.log | tee logs/$unixtime-modem.cept
