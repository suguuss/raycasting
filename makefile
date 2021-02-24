CC = gcc
OPTS = -Wall -Wextra -std=c11 -lm
LINK = -g -fsanitize=address -fsanitize=leak
LIBS=-lSDL2
BOUT = build
SRC=src

all: main.o parsemap.o raycasting.o gfx.o graphics.o
	$(CC) $(BOUT)/main.o $(BOUT)/parsemap.o $(BOUT)/raycasting.o $(BOUT)/gfx.o $(BOUT)/graphics.o $(LIBS) $(LINK) $(OPTS) -o $(BOUT)/prog

pre-build:
	mkdir -p build

main.o: pre-build $(SRC)/main.c
	$(CC) $(OPTS) -c $(SRC)/main.c $(LINK) -o $(BOUT)/main.o

parsemap.o: pre-build $(SRC)/parsemap.c
	$(CC) $(OPTS) -c $(SRC)/parsemap.c $(LINK) -o $(BOUT)/parsemap.o

raycasting.o: pre-build $(SRC)/raycasting.c
	$(CC) $(OPTS) -c $(SRC)/raycasting.c $(LINK) -o $(BOUT)/raycasting.o

gfx.o: pre-build $(SRC)/gfx.c
	$(CC) $(OPTS) -c $(SRC)/gfx.c $(LINK) -o $(BOUT)/gfx.o

graphics.o: pre-build $(SRC)/graphics.c
	$(CC) $(OPTS) -c $(SRC)/graphics.c $(LINK) -o $(BOUT)/graphics.o

clean:
	rm -r build
