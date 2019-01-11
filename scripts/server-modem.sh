unixtime=$(date +%s)
python3 neu-ulm.py --modem 2> logs/$unixtime-modem.log | tee logs/$unixtime-modem.cept
