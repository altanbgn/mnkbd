use phf::phf_map;

// АБВГДЕЁЖЗИЙКЛМНОӨПРСТУҮФХЦЧШЩЫЬЭЮЯ
// абвгдеёжзийклмноөпрстуүфхцчшщыьэюя
pub static TRANSFER_DICTIONARY: phf::Map<&'static str, &'static str> = phf_map! {
    // Lowercase
    "ch" => "ч",
    "sh" => "ш",
    "ts" => "ц",
    "yo" => "ё",
    "ye" => "е",
    "yu" => "ю",
    "ya" => "я",
    "a" => "а",
    "b" => "б",
    "c" => "",
    "d" => "д",
    "e" => "э",
    "f" => "ф",
    "g" => "г",
    "h" => "х",
    "i" => "и",
    "i=" => "й",
    "j" => "ж",
    "k" => "к",
    "l" => "л",
    "m" => "м",
    "n" => "н",
    "o" => "о",
    "o=" => "ө",
    "p" => "п",
    "q" => "",
    "r" => "р",
    "s" => "с",
    "t" => "т",
    "u" => "у",
    "u=" => "ү",
    "v" => "в",
    "w" => "",
    "x" => "",
    "y" => "ы",
    "z" => "з",

    // Uppercase
    "Ch" => "Ч",
    "Sh" => "Ш",
    "Ts" => "Ц",
    "Yo" => "Ё",
    "Ye" => "Е",
    "Yu" => "Ю",
    "Ya" => "Я",
    "A" => "А",
    "B" => "Б",
    "C" => "",
    "D" => "Д",
    "E" => "Э",
    "F" => "Ф",
    "G" => "Г",
    "H" => "Х",
    "I" => "И",
    "I=" => "Й",
    "J" => "Ж",
    "K" => "К",
    "L" => "Л",
    "M" => "М",
    "N" => "Н",
    "O" => "О",
    "O=" => "Ө",
    "P" => "П",
    "Q" => "",
    "R" => "Р",
    "S" => "С",
    "T" => "Т",
    "U" => "У",
    "U=" => "Ү",
    "V" => "В",
    "W" => "",
    "X" => "",
    "Y" => "Ы",
    "Z" => "З",

    // Symbols
    "?" => "?",
    "!" => "!",
    "." => ".",
    "," => ",",
    "(" => "(",
    ")" => ")",
};
