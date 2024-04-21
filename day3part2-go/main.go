package main

import (
	"fmt"
	"os"
	"strconv"
)

// * symbol adjacent to two numbers
// gear ratio is mult those two num
// add up all ratios

type GearInformation struct {
	Line   int
	Column int
}

type NumberInformation struct {
	Number     string
	Line       int
	StartIndex int
}

func main() {
	inputFile := "../day3-input.txt"
	bytes, err := os.ReadFile(inputFile)
	if err != nil {
		fmt.Println(err)
		os.Exit(1)
	}

	var gearInformationList = make([]GearInformation, 0)
	var numberInformationList = make([]NumberInformation, 0)

	var line int
	var column int

	var concatenatedNumber string
	var concatenatedNumberStartIndex int

	for _, b := range bytes {
		// New line.
		if b == '\n' {
			if len(concatenatedNumber) > 0 {
				numberInformationList = append(numberInformationList, NumberInformation{
					Number:     concatenatedNumber,
					Line:       line,
					StartIndex: concatenatedNumberStartIndex,
				})

				concatenatedNumber = ""
				concatenatedNumberStartIndex = 0
			}

			line++
			column = 0
			continue
		}

		// Search for gears and save their location.
		if b == '*' {
			gearInformationList = append(gearInformationList, GearInformation{
				Line:   line,
				Column: column,
			})
		}

		// Search for numbers.
		if number, ok := isNumber(b); ok {
			if len(concatenatedNumber) == 0 {
				concatenatedNumberStartIndex = column
			}

			concatenatedNumber = concatenatedNumber + number
		} else {
			if len(concatenatedNumber) > 0 {
				numberInformationList = append(numberInformationList, NumberInformation{
					Number:     concatenatedNumber,
					Line:       line,
					StartIndex: concatenatedNumberStartIndex,
				})

				concatenatedNumber = ""
				concatenatedNumberStartIndex = 0
			}
		}

		column++
	}

	if len(concatenatedNumber) > 0 {
		numberInformationList = append(numberInformationList, NumberInformation{
			Number:     concatenatedNumber,
			Line:       line,
			StartIndex: concatenatedNumberStartIndex,
		})

		concatenatedNumber = ""
		concatenatedNumberStartIndex = 0
	}

	//fmt.Println(gearInformationList)
	//fmt.Println(numberInformationList)

	var total int
	for _, gear := range gearInformationList {
		var gearRatio = make([]int, 0)

		for _, number := range numberInformationList {
			// Adjacent numbers can only be in the before, same or after line as the gear line.
			//fmt.Printf("%v --- %v\n", number, gear)
			if number.Line >= gear.Line-1 && number.Line <= gear.Line+1 {
				//fmt.Printf("LINE %s\n", number.Number)
				if gear.Column >= number.StartIndex-1 && gear.Column <= len(number.Number)+number.StartIndex {
					n, err := strconv.Atoi(number.Number)
					if err != nil {
						fmt.Printf("ERROR: can not convert string %s to int\n", number.Number)
						os.Exit(1)
					}

					gearRatio = append(gearRatio, n)
					//fmt.Printf("AD --> %v\n", gearRatio)
				}
			}
		}

		// ONly If there are two adjacent numbers  o the gear we multiple them.
		if len(gearRatio) == 2 {
			fmt.Printf("Gear line: %d column: %d. Adjacent numbers: %d %d\n",
				gear.Line,
				gear.Column,
				gearRatio[0],
				gearRatio[1])

			total += (gearRatio[0] * gearRatio[1])
		} else {
			fmt.Printf("Gear line: %d column: %d. Not a gear\n",
				gear.Line,
				gear.Column)
		}
	}

	fmt.Println(total)
}

func isNumber(b byte) (string, bool) {
	switch string(b) {
	case "0", "1", "2", "3", "4", "5", "6", "7", "8", "9":
		return string(b), true
	}

	return "", false
}
