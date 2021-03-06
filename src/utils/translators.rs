pub fn char_to_u8(val: char) -> u8 {
    match val {
    ' ' => 0x20,
    '!' => 0x21,
    '\"' => 0x22,
    '#' => 0x23,
    '$' => 0x24,
    '%' => 0x25,
    '&' => 0x26,
    '\'' => 0x27,
    '(' => 0x28,
    ')' => 0x29,
    '*' => 0x2A,
    '+' => 0x2B,
    ',' => 0x2C,
    '-' => 0x2D,
    '.' => 0x2E,
    '/' => 0x2F,
    '0' => 0x30,
    '1' => 0x31,
    '2' => 0x32,
    '3' => 0x33,
    '4' => 0x34,
    '5' => 0x35,
    '6' => 0x36,
    '7' => 0x37,
    '8' => 0x38,
    '9' => 0x39,
    ':' => 0x3A,
    ';' => 0x3B,
    '<' => 0x3C,
    '=' => 0x3D,
    '>' => 0x3E,
    '?' => 0x3F,
    '@' => 0x40,
    'A' => 0x41,
    'B' => 0x42,
    'C' => 0x43,
    'D' => 0x44,
    'E' => 0x45,
    'F' => 0x46,
    'G' => 0x47,
    'H' => 0x48,
    'I' => 0x49,
    'J' => 0x4A,
    'K' => 0x4B,
    'L' => 0x4C,
    'M' => 0x4D,
    'N' => 0x4E,
    'O' => 0x4F,
    'P' => 0x50,
    'Q' => 0x51,
    'R' => 0x52,
    'S' => 0x53,
    'T' => 0x54,
    'U' => 0x55,
    'V' => 0x56,
    'W' => 0x57,
    'X' => 0x58,
    'Y' => 0x59,
    'Z' => 0x5A,
    '[' => 0x5B,
    '\\' => 0x5C,
    ']' => 0x5D,
    '^' => 0x5E,
    '_' => 0x5F,
    '`' => 0x60,
    'a' => 0x61,
    'b' => 0x62,
    'c' => 0x63,
    'd' => 0x64,
    'e' => 0x65,
    'f' => 0x66,
    'g' => 0x67,
    'h' => 0x68,
    'i' => 0x69,
    'j' => 0x6A,
    'k' => 0x6B,
    'l' => 0x6C,
    'm' => 0x6D,
    'n' => 0x6E,
    'o' => 0x6F,
    'p' => 0x70,
    'q' => 0x71,
    'r' => 0x72,
    's' => 0x73,
    't' => 0x74,
    'u' => 0x75,
    'v' => 0x76,
    'w' => 0x77,
    'x' => 0x78,
    'y' => 0x79,
    'z' => 0x7A,
    '{' => 0x7B,
    '|' => 0x7C,
    '}' => 0x7D,
    '~' => 0x7E,
    _ => panic!("Error in encoding hexadecimal values"),
    }
}

