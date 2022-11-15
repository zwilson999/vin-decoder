import sys

WEIGHTS: dict = {
    '1': 8,
    '2': 7,
    '3': 6,
    '4': 5,
    '5': 4,
    '6': 3,
    '7': 2,
    '8': 10,
    '9': 0,
    '10': 9,
    '11': 8,
    '12': 7,
    '13': 6,
    '14': 5,
    '15': 4,
    '16': 3,
    '17': 2
}

TRANSLITERATION: dict = {
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
    'Z': 9
}

def vin_checksum(vin: str) -> bool:

    # Obtain transliteration & weights for VIN checksum
    transliteration: list[int] = [TRANSLITERATION[c] if c in TRANSLITERATION.keys() else int(c) for c in vin]
    weights: list[int] = [WEIGHTS[str(pos+1)] for pos, char in enumerate(vin)]

    # Sum the product of the element-wise transliteration and weights and take mod 11 to get remainder
    sum_prod: int = sum(a*b for a,b in zip(transliteration, weights))
    remainder: int = sum_prod % 11

    # 9th character in VIN is the check digit
    check_digit: str = vin[8]

    # If the remainder is 10 then a special case follows where the check_digit should be 'X'
    if check_digit == 'X' and remainder == 10:
        return True
    elif isinstance(check_digit, int) and remainder == int(check_digit):
        return True
    else:
        return False

def main(vin: str) -> None:
    results: dict = {
        'vin': vin,
        'valid_length': True if len(vin) == 17 else False
    }

    # Test if the VIN has any illegal characters ('I', 'O' or 'Q')
    illegal_chars: list[str] = ['I','O','Q']
    results['valid_format'] = not any(c in vin for c in illegal_chars)

    # Perform checksum algorithm
    # Source: https://en.wikibooks.org/wiki/Vehicle_Identification_Numbers_(VIN_codes)/Check_digit
    results['valid_checksum'] = vin_checksum(vin)
    print(results)

if __name__ == '__main__':
    if len(sys.argv) != 2:
        exit("Need to input a VIN arg for checksum algorithm")

    vin: str = sys.argv[1].upper().strip()
    main(vin)
