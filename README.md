## VIN Checksum Program

This is a simple suite of programs to take in a single VIN argument and check it for valid characters, valid length as well as perform a VIN checksum algorithm to validate if the VIN was entered correctly. The implementations are available in Python, Rust & Go.


## Usage (Python)


```
# In terminal
python decoder.py 4Y1SL65848Z411439

# Sample output
{'vin': '4Y1SL65848Z411439', 'valid_length': True, 'valid_format': True, 'valid_checksum': False}
```


## Usage (Rust)

```
// In terminal
cargo run 4Y1SL65848Z411439

// Sample output
Vin { _full_vin: "4Y1SL65848Z411439", _length_valid: true, _chars_valid: true, _checksum_valid: false }
```

## Usage (Go)

```
// In terminal
go run vin_decoder.go 4Y1SL65848Z411439

// Sample output
{fullVin:4Y1SL65848Z411439, lengthValid:true, charsValid:true, checksumValid:false}
```


## Read more
Read more about the checksum algorithm here: https://en.wikibooks.org/wiki/Vehicle_Identification_Numbers_(VIN_codes)/Check_digit