pub fn ascii_to_hex(val: &str) -> u32 {
    match val {
    "NUL" => 0x0,
    "SOH" => 0x1,
    "STX" => 0x2,
    "ETX" => 0x3,
    "EOT" => 0x4,
    "ENQ" => 0x5,
    "ACK" => 0x6,
    "BEL" => 0x7,
    "BS" => 0x8,
    "HT" => 0x9,
    "LF" => 0x0A,
    "VT" => 0x0B,
    "FF" => 0x0C,
    "CR" => 0x0D,
    "SO" => 0x0E,
    "SI" => 0x0F,
    "DLE" => 0x10,
    "DC1" => 0x11,
    "DC2" => 0x12,
    "DC3" => 0x13,
    "DC4" => 0x14,
    "NAK" => 0x15,
    "SYN" => 0x16,
    "ETB" => 0x17,
    "CAN" => 0x18,
    "EM" => 0x19,
    "SUB" => 0x1A,
    "ESC" => 0x1B,
    "FS" => 0x1C,
    "GS" => 0x1D,
    "RS" => 0x1E,
    "US" => 0x1F,
    "space" => 0x20,
    "!" => 0x21,
    "\"" => 0x22,
    "#" => 0x23,
    "$" => 0x24,
    "%" => 0x25,
    "&" => 0x26,
    "'" => 0x27,
    "(" => 0x28,
    ")" => 0x29,
    "*" => 0x2A,
    "+" => 0x2B,
    "," => 0x2C,
    "-" => 0x2D,
    "." => 0x2E,
    "/" => 0x2F,
    "0" => 0x30,
    "1" => 0x31,
    "2" => 0x32,
    "3" => 0x33,
    "4" => 0x34,
    "5" => 0x35,
    "6" => 0x36,
    "7" => 0x37,
    "8" => 0x38,
    "9" => 0x39,
    ":" => 0x3A,
    ";" => 0x3B,
    "<" => 0x3C,
    "=" => 0x3D,
    ">" => 0x3E,
    "?" => 0x3F,
    "@" => 0x40,
    "A" => 0x41,
    "B" => 0x42,
    "C" => 0x43,
    "D" => 0x44,
    "E" => 0x45,
    "F" => 0x46,
    "G" => 0x47,
    "H" => 0x48,
    "I" => 0x49,
    "J" => 0x4A,
    "K" => 0x4B,
    "L" => 0x4C,
    "M" => 0x4D,
    "N" => 0x4E,
    "O" => 0x4F,
    "P" => 0x50,
    "Q" => 0x51,
    "R" => 0x52,
    "S" => 0x53,
    "T" => 0x54,
    "U" => 0x55,
    "V" => 0x56,
    "W" => 0x57,
    "X" => 0x58,
    "Y" => 0x59,
    "Z" => 0x5A,
    "[" => 0x5B,
    "\\" => 0x5C,
    "]" => 0x5D,
    "^" => 0x5E,
    "_" => 0x5F,
    "`" => 0x60,
    "a" => 0x61,
    "b" => 0x62,
    "c" => 0x63,
    "d" => 0x64,
    "e" => 0x65,
    "f" => 0x66,
    "g" => 0x67,
    "h" => 0x68,
    "i" => 0x69,
    "j" => 0x6A,
    "k" => 0x6B,
    "l" => 0x6C,
    "m" => 0x6D,
    "n" => 0x6E,
    "o" => 0x6F,
    "p" => 0x70,
    "q" => 0x71,
    "r" => 0x72,
    "s" => 0x73,
    "t" => 0x74,
    "u" => 0x75,
    "v" => 0x76,
    "w" => 0x77,
    "x" => 0x78,
    "y" => 0x79,
    "z" => 0x7A,
    "{" => 0x7B,
    "|" => 0x7C,
    "}" => 0x7D,
    "~" => 0x7E,
    "DEL" => 0x7F,
    _ => panic!("Error in encoding hexadecimal values"),
    }
}

