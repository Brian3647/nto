use structopt::StructOpt;

/// Extra 2 characters needed ('-' and '_')
pub const EXTRA_CHARS: &[char; 2] = &['-', '_'];

/// All characters used. Contains 0-9, A-Z, a-z, '_' and '-'
pub const CHARS: &[char; 64] = &[
    '0',
    '1',
    '2',
    '3',
    '4',
    '5',
    '6',
    '7',
    '8',
    '9',
    'A',
    'B',
    'C',
    'D',
    'E',
    'F',
    'G',
    'H',
    'I',
    'J',
    'K',
    'L',
    'M',
    'N',
    'O',
    'P',
    'Q',
    'R',
    'S',
    'T',
    'U',
    'V',
    'W',
    'X',
    'Y',
    'Z',
    'a',
    'b',
    'c',
    'd',
    'e',
    'f',
    'g',
    'h',
    'i',
    'j',
    'k',
    'l',
    'm',
    'n',
    'o',
    'p',
    'q',
    'r',
    's',
    't',
    'u',
    'v',
    'w',
    'x',
    'y',
    'z',
    EXTRA_CHARS[0],
    EXTRA_CHARS[1],
];

pub fn numbertob64(num: usize) -> String {
    let mut num = num;
    let mut b64 = String::new();

    if num == 0 {
        return "0".into();
    }

    while num > 0 {
        b64 += &CHARS[(num & 63)].to_string();
        num >>= 6;
    }

    b64.chars().rev().collect()
}

#[derive(StructOpt)]
enum Cli {
    Bin { n: u8 },
    Hex { n: usize },
    B64 { n: usize },
}

fn main() {
    let cli = Cli::from_args();

    println!("{}", match cli {
        Cli::Bin { n } => format!("{:b}", n),
        Cli::B64 { n } => numbertob64(n),
        Cli::Hex { n } => format!("{:X}", n)
    });
}
