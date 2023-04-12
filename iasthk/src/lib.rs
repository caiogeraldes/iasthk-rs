pub mod converter {
    use unicode_normalization::UnicodeNormalization;
    fn ascii_to_unicode<T: Into<String>>(input: T) -> String {
        let mut output: String = input.into();
        output = output
            .replace('/', "\u{0301}")
            .replace('\\', "\u{0300}")
            .replace('A', "a\u{0304}")
            .replace('I', "i\u{0304}")
            .replace('U', "u\u{0304}")
            .replace("lRR", "ḷ\u{0304}")
            .replace("lR", "ḷ")
            .replace("RR", "ṛ\u{0304}")
            .replace('R', "ṛ")
            .replace('M', "ṃ")
            .replace('H', "ḥ")
            .replace('G', "ṅ")
            .replace('J', "ñ")
            .replace('T', "ṭ")
            .replace('D', "ḍ")
            .replace('N', "ṇ")
            .replace('z', "ś")
            .replace('S', "ṣ")
            .replace("||", "\u{0965}")
            .replace('|', "\u{0964}");
        output
    }
    fn normalize_unicode<T: Into<String>>(input: T) -> String {
        let input: &str = &input.into();
        input.nfkc().collect::<String>()
    }
    pub fn convert<T: Into<String>>(input: T) -> String {
        let mut output = input.into();
        output = ascii_to_unicode(output);
        output = normalize_unicode(output);

        output
    }
    #[cfg(test)]
    mod test {
        use super::*;
        #[test]
        fn test_convert() {
            let string = String::from("A");
            let result = convert(string);
            assert_eq!(result, normalize_unicode("ā".to_string()));
            let string = String::from("asti nRpo");
            let result = convert(string);
            assert_eq!(result, "asti nṛpo".to_string());
            let string = String::from("RR");
            let result = convert(string);
            assert_eq!(result, normalize_unicode("ṝ".to_string()));
        }
        #[test]
        fn unicode_normalized() {
            let input = String::from("a/sti");
            let output = normalize_unicode("ásti");
            let result = convert(input);
            assert_eq!(result, output);
        }
    }
}

/// Validation module for Harvard-Kyoto texts
pub mod validator {
    use regex::Regex;
    use std::fmt;

    #[derive(Debug)]
    pub enum ValidationError {
        NotASCII(Vec<char>),
        InvalidChars(Vec<char>),
        InvalidDiacriticOrder(Vec<String>),
    }
    impl fmt::Display for ValidationError {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            match self {
                ValidationError::NotASCII(a) => write!(f, "Non ASCII chars {:?}", a),
                ValidationError::InvalidChars(a) => write!(f, "Invalid characteres {:?}", a),
                ValidationError::InvalidDiacriticOrder(a) => {
                    write!(f, "Invalid diacritic order: {:?}", a)
                }
            }
        }
    }

    fn diacritics_ordered<T: Into<String>>(input: T) -> Result<(), ValidationError> {
        let input: String = input.into();
        let re = Regex::new(r"[bcdghjklmprstvzGHJLMS][/\\=]").unwrap();

        let matches: Vec<regex::Match> = re.find_iter(&input).collect();
        match matches.len() {
            0 => Ok(()),
            _ => {
                let v: Vec<String> = matches
                    .into_iter()
                    .map(|m| m.as_str().to_string())
                    .collect();
                Err(ValidationError::InvalidDiacriticOrder(v))
            }
        }
    }

    fn standard_characteres<T: Into<String>>(input: T) -> Result<(), ValidationError> {
        let input: String = input.into();
        let valid_chars = vec![
            'a', 'b', 'c', 'd', 'e', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'r', 's',
            't', 'u', 'v', 'z', 'A', 'G', 'H', 'I', 'J', 'L', 'M', 'N', 'R', 'S', 'U', '/', '\\',
            '\n', '-', '|', ' ', '\'',
        ];
        match input.chars().all(|c| valid_chars.contains(&c)) {
            true => Ok(()),
            false => {
                let mut invalid_chars: Vec<char> =
                    input.chars().filter(|c| !valid_chars.contains(c)).collect();
                invalid_chars.dedup();
                Err(ValidationError::InvalidChars(invalid_chars))
            }
        }
    }

    pub fn validate<T: Into<String>>(input: T) -> Result<(), ValidationError> {
        let input: String = input.into();

        check_ascii(&input)?;
        diacritics_ordered(&input)?;
        standard_characteres(input)?;
        Ok(())
    }

    fn check_ascii<T: Into<String>>(input: T) -> Result<(), ValidationError> {
        let input: String = input.into();

        if !input.is_ascii() {
            let mut non_ascii_chars: Vec<char> = input.chars().filter(|c| !c.is_ascii()).collect();
            non_ascii_chars.dedup();
            Err(ValidationError::NotASCII(non_ascii_chars))
        } else {
            Ok(())
        }
    }

    #[cfg(test)]
    mod test {
        use crate::validator;
        #[test]
        fn validation() {
            assert!(validator::validate("agnimiLepurohitaM").is_ok());
            assert!(validator::validate("ab=").is_err());
            assert!(validator::validate("af=").is_err());
        }
    }
}