pub fn hex_string_to_ascii(val: String) -> char {
    // dbg!(&val);
    match val.as_str() {
    "20"	=> ' ',
    "21"	=> '!',
    "22"	=> '\\',
    "23"	=> '#',
    "24"	=> '$',
    "25"	=> '%',
    "26"	=> '&',
    "27"	=> '\'',
    "28"	=> '(',
    "29"	=> ')',
    "2A"	=> '*',
    "2B"	=> '+',
    "2C"	=> ',',
    "2D"	=> '-',
    "2E"	=> '.',
    "2F"	=> '/',
    "30"	=> '0',
    "31"	=> '1',
    "32"	=> '2',
    "33"	=> '3',
    "34"	=> '4',
    "35"	=> '5',
    "36"	=> '6',
    "37"	=> '7',
    "38"	=> '8',
    "39"	=> '9',
    "3A"	=> ':',
    "3B"	=> ';',
    "3C"	=> '<',
    "3D"	=> '=',
    "3E"	=> '>',
    "3F"	=> '?',
    "40"	=> '@',
    "41"	=> 'A',
    "42"	=> 'B',
    "43"	=> 'C',
    "44"	=> 'D',
    "45"	=> 'E',
    "46"	=> 'F',
    "47"	=> 'G',
    "48"	=> 'H',
    "49"	=> 'I',
    "4A"	=> 'J',
    "4B"	=> 'K',
    "4C"	=> 'L',
    "4D"	=> 'M',
    "4E"	=> 'N',
    "4F"	=> 'O',
    "50"	=> 'P',
    "51"	=> 'Q',
    "52"	=> 'R',
    "53"	=> 'S',
    "54"	=> 'T',
    "55"	=> 'U',
    "56"	=> 'V',
    "57"	=> 'W',
    "58"	=> 'X',
    "59"	=> 'Y',
    "5A"	=> 'Z',
    "5B"	=> '[',
    "5C"	=> '\\',
    "5D"	=> ']',
    "5E"	=> '^',
    "5F"	=> '_',
    "60"	=> '`',
    "61"	=> 'a',
    "62"	=> 'b',
    "63"	=> 'c',
    "64"	=> 'd',
    "65"	=> 'e',
    "66"	=> 'f',
    "67"	=> 'g',
    "68"	=> 'h',
    "69"	=> 'i',
    "6A"	=> 'j',
    "6B"	=> 'k',
    "6C"	=> 'l',
    "6D"	=> 'm',
    "6E"	=> 'n',
    "6F"	=> 'o',
    "70"	=> 'p',
    "71"	=> 'q',
    "72"	=> 'r',
    "73"	=> 's',
    "74"	=> 't',
    "75"	=> 'u',
    "76"	=> 'v',
    "77"	=> 'w',
    "78"	=> 'x',
    "79"	=> 'y',
    "7A"	=> 'z',
    "7B"	=> '{',
    "7C"	=> '|',
    "7D"	=> '}',
    "7E"	=> '~',
    "80"    => ' ',
    "81"    => ' ',
    "82"    => '‚',
    "83"    => 'ƒ',
    "84"    => '„',
    "85"    => '…',
    "86"    => '†',
    "87"    => '‡',
    "88"    => 'ˆ',
    "89"    => '‰',
    "8A"    => 'Š',
    "8B"    => '‹',
    "8C"    => 'Œ',
    "8D"    => ' ',
    "8E"    => ' ',
    "8F"    => ' ',
    "90"    => ' ',
    "91"    => '‘',
    "92"    => '’',
    "93"    => '“',
    "94"    => '”',
    "95"    => '•',
    "96"    => '–',
    "97"    => '—',
    "98"    => '˜',
    "99"    => '™',
    "9A"    => 'š',
    "9B"    => '›',
    "9C"    => 'œ',
    "9D"    => ' ',
    "9E"    => ' ',
    "9F"    => 'Ÿ',
    "A0"    => ' ',
    "A1"    => '¡',
    "A2"    => '¢',
    "A3"    => '£',
    "A4"    => '¤',
    "A5"    => '¥',
    "A6"    => '¦',
    "A7"    => '§',
    "A8"    => '¨',
    "A9"    => '©',
    "AA"    => 'ª',
    "AB"    => '«',
    "AC"    => '¬',
    "AD"    => '­',
    "AE"    => '®',
    "AF"    => '¯',
    "B0"    => '°',
    "B1"    => '±',
    "B2"    => '²',
    "B3"    => '³',
    "B4"    => '´',
    "B5"    => 'µ',
    "B6"    => '¶',
    "B7"    => '·',
    "B8"    => '¸',
    "B9"    => '¹',
    "BA"    => 'º',
    "BB"    => '»',
    "BC"    => '¼',
    "BD"    => '½',
    "BE"    => '¾',
    "BF"    => '¿',
    "C0"    => 'À',
    "C1"    => 'Á',
    "C2"    => 'Â',
    "C3"    => 'Ã',
    "C4"    => 'Ä',
    "C5"    => 'Å',
    "C6"    => 'Æ',
    "C7"    => 'Ç',
    "C8"    => 'È',
    "C9"    => 'É',
    "CA"    => 'Ê',
    "CB"    => 'Ë',
    "CC"    => 'Ì',
    "CD"    => 'Í',
    "CE"    => 'Î',
    "CF"    => 'Ï',
    "D0"    => 'Ð',
    "D1"    => 'Ñ',
    "D2"    => 'Ò',
    "D3"    => 'Ó',
    "D4"    => 'Ô',
    "D5"    => 'Õ',
    "D6"    => 'Ö',
    "D7"    => '×',
    "D8"    => 'Ø',
    "D9"    => 'Ù',
    "DA"    => 'Ú',
    "DB"    => 'Û',
    "DC"    => 'Ü',
    "DD"    => 'Ý',
    "DE"    => 'Þ',
    "DF"    => 'ß',
    "E0"    => 'à',
    "E1"    => 'á',
    "E2"    => 'â',
    "E3"    => 'ã',
    "E4"    => 'ä',
    "E5"    => 'å',
    "E6"    => 'æ',
    "E7"    => 'ç',
    "E8"    => 'è',
    "E9"    => 'é',
    "EA"    => 'ê',
    "EB"    => 'ë',
    "EC"    => 'ì',
    "ED"    => 'í',
    "EE"    => 'î',
    "EF"    => 'ï',
    "F0"    => 'ð',
    "F1"    => 'ñ',
    "F2"    => 'ò',
    "F3"    => 'ó',
    "F4"    => 'ô',
    "F5"    => 'õ',
    "F6"    => 'ö',
    "F7"    => '÷',
    "F8"    => 'ø',
    "F9"    => 'ù',
    "FA"    => 'ú',
    "FB"    => 'û',
    "FC"    => 'ü',
    "FD"    => 'ý',
    "FE"    => 'þ',
    "FF"    => 'ÿ',
    _ => ' ',
    }
}

