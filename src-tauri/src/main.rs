#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::ser::{Serialize, Serializer, SerializeStruct};

mod country_data;

#[derive(PartialEq, Debug)]
struct IbanVO {
    iban: String,
    is_valid_country: bool,
    is_correct_length: bool,
    is_divisible_by_97: bool
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
        state.serialize_field("isDivisibleBy97", &self.is_divisible_by_97)?;
        state.end()
    }
}


fn replace_letters_with_numbers(iban: &str) -> String {
    iban.to_uppercase().chars().map(|c| {
        c.to_digit(36).unwrap().to_string()
    }).collect::<String>()
}

fn divide_by_97(iban: &str) -> bool {
    match iban.parse::<u128>() {
        Ok(n) => {
            (n % 97) == 1
        } 
        _ => false
    }
}

fn get_iban_vo_from_str(iban: &str) -> IbanVO {
    let mut iban_vo = IbanVO {
        iban: String::from(iban),
        is_valid_country: false,
        is_correct_length: false,
        is_divisible_by_97: false,
    };

    if iban.len() < 2 {
        // Sanity check so program won't crash in case the string length is less 2
        return iban_vo;
    }
     
    // Get country code from iban string
    let country_code: &str = &iban[0..2];

    // Lookup valid country codes, and look up valid length per country
    let country_codes_and_length = country_data::get_country_codes_and_length();
    
    // Has valid country code
    iban_vo.is_valid_country = country_codes_and_length.get(country_code).is_some();

    // Is iban correct length
    let country_length = country_codes_and_length.get(country_code);
    iban_vo.is_correct_length = country_length == Some(&iban.len());

    // move the first four characters to the end of the string
    let iban = iban[4..].to_string() + &iban[..4];

    // replace each letter with number
    let iban = replace_letters_with_numbers(&iban);

    // Is IBAN divisible by 97
    iban_vo.is_divisible_by_97 = divide_by_97(&iban);
    return iban_vo

}


#[tauri::command] 
fn validate_iban(iban_numbers: Vec<&str>) -> Vec<IbanVO> {

    let mut iban_results: Vec<IbanVO> = Vec::new();
    if iban_numbers.len() > 1 {
        for iban in iban_numbers.into_iter() {
            iban_results.push(get_iban_vo_from_str(&iban))
        };
    } else {
        let first_number = iban_numbers.first().unwrap();
        iban_results.push(get_iban_vo_from_str(&first_number));
    }
    
    return iban_results

}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![validate_iban])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");    
}




// Tests: Placing unit tests in the same file is the idiomatic way of doing it in rust
#[cfg(test)] // Do not include this in the executable
mod tests {   

    use super::*; // Brings in all methods in this file in the tests scope

    #[test]
    fn test_letter_replacer() {
        let iban = "DE89370400440532013000";
        let expected = "131489370400440532013000";
        assert_eq!(replace_letters_with_numbers(iban), expected);

        let iban_two = "89370400440532013DE000";
        let expected_two = "893704004405320131314000";
        assert_eq!(replace_letters_with_numbers(iban_two), expected_two);
    }

    #[test]
    fn test_valid_iban() {
        let iban = "DE89370400440532013000";
        let expected = IbanVO {
            iban: String::from(iban),
            is_valid_country: true,
            is_correct_length: true,
            is_divisible_by_97: true,
        };
        assert_eq!(get_iban_vo_from_str(iban), expected);
    }

    #[test]
    fn test_not_divisible_by_97() {
        let iban = "DE89370400440532013001";
        let expected_output = IbanVO {
            iban: String::from(iban),
            is_valid_country: true,
            is_correct_length: true,
            is_divisible_by_97: false,
        };
        assert_eq!(get_iban_vo_from_str(iban), expected_output);
    }

    #[test]
    fn test_incorrect_length() {
        let iban = "DE8937040044053201300044";
        let expected_output = IbanVO {
            iban: String::from(iban),
            is_valid_country: true,
            is_correct_length: false,
            is_divisible_by_97: false,
        };
        assert_eq!(get_iban_vo_from_str(iban), expected_output);
    }

    #[test]
    fn test_invalid_country_code() {
        let iban = "XX89370400440532013000";
        let expected_output = IbanVO {
            iban: String::from(iban),
            is_valid_country: false,
            is_correct_length: false,
            is_divisible_by_97: false,
        };
        assert_eq!(get_iban_vo_from_str(iban), expected_output);
    }
}
