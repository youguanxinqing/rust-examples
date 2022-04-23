.PTHON: clean
clean:
	find . -maxdepth 1 -type d | awk '!/idea/{print "rm -rf " $$1 "/target"}' | sh