pub fn hex_val_to_ascii(val: u32) -> (u8, &'static str) {
    match val {
    0x0    => (0b00000000, "NUL"),
    0x1    => (0b00000001, "SOH"),
    0x2    => (0b00000010, "STX"),
    0x3    => (0b00000011, "ETX"),
    0x4    => (0b00000100, "EOT"),
    0x5    => (0b00000101, "ENQ"),
    0x6    => (0b00000110, "ACK"),
    0x7    => (0b00000111, "BEL"),
    0x8    => (0b00001000, "BS"),
    0x9    => (0b00001001, "HT"),
    0x0A	=> (0b00001010, "LF"),
    0x0B	=> (0b00001011, "VT"),
    0x0C	=> (0b00001100, "FF"),
    0x0D	=> (0b00001101, "CR"),
    0x0E	=> (0b00001110, "SO"),
    0x0F	=> (0b00001111, "SI"),
    0x10	=> (0b00010000, "DLE"),
    0x11	=> (0b00010001, "DC1"),
    0x12	=> (0b00010010, "DC2"),
    0x13	=> (0b00010011, "DC3"),
    0x14	=> (0b00010100, "DC4"),
    0x15	=> (0b00010101, "NAK"),
    0x16	=> (0b00010110, "SYN"),
    0x17	=> (0b00010111, "ETB"),
    0x18	=> (0b00011000, "CAN"),
    0x19	=> (0b00011001, "EM"),
    0x1A	=> (0b00011010, "SUB"),
    0x1B	=> (0b00011011, "ESC"),
    0x1C	=> (0b00011100, "FS"),
    0x1D	=> (0b00011101, "GS"),
    0x1E	=> (0b00011110, "RS"),
    0x1F	=> (0b00011111, "US"),
    0x20	=> (0b00100000, "space"),
    0x21	=> (0b00100001, "!"),
    0x22	=> (0b00100010, "\""),
    0x23	=> (0b00100011, "#"),
    0x24	=> (0b00100100, "$"),
    0x25	=> (0b00100101, "%"),
    0x26	=> (0b00100110, "&"),
    0x27	=> (0b00100111, "\""),
    0x28	=> (0b00101000, "("),
    0x29	=> (0b00101001, ")"),
    0x2A	=> (0b00101010, "*"),
    0x2B	=> (0b00101011, "+"),
    0x2C	=> (0b00101100, ","),
    0x2D	=> (0b00101101, "-"),
    0x2E	=> (0b00101110, "."),
    0x2F	=> (0b00101111, "/"),
    0x30	=> (0b00110000, "0"),
    0x31	=> (0b00110001, "1"),
    0x32	=> (0b00110010, "2"),
    0x33	=> (0b00110011, "3"),
    0x34	=> (0b00110100, "4"),
    0x35	=> (0b00110101, "5"),
    0x36	=> (0b00110110, "6"),
    0x37	=> (0b00110111, "7"),
    0x38	=> (0b00111000, "8"),
    0x39	=> (0b00111001, "9"),
    0x3A	=> (0b00111010, ":"),
    0x3B	=> (0b00111011, ";"),
    0x3C	=> (0b00111100, "<"),
    0x3D	=> (0b00111101, "="),
    0x3E	=> (0b00111110, ">"),
    0x3F	=> (0b00111111, "?"),
    0x40	=> (0b01000000, "@"),
    0x41	=> (0b01000001, "A"),
    0x42	=> (0b01000010, "B"),
    0x43	=> (0b01000011, "C"),
    0x44	=> (0b01000100, "D"),
    0x45	=> (0b01000101, "E"),
    0x46	=> (0b01000110, "F"),
    0x47	=> (0b01000111, "G"),
    0x48	=> (0b01001000, "H"),
    0x49	=> (0b01001001, "I"),
    0x4A	=> (0b01001010, "J"),
    0x4B	=> (0b01001011, "K"),
    0x4C	=> (0b01001100, "L"),
    0x4D	=> (0b01001101, "M"),
    0x4E	=> (0b01001110, "N"),
    0x4F	=> (0b01001111, "O"),
    0x50	=> (0b01010000, "P"),
    0x51	=> (0b01010001, "Q"),
    0x52	=> (0b01010010, "R"),
    0x53	=> (0b01010011, "S"),
    0x54	=> (0b01010100, "T"),
    0x55	=> (0b01010101, "U"),
    0x56	=> (0b01010110, "V"),
    0x57	=> (0b01010111, "W"),
    0x58	=> (0b01011000, "X"),
    0x59	=> (0b01011001, "Y"),
    0x5A	=> (0b01011010, "Z"),
    0x5B	=> (0b01011011, "["),
    0x5C	=> (0b01011100, "\\"),
    0x5D	=> (0b01011101, "]"),
    0x5E	=> (0b01011110, "^"),
    0x5F	=> (0b01011111, "_"),
    0x60	=> (0b01100000, "`"),
    0x61	=> (0b01100001, "a"),
    0x62	=> (0b01100010, "b"),
    0x63	=> (0b01100011, "c"),
    0x64	=> (0b01100100, "d"),
    0x65	=> (0b01100101, "e"),
    0x66	=> (0b01100110, "f"),
    0x67	=> (0b01100111, "g"),
    0x68	=> (0b01101000, "h"),
    0x69	=> (0b01101001, "i"),
    0x6A	=> (0b01101010, "j"),
    0x6B	=> (0b01101011, "k"),
    0x6C	=> (0b01101100, "l"),
    0x6D	=> (0b01101101, "m"),
    0x6E	=> (0b01101110, "n"),
    0x6F	=> (0b01101111, "o"),
    0x70	=> (0b01110000, "p"),
    0x71	=> (0b01110001, "q"),
    0x72	=> (0b01110010, "r"),
    0x73	=> (0b01110011, "s"),
    0x74	=> (0b01110100, "t"),
    0x75	=> (0b01110101, "u"),
    0x76	=> (0b01110110, "v"),
    0x77	=> (0b01110111, "w"),
    0x78	=> (0b01111000, "x"),
    0x79	=> (0b01111001, "y"),
    0x7A	=> (0b01111010, "z"),
    0x7B	=> (0b01111011, "{"),
    0x7C	=> (0b01111100, "|"),
    0x7D	=> (0b01111101, "}"),
    0x7E	=> (0b01111110, "~"),
    0x7F	=> (0b01111111, "DEL"),
    _ => panic!("Error in encoding hexadecimal values"),
    }
}

