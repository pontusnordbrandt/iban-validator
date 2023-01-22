#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]



use serde::ser::{Serialize, Serializer, SerializeStruct};
use std::collections::HashMap;

struct IbanVO {
    iban: String,
    is_valid_country: bool,
    is_correct_length: bool,
    has_valid_check_digits: bool
}

impl Serialize for IbanVO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("IbanVO", 2)?;
        state.serialize_field("iban", &self.iban)?;
        state.serialize_field("isValidCountry", &self.is_valid_country)?;
        state.serialize_field("isCorrectLength", &self.is_correct_length)?;
        state.serialize_field("hasValidCheckDigits", &self.has_valid_check_digits)?;
        state.end()
    }
}

/* 
1. Check that the total IBAN length is correct as per the country. If not, the IBAN is invalid
2. Move the four initial characters to the end of the string
3. Replace each letter in the string with two digits, thereby expanding the string, where A = 10, B = 11, ..., Z = 35
4. Interpret the string as a decimal integer and compute the remainder of that number on division by 97 
*/

#[tauri::command] 
fn validate_iban(iban: &str) -> IbanVO {

    // Initialize response object
    let mut iban_vo = IbanVO {
        iban: String::from(iban),
        is_valid_country: false,
        is_correct_length: false,
        has_valid_check_digits: false,
    };

    if iban.len() < 2 {
        // Sanity check so program won't crash in case the string length is less 2
        return iban_vo;
    }
     
    // Get country code from iban string
    let country_code: &str = &iban[0..2];

    // Lookup valid country codes, and look up valid length per country
    let country_codes_and_length = get_country_codes_and_length();
    
    // Has valid country code
    iban_vo.is_valid_country = country_codes_and_length.get(country_code).is_some();

    // Is iban correct length
    let country_length = country_codes_and_length.get(country_code);
    iban_vo.is_correct_length = country_length == Some(&iban.len());

  
    // TODO See if check digits are correct
    iban_vo.has_valid_check_digits = true;

    return iban_vo;
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![validate_iban])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");    
}

fn get_country_codes_and_length() -> HashMap<&'static str, usize> {
    let country_codes = std::collections::HashMap::from([
        ("AL", 28),
        ("AD", 24),
        ("AT", 20),
        ("AZ", 28),
        ("BH", 22),
        ("BY", 28),
        ("BE", 16),
        ("BA", 20),
        ("BR", 29),
        ("BG", 22),
        ("BI", 27),
        ("CR", 22),
        ("HR", 21),
        ("CY", 28),
        ("CZ", 24),
        ("DK", 18),
        ("DJ", 27),
        ("DO", 28),
        ("EG", 29),
        ("SV", 28),
        ("EE", 20),
        ("FO", 18),
        ("FI", 18),
        ("FR", 27),
        ("GE", 22),
        ("DE", 22),
        ("GI", 23),
        ("GR", 27),
        ("GL", 18),
        ("GT", 28),
        ("VA", 22),
        ("HU", 28),
        ("IS", 26),
        ("IQ", 23),
        ("IE", 22),
        ("IL", 23),
        ("IT", 27),
        ("JO", 30),
        ("KZ", 20),
        ("XK", 20),
        ("KW", 30),
        ("LV", 21),
        ("LB", 28),
        ("LY", 25),
        ("LI", 21),
        ("LT", 20),
        ("LU", 20),
        ("MT", 31),
        ("MR", 27),
        ("MU", 30),
        ("MD", 24),
        ("MC", 27),
        ("ME", 22),
        ("NL", 18),
        ("MK", 19),
        ("NO", 15),
        ("PK", 24),
        ("PS", 29),
        ("PL", 28),
        ("PT", 25),
        ("QA", 29),
        ("RO", 24),
        ("RU", 33),
        ("LC", 32),
        ("SM", 27),
        ("ST", 25),
        ("SA", 24),
        ("RS", 22),
        ("SC", 31),
        ("SK", 24),
        ("SI", 19),
        ("ES", 24),
        ("SD", 18),
        ("SE", 24),
        ("CH", 21),
        ("TL", 23),
        ("TN", 24),
        ("TR", 26),
        ("UA", 29),
        ("AE", 23),
        ("GB", 22),
        ("VG", 24),
        ("DZ", 26),
        ("AO", 25),
        ("BJ", 28),
        ("BF", 28),
        ("CM", 27),
        ("CV", 25),
        ("CF", 27),
        ("TD", 27),
        ("KM", 27),
        ("CG", 27),
        ("GQ", 27),
        ("GA", 27),
        ("GW", 25),
        ("HN", 28),
        ("IR", 26),
        ("CI", 28),
        ("MG", 27),
        ("ML", 28),
        ("MN", 20),
        ("MA", 28),
        ("MZ", 25),
        ("NI", 32),
        ("NE", 28),
        ("SN", 28),
        ("TG", 28)
    ]);
    return country_codes;
}


// Tests: Placing unit tests in the same file is the idiomatic way of doing it in rust
#[cfg(test)] // Do not include this in the executable
mod tests {   
    // use super::*; // Brings in all methods in this file in the tests scope

    #[test]
    fn should_return_true() {
       /* 
       assert_eq!(validate_iban("BE71096123456769"), IbanVO::from([
       iban: String::new("BE71096123456769"),
       is_valid_country: true,
                is_correct_length: true,
                has_valid_check_digits: true
        ]
        ));
         */
    }
}
