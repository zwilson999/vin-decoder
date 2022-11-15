use std::env;
use std::collections::HashMap;

#[derive(Debug)]
struct Vin {
    _full_vin: String,
    _length_valid: bool,
    _chars_valid: bool,
    _checksum_valid: bool
}

impl Vin {
    fn _transliterate(&self) -> Vec<u32> {

        // Transliteration table for mapping characters to values
        let transliteration_map: HashMap<char, u32> = HashMap::from([
            ('A', 1),
            ('B', 2),
            ('C', 3),
            ('D', 4),
            ('E', 5),
            ('F', 6),
            ('G', 7),
            ('H', 8),
            ('J', 1),
            ('K', 2),
            ('L', 3),
            ('M', 4),
            ('N', 5),
            ('P', 7),
            ('R', 9),
            ('S', 2),
            ('T', 3),
            ('U', 4),
            ('V', 5),
            ('W', 6),
            ('X', 7),
            ('Y', 8),
            ('Z', 9)
        ]);
        
        let transliterated: Vec<u32> = self._full_vin.chars()
                                                     .map(|c| transliteration_map.get(&c).cloned().or_else(|| c.to_digit(10) ).unwrap_or(0)) // Clones the HashMap value 
                                                     .collect();
        return transliterated;
    }

    fn _weight(&self) -> Vec<u32> {
        let weight_map: HashMap<&str, u32> = HashMap::from([
            ("1", 8),
            ("2", 7),
            ("3", 6),
            ("4", 5),
            ("5", 4),
            ("6", 3),
            ("7", 2),
            ("8", 10),
            ("9", 0),
            ("10", 9),
            ("11", 8),
            ("12", 7),
            ("13", 6),
            ("14", 5),
            ("15", 4),
            ("16", 3),
            ("17", 2)
        ]);
       
        // Obtain weights based on the position on the character of the vin in the string
        let weights: Vec<u32> = self._full_vin
            .chars()
            .enumerate()
            .map(|(i, _c)| *weight_map.get( (i+1).to_string().as_str() ).unwrap_or(&0))
            .collect();
        return weights;
    }

    fn _checksum(&mut self) {
        let transliterated: Vec<u32> = self._transliterate();
        let weights: Vec<u32> = self._weight();

        // Take sum product of transliteration and weights.
        let sum_product: u32 = transliterated
            .iter()
            .zip(weights.iter())
            .map(|(x, y)| x * y)
            .sum();
        let remainder: u32 = sum_product % 11;

        // Ninth character of vin is the check digit
        let check_digit: char = self._full_vin
            .chars()
            .nth(8)
            .expect("Index out of range! Entered VIN does not have a check digit. Check the length of the VIN to be sure it is 17 digits.");
        
        //println!("Check Digit: {check_digit}, remainder: {remainder}, sum_product: {sum_product}");
        // Perform check using the check digit compared to the sum product remainder with divisor 11
        if check_digit.eq(&'X') && remainder == 10 {
            self._checksum_valid = true;
        }
        // Use radix of 10 for to_digit() to convert character to integer between 0-9
        else if check_digit.is_numeric() && remainder == check_digit.to_digit(10).unwrap_or(0) {
            self._checksum_valid = true;
        }
    }

    fn validate(&mut self) {
        // Check if VIN has I O or Q characters. These are invalid characters and are not possible in a formal VIN
        if self._full_vin.contains(['I','O','Q']) { self._chars_valid = false } else { self._chars_valid = true };

        // All VINs after 1981 should be 17 digits
        if self._full_vin.len() == 17 { self._length_valid = true };

        // This checksum algorithm is what determines if a VIN is valid based on the 9th digit of the VIN
        // See: https://en.wikibooks.org/wiki/Vehicle_Identification_Numbers_(VIN_codes)/Check_digit
        self._checksum();
    }
}

fn main() {

    // Collect user args from stdin
    let args: Vec<String> = env::args()
                                .collect();
    
    // Capture results in a struct for display
    let mut vin = Vin {
        _full_vin: args[1].to_uppercase(),
        _length_valid: false,
        _chars_valid: false,
        _checksum_valid: false
    };

    vin.validate(); 
    println!("{:?}", vin);
}