pub fn u32_to_hex(val: u32) -> &'static str {
    match val {
        0b0000_0000 => "0" ,
        0b0000_0001 => "1" ,
        0b0000_0010 => "2" ,
        0b0000_0011 => "3" ,
        0b0000_0100 => "4" ,
        0b0000_0101 => "5" ,
        0b0000_0110 => "6" ,
        0b0000_0111 => "7" ,
        0b0000_1000 => "8" ,
        0b0000_1001 => "9" ,
    	0b0000_1010 => "0A",
    	0b0000_1011 => "0B",
    	0b0000_1100 => "0C",
    	0b0000_1101 => "0D",
    	0b0000_1110 => "0E",
    	0b0000_1111 => "0F",
    	0b0001_0000 => "10",
    	0b0001_0001 => "11",
    	0b0001_0010 => "12",
    	0b0001_0011 => "13",
    	0b0001_0100 => "14",
    	0b0001_0101 => "15",
    	0b0001_0110 => "16",
    	0b0001_0111 => "17",
    	0b0001_1000 => "18",
    	0b0001_1001 => "19",
    	0b0001_1010 => "1A",
    	0b0001_1011 => "1B",
    	0b0001_1100 => "1C",
    	0b0001_1101 => "1D",
    	0b0001_1110 => "1E",
    	0b0001_1111 => "1F",
    	0b0010_0000 => "20",
    	0b0010_0001 => "21",
    	0b0010_0010 => "22",
    	0b0010_0011 => "23",
    	0b0010_0100 => "24",
    	0b0010_0101 => "25",
    	0b0010_0110 => "26",
    	0b0010_0111 => "27",
    	0b0010_1000 => "28",
    	0b0010_1001 => "29",
    	0b0010_1010 => "2A",
    	0b0010_1011 => "2B",
    	0b0010_1100 => "2C",
    	0b0010_1101 => "2D",
    	0b0010_1110 => "2E",
    	0b0010_1111 => "2F",
    	0b0011_0000 => "30",
    	0b0011_0001 => "31",
    	0b0011_0010 => "32",
    	0b0011_0011 => "33",
    	0b0011_0100 => "34",
    	0b0011_0101 => "35",
    	0b0011_0110 => "36",
    	0b0011_0111 => "37",
    	0b0011_1000 => "38",
    	0b0011_1001 => "39",
    	0b0011_1010 => "3A",
    	0b0011_1011 => "3B",
    	0b0011_1100 => "3C",
    	0b0011_1101 => "3D",
    	0b0011_1110 => "3E",
    	0b0011_1111 => "3F",
    	0b0100_0000 => "40",
    	0b0100_0001 => "41",
    	0b0100_0010 => "42",
    	0b0100_0011 => "43",
    	0b0100_0100 => "44",
    	0b0100_0101 => "45",
    	0b0100_0110 => "46",
    	0b0100_0111 => "47",
    	0b0100_1000 => "48",
    	0b0100_1001 => "49",
    	0b0100_1010 => "4A",
    	0b0100_1011 => "4B",
    	0b0100_1100 => "4C",
    	0b0100_1101 => "4D",
    	0b0100_1110 => "4E",
    	0b0100_1111 => "4F",
    	0b0101_0000 => "50",
    	0b0101_0001 => "51",
    	0b0101_0010 => "52",
    	0b0101_0011 => "53",
    	0b0101_0100 => "54",
    	0b0101_0101 => "55",
    	0b0101_0110 => "56",
    	0b0101_0111 => "57",
    	0b0101_1000 => "58",
    	0b0101_1001 => "59",
    	0b0101_1010 => "5A",
    	0b0101_1011 => "5B",
    	0b0101_1100 => "5C",
    	0b0101_1101 => "5D",
    	0b0101_1110 => "5E",
    	0b0101_1111 => "5F",
    	0b0110_0000 => "60",
    	0b0110_0001 => "61",
    	0b0110_0010 => "62",
    	0b0110_0011 => "63",
    	0b0110_0100 => "64",
    	0b0110_0101 => "65",
    	0b0110_0110 => "66",
    	0b0110_0111 => "67",
    	0b0110_1000 => "68",
    	0b0110_1001 => "69",
    	0b0110_1010 => "6A",
    	0b0110_1011 => "6B",
    	0b0110_1100 => "6C",
    	0b0110_1101 => "6D",
    	0b0110_1110 => "6E",
    	0b0110_1111 => "6F",
    	0b0111_0000 => "70",
    	0b0111_0001 => "71",
    	0b0111_0010 => "72",
    	0b0111_0011 => "73",
    	0b0111_0100 => "74",
    	0b0111_0101 => "75",
    	0b0111_0110 => "76",
    	0b0111_0111 => "77",
    	0b0111_1000 => "78",
    	0b0111_1001 => "79",
    	0b0111_1010 => "7A",
    	0b0111_1011 => "7B",
    	0b0111_1100 => "7C",
    	0b0111_1101 => "7D",
    	0b0111_1110 => "7E",
    	0b0111_1111 => "7F",
        _ => panic!("Error in encoding hexadecimal values"),
    }
}

