make cut_btx &&
rm test/*.meta test/*.glob test/*.cept test/*.pal test/*.inc
for i in test/*; do
	echo $i
	./cut_btx $i > $i.meta
	perl -e 'truncate $ARGV[0], ((-s $ARGV[0]) -2)' $i.meta
	(echo "{"; cat $i.meta; echo; echo "}") > /tmp/tmpcut$$$; mv /tmp/tmpcut$$$ $i.meta
	python -m json.tool $i.meta > /tmp/tmpcut$$$; mv /tmp/tmpcut$$$ $i.meta
	python -m json.tool $i.glob > /tmp/tmpcut$$$; mv /tmp/tmpcut$$$ $i.glob
	if [ -f $i.pal ]; then
		python -m json.tool $i.pal > /tmp/tmpcut$$$; mv /tmp/tmpcut$$$ $i.pal
	fi
done
mv test/*.meta test/*.glob test/*.cept test/*.pal test/*.inc x
