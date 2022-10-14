use phf::{phf_map, Map};
use unicode_segmentation::UnicodeSegmentation;

static MAPPING: Map<&'static str, (Option<&'static str>, &'static str)> = phf_map! {
    "0" => (None, "null"),
    "1" => (None, "eins"),
    "2" => (None, "zwei"),
    "3" => (None, "drei"),
    "4" => (None, "vier"),
    "5" => (None, "fünf"),
    "6" => (None, "sechs"),
    "7" => (None, "sieben"),
    "8" => (None, "acht"),
    "9" => (None, "neun"),
    "a" => (Some("a"), "Anton"),
    "ä" => (Some("ae"), "Ärger"),
    "b" => (Some("bê"), "Berta"),
    "c" => (Some("tsê"), "Cäsar"),
    "ch" => (None, "Charlotte"),
    "d" => (Some("dê"), "Dora"),
    "e" => (Some("e"), "Emil"),
    "f" => (Some("eff"), "Friedrich"),
    "g" => (Some("guê"), "Gustav"),
    "h" => (Some("rá"), "Heinrich"),
    "i" => (Some("i"), "Ida"),
    "j" => (Some("iot"), "Julius"),
    "k" => (Some("ká"), "Kaufmann"),
    "l" => (Some("el"), "Ludwig"),
    "m" => (Some("em"), "Martha"),
    "n" => (Some("en"), "Nordpol"),
    "o" => (Some("o"), "Otto"),
    "ö" => (Some("oe"), "Ökonom"),
    "p" => (Some("pê"), "Paula"),
    "q" => (Some("ku"), "Quelle"),
    "r" => (Some("arr"), "Richard"),
    "s" => (Some("és"), "Samuel"),
    "sch" => (None, "Schule"),
    "ß" => (None, "Eszett"),
    "t" => (Some("tê"), "Theodor"),
    "u" => (Some("u"), "Ulrich"),
    "ü" => (Some("ue"), "Übermut"),
    "v" => (Some("fau"), "Viktor"),
    "w" => (Some("vê"), "Wilhelm"),
    "x" => (Some("ics"), "Xanthippe"),
    "y" => (Some("úpsilon"), "Ypsilon"),
    "z" => (Some("tsét"), "Zacharias"),
};

pub(crate) fn spell_it<S>(phrase: S) -> String
where
        S: AsRef<str>,
{
    let phrase = phrase.as_ref().to_lowercase();
    let mut output = format!("Spelling of {phrase}\n\n");
    let mut iter = phrase.graphemes(true);
    'outer: loop {
        if let (0, _) = iter.size_hint() {
            break;
        }
        for size in [3, 2, 1] {
            if let Some((letter, w)) = iter.as_str().get(0..size).and_then(|letter| {
                MAPPING
                    .get(letter)
                    .and_then(|(_, word)| Some((letter, word)))
            }) {
                output.push_str(letter);
                output.push_str(" - ");
                output.push_str(w);
                output.push('\n');
                for _ in 0..letter.chars().count() {
                    iter.next();
                }
                continue 'outer;
            }
            if size == 1 {
                iter.next();
            }
        }
    }
    output
}

#[cfg(test)]
mod tests {
    use super::spell_it;

    macro_rules! case {
        ($test_name:ident) => {
            #[allow(non_snake_case)]
            #[test]
            fn $test_name() {
                let name = stringify!($test_name);
                test_case_impl(name);
            }
        };
        ($test_name:ident, $($test_names:ident),+) => {
            case!($test_name);
            case!($($test_names),+);
        }
    }

    case!(hello, schnell, Charlotte, Straße);

    fn test_case_impl(name: &str) {
        let filename = &format!("fixtures/{name}.txt");
        let expected =
            std::fs::read_to_string(filename).expect(&format!("failed to read file {filename}"));
        assert_eq!(expected, spell_it(name));
    }
}