pub fn u8_to_hex(c: char) -> u8 {
    println!("{:b}", c as u8);
    println!("{:X}", c as u8);
    match c as u16 {
        0b0001 => 0x1,
        0b0010 => 0x2,
        0b0011 => 0x3,
        0b0100 => 0x4,
        0b0101 => 0x5,
        0b0110 => 0x6,
        0b0111 => 0x7,
        0b1000 => 0x8,
        0b1001 => 0x9,
        0b1010 => 0xA,
        0b1011 => 0xB,
        0b1100 => 0xC,
        0b1101 => 0xD,
        0b1110 => 0xE,
        0b1111 => 0xF,
    _ => panic!("Error translating u8 to hex"),
    }
}

pub fn from_ascii_to_hex(c: char) -> u8 {
    match c {
        '1' => 0b0001,
        '2' => 0b0010,
        '3' => 0b0011,
        '4' => 0b0100,
        '5' => 0b0101,
        '6' => 0b0110,
        '7' => 0b0111,
        '8' => 0b1000,
        '9' => 0b1001,
        'A' | 'a' => 0b1010,
        'B' | 'b' => 0b1011,
        'C' | 'c' => 0b1100,
        'D' | 'd' => 0b1101,
        'E' | 'e' => 0b1110,
        'F' | 'f' => 0b1111,
        _ => 0b0000,
    }
}

