### Computers are logic gates
 * either electricity is on or off, 1 or 0
 * binary
 * right to left, each place is 2^(that place), where places are indexed by 0 (first spot is 2^0, = 1).
 * you multiply the value (on or off, 1 or 0) by the place's value (2^zero-indexed-place-number)
 
 * 65 in decimal is like saying, 6*10 + 5 * 1. Base 10. 6*(10^1) + 5*(10^0).
 * In binary, the base is 2, in hex, 16, in decimal, 10. 
 * The multiplier is constant across all number types, based on exponent of the place's zero-based index.
 * The value in that place is in the range of the base value.

 * hex is bunching up 4 bits into 1
 * max value of 4 bits in binary is 15, min value is 0, range = 16. hence, hex.
 * think of 4 slots with 2 possible values as 2^4 permutations; 16 possible values.
 * 0100 0001 = is binary for the decimal value 65
 * 0100 0001, each 4 bit chunk in isolation, is 4 1. 
 * So hex 41 is decimal 65, and it represents 0100 0001 binary.
 * Like you use $10.00 to say ten dollars versus the symbol for Japanese Yen, you preface hex with 0x or 0+, binary with 0b.

### Bytes flow over the wire
 * TCP sends raw bytes
 * HTTP is a higher abstraction level that frames bytes with headers like memos in the mail saying "hey, here it comes!"
 * bytes come across the wire _serialized_. meaning sequential, serial, packaged up in some binary format.
 * Deserialization is taking those bytes and putting them in address space RAM.
 * So a JSON struct on the wire in binary will be binary values for "{" then the double quote, then each letter of first key then the colon, etc, all in binary.
 * One prints them as hex because it takes up 4 times less space.
 * If you have '{"name":"Alice"}' on wire and deserialize it into Rust struct:
```
struct User {
  name: String
}
```
 * In user space ram, you'll see "Alice" as the exact same hex values as you would in the wire format, but with surrounding pointer values that are specific to the operating system.

### Parsing
 * UTF-8 means universal text format 8 bit, where each byte is 8 bits.
 * Characters can be up to 4 bytes
 * ASCII (american letters and numbers) is one byte
 * In rust when you read somethign with `.lines()`  or `into_string()` you're doing CPU work of UTF-8 parsing (validating each character)
 * You're also allocating the string into the HEAP, making room for those bytes
 * You can get around both (be quick and ninja-like) using Memory Mapping but im not sure how.
 
