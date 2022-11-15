package main

import (
	"fmt"
	"os"
	"strings"
	"unicode"
)

type Vin struct {
	fullVin       string
	lengthValid   bool
	charsValid    bool
	checksumValid bool
}

func (vin *Vin) transliterate() []uint {
	// Lookup map for transliterating our VIN
	transliterationMap := map[rune]uint{
		'A': 1,
		'B': 2,
		'C': 3,
		'D': 4,
		'E': 5,
		'F': 6,
		'G': 7,
		'H': 8,
		'J': 1,
		'K': 2,
		'L': 3,
		'M': 4,
		'N': 5,
		'P': 7,
		'R': 9,
		'S': 2,
		'T': 3,
		'U': 4,
		'V': 5,
		'W': 6,
		'X': 7,
		'Y': 8,
		'Z': 9,
	}

	// Slice to hold transliterated uints
	var transliterated []uint
	for _, c := range vin.fullVin {
		val, isPresent := transliterationMap[c]
		if isPresent {
			transliterated = append(transliterated, val)
		} else {
			transliterated = append(transliterated, uint(c-'0'))
		}
	}
	return transliterated
}

func (vin *Vin) weight() []uint {

	// Weights map for weighting the position of each character in the VIN
	weightMap := map[uint]uint{
		1:  8,
		2:  7,
		3:  6,
		4:  5,
		5:  4,
		6:  3,
		7:  2,
		8:  10,
		9:  0,
		10: 9,
		11: 8,
		12: 7,
		13: 6,
		14: 5,
		15: 4,
		16: 3,
		17: 2,
	}

	var weights []uint
	for pos, _ := range vin.fullVin {
		weights = append(weights, weightMap[uint(pos)+1])
	}
	return weights
}

func (vin *Vin) checksum() {
	// Transliterated each char in the VIN
	transliterated := vin.transliterate()

	// Weight the position of each character in the VIN
	weights := vin.weight()

	// Perform the sum product of transliterationa nd weights to verify checksum
	var sumProd uint
	for i, _ := range weights {
		sumProd += weights[i] * transliterated[i]
	}
	remainder := sumProd % 11
	fmt.Printf("The checksum remainder is: %d\n", remainder)

	// Check digit in the VIN
	checkDigit := rune(vin.fullVin[8])
	fmt.Printf("The check digit of the VIN is %c\n", checkDigit)

	if checkDigit == 'X' && remainder == 10 {
		vin.checksumValid = true
	} else if unicode.IsNumber(checkDigit) && remainder == uint(checkDigit-'0') {
		vin.checksumValid = true
	}
	fmt.Printf("%+v\n", *vin)
}

func (vin *Vin) validate() {
	// Assess VIN length
	if len(vin.fullVin) == 17 {
		vin.lengthValid = true
	} else {
		vin.checksumValid = false
		fmt.Printf("%v\n", *vin)
	}

	// Check for invalid chars in VIN
	// Chars 'I', 'O', 'Q' are invalid VIN chars
	invalid := []string{"I", "O", "Q"}
	for _, c := range invalid {
		if !strings.Contains(vin.fullVin, c) {
			vin.charsValid = true
		}
	}

	// Perform checksum
	vin.checksum()
}

func main() {
	// Check user args
	if len(os.Args) < 2 {
		panic("Please enter valid 17 character alphanumeric VIN for decoding.")
	}

	// Run decoder
	vin := Vin{fullVin: strings.ToUpper(os.Args[1])}
	vin.validate()
}
