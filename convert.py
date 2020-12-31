import json
import sys
import pprint

filename = sys.argv[1];

with open(filename) as f:
	meta = json.load(f)
	pprint.pprint(meta)
	if 'links' in meta:
		links = meta['links']
		links_new = []
		for key in links:
#			pprint.pprint(key + " + " + links[key])
			links_new.append({ "code": key, "target": links[key] })
		meta['links'] = links_new
	pprint.pprint(meta)
	with open(sys.argv[2], 'w') as f2:
		json.dump(meta, f2)