pub fn to_hex(v: &u32) -> char {
    match v {
        0 => '0',
        1 => '1',
        2 => '2',
        3 => '3',
        4 => '4',
        5 => '5',
        6 => '6',
        7 => '7',
        8 => '8',
        9 => '9',
        10 => 'a',
        11 => 'b',
        12 => 'c',
        13 => 'd',
        14 => 'e',
        15 => 'f',
        _ => panic!("unknown value in to_hex conversion"),
    }
}

pub fn to_base64(b: u32) -> char {
    match b {
        0b000000 => 'A', 	
        0b000001 => 'B', 	
        0b000010 => 'C', 	
        0b000011 => 'D', 	
        0b000100 => 'E', 	
        0b000101 => 'F', 	
        0b000110 => 'G', 	
        0b000111 => 'H', 	
        0b001000 => 'I', 	
        0b001001 => 'J', 	
        0b001010 => 'K', 	
        0b001011 => 'L', 	
        0b001100 => 'M', 	
        0b001101 => 'N', 	
        0b001110 => 'O', 	
        0b001111 => 'P',
        0b010000 => 'Q', 	
        0b010001 => 'R', 	
        0b010010 => 'S', 	
        0b010011 => 'T', 	
        0b010100 => 'U', 	
        0b010101 => 'V', 	
        0b010110 => 'W', 	
        0b010111 => 'X', 	
        0b011000 => 'Y', 	
        0b011001 => 'Z', 	
        0b011010 => 'a', 	
        0b011011 => 'b', 	
        0b011100 => 'c', 	
        0b011101 => 'd', 	
        0b011110 => 'e', 	
        0b011111 => 'f',
        0b100000 => 'g', 	
        0b100001 => 'h', 	
        0b100010 => 'i', 	
        0b100011 => 'j', 	
        0b100100 => 'k', 	
        0b100101 => 'l', 	
        0b100110 => 'm', 	
        0b100111 => 'n', 	
        0b101000 => 'o', 	
        0b101001 => 'p', 	
        0b101010 => 'q', 	
        0b101011 => 'r', 	
        0b101100 => 's', 	
        0b101101 => 't', 	
        0b101110 => 'u', 	
        0b101111 => 'v',
        0b110000 => 'w',
        0b110001 => 'x',
        0b110010 => 'y',
        0b110011 => 'z',
        0b110100 => '0',
        0b110101 => '1',
        0b110110 => '2',
        0b110111 => '3',
        0b111000 => '4',
        0b111001 => '5',
        0b111010 => '6',
        0b111011 => '7',
        0b111100 => '8',
        0b111101 => '9',
        0b111110 => '+',
        0b111111 => '/',
        _ => ' ',
    }
}

