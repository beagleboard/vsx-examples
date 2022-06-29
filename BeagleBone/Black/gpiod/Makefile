#
# Programs
#
all:	toggle1 toggle2 get getset getsetEvent toggleLED

toggle1: toggle1.c
	$(CC) $(LDFLAGS) -o $@ $^ -lgpiod

toggle2: toggle2.c
	$(CC) $(LDFLAGS) -o $@ $^ -lgpiod

get: get.c
	$(CC) $(LDFLAGS) -o $@ $^ -lgpiod

getset: getset.c
	$(CC) $(LDFLAGS) -o $@ $^ -lgpiod

getsetEvent: getsetEvent.c
	$(CC) $(LDFLAGS) -o $@ $^ -lgpiod

toggleLED: toggleLED.c
	$(CC) $(LDFLAGS) -o $@ $^ -lgpiod

#
# Objects
#

%.o: %.c
	$(CC) $(CFLAGS) -c $< -o $@

clean:
	rm toggle1 toggle2 get getset getsetEvent toggleLED
