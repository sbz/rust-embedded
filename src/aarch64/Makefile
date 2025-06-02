hw: hw.o
	ld -o hw hw.o
hw.o: hw.s
	as -o hw.o hw.s
clean:
	rm -f hw.o hw

run: hw
	./hw || /bin/true