pub fn from_base64(c: char) -> u8 {
    // dbg!(c);
    match c {
       'A'  => 0b000000, 	
       'B'  => 0b000001, 	
       'C'  => 0b000010, 	
       'D'  => 0b000011, 	
       'E'  => 0b000100, 	
       'F'  => 0b000101, 	
       'G'  => 0b000110, 	
       'H'  => 0b000111, 	
       'I'  => 0b001000, 	
       'J'  => 0b001001, 	
       'K'  => 0b001010, 	
       'L'  => 0b001011, 	
       'M'  => 0b001100, 	
       'N'  => 0b001101, 	
       'O'  => 0b001110, 	
       'P'  => 0b001111,
       'Q'  => 0b010000, 	
       'R'  => 0b010001, 	
       'S'  => 0b010010, 	
       'T'  => 0b010011, 	
       'U'  => 0b010100, 	
       'V'  => 0b010101, 	
       'W'  => 0b010110, 	
       'X'  => 0b010111, 	
       'Y'  => 0b011000, 	
       'Z'  => 0b011001, 	
       'a'  => 0b011010, 	
       'b'  => 0b011011, 	
       'c'  => 0b011100, 	
       'd'  => 0b011101, 	
       'e'  => 0b011110, 	
       'f'  => 0b011111,
       'g'  => 0b100000, 	
       'h'  => 0b100001, 	
       'i'  => 0b100010, 	
       'j'  => 0b100011, 	
       'k'  => 0b100100, 	
       'l'  => 0b100101, 	
       'm'  => 0b100110, 	
       'n'  => 0b100111, 	
       'o'  => 0b101000, 	
       'p'  => 0b101001, 	
       'q'  => 0b101010, 	
       'r'  => 0b101011, 	
       's'  => 0b101100, 	
       't'  => 0b101101, 	
       'u'  => 0b101110, 	
       'v'  => 0b101111,
       'w'  => 0b110000,
       'x'  => 0b110001,
       'y'  => 0b110010,
       'z'  => 0b110011,
       '0'  => 0b110100,
       '1'  => 0b110101,
       '2'  => 0b110110,
       '3'  => 0b110111,
       '4'  => 0b111000,
       '5'  => 0b111001,
       '6'  => 0b111010,
       '7'  => 0b111011,
       '8'  => 0b111100,
       '9'  => 0b111101,
       '+'  => 0b111110,
       '/'  => 0b111111,
       '='  => 0b000000,
        _ => panic!("Error in decoding from_base64"),
    }
}