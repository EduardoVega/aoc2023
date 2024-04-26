package main

import (
	"bytes"
	"fmt"
	"io"
	"os"
	"strconv"
)

func main() {
	var filename = "day4-input.txt"

	f, err := os.Open(filename)
	if err != nil {
		fmt.Println("ERROR: open file.", err)
		return
	}
	defer f.Close()

	fileInfo, err := f.Stat()
	if err != nil {
		fmt.Println("ERROR: get file info.", err)
		return
	}
	filesize := fileInfo.Size()

	var bufSize int64
	if filesize < 512 {
		bufSize = filesize
	} else {
		bufSize = int64(512)
	}

	var buf = make([]byte, bufSize)
	var off = int64(0)
	var lastLine = make([]byte, 0)
	var total int
	for {
		_, err := io.NewSectionReader(
			f,
			off,
			bufSize).Read(buf)
		if err != nil && err != io.EOF {
			fmt.Println("ERROR: read chunk.", err)
			return
		}

		var newBuf = make([]byte, 0)
		newBuf = append(newBuf, lastLine...)
		newBuf = append(newBuf, buf...)

		splitLines := bytes.Split(newBuf, []byte("\n"))
		for _, l := range splitLines[:len(splitLines)-1] {
			total += findWinningNumbers(l)
		}

		lastLine = make([]byte, len(splitLines[len(splitLines)-1]))
		for i, b := range splitLines[len(splitLines)-1] {
			lastLine[i] = b
		}

		clear(buf)

		off += bufSize
		if err == io.EOF {
			total += findWinningNumbers(lastLine)
			break
		}
	}

	fmt.Println(total)
}

func findWinningNumbers(card []byte) int {
	var storePlayed, storeWinners bool
	var played = make([]int, 0)
	var winners = make([]int, 0)

	for _, b := range bytes.Split(card, []byte(" ")) {
		if bytes.Contains(b, []byte(":")) {
			storePlayed = true
			continue
		}

		if bytes.Compare(b, []byte("|")) == 0 {
			storeWinners = true
			storePlayed = false
			continue
		}

		if storePlayed && !storeWinners {
			if n, err := strconv.Atoi(string(b)); err == nil {
				played = append(played, n)
			}
		}

		if !storePlayed && storeWinners {
			if n, err := strconv.Atoi(string(b)); err == nil {
				winners = append(winners, n)
			}
		}
	}

	var total int
	for _, p := range played {
		for _, w := range winners {
			if p == w {
				if total == 0 {
					total = 1
				} else {
					total *= 2
				}
				break
			}
		}
	}

	return total
}
