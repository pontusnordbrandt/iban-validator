#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use serde::ser::{Serialize, Serializer, SerializeStruct};

mod country_data;

#[derive(PartialEq, Debug)]
struct IbanVO {
    iban: String,
    is_alphanumeric: bool,
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
        state.serialize_field("isAlphanumeric", &self.is_alphanumeric)?;
        state.serialize_field("isCorrectLength", &self.is_correct_length)?;
        state.serialize_field("isDivisibleBy97", &self.is_divisible_by_97)?;
        state.end()
    }
}

fn is_alphanumeric(iban: &str) -> bool {
    iban.chars().all(|c| {
        c.is_alphanumeric()
    })
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
        is_alphanumeric: false,
        is_valid_country: false,
        is_correct_length: false,
        is_divisible_by_97: false,
    };

    if iban.len() < 2 {
        // Sanity check so program won't crash in case the string length is less 2
        return iban_vo;
    }
     let is_iban_alphanumeric = is_alphanumeric(&iban);
    if !is_iban_alphanumeric {
        return iban_vo;
    } else {
        iban_vo.is_alphanumeric = true;
    }
    // Get country code from iban string
    let country_code: &str = &iban[0..2];

    // Lookup valid country codes, and look up valid length per country
    let country_codes_and_length = country_data::get_country_codes_and_length();
    
    // Has valid country code
    iban_vo.is_valid_country = country_codes_and_length.get(country_code).is_some();

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
        // Numbers taken from www.iban.com/structure
        // It did not work for the IBAN for Burundi, for some reason the IBAN length for that country is listed as 27, but the example account has 28 digits
        // Burundi	BI	27	BI43220001131012345678912345

        let iban1 = "BH02CITI00001077181611";
        let iban2 = "DJ2110002010010409943020008";
        let iban3 = "SV43ACAT00000000000000123123";
        let iban4 = "GT20AGRO00000000001234567890";
        
        let expected1 = IbanVO {
            iban: String::from(iban1),
            is_alphanumeric: true,
            is_valid_country: true,
            is_correct_length: true,
            is_divisible_by_97: true,
        };
        let expected2 = IbanVO {
            iban: String::from(iban2),
            is_alphanumeric: true,
            is_valid_country: true,
            is_correct_length: true,
            is_divisible_by_97: true,
        };
        let expected3 = IbanVO {
            iban: String::from(iban3),
            is_alphanumeric: true,
            is_valid_country: true,
            is_correct_length: true,
            is_divisible_by_97: true,
        };
        let expected4 = IbanVO {
            iban: String::from(iban4),
            is_alphanumeric: true,
            is_valid_country: true,
            is_correct_length: true,
            is_divisible_by_97: true,
        };
        assert_eq!(get_iban_vo_from_str(iban1), expected1);
        assert_eq!(get_iban_vo_from_str(iban2), expected2);
        assert_eq!(get_iban_vo_from_str(iban3), expected3);
        assert_eq!(get_iban_vo_from_str(iban4), expected4);
    }

    #[test]
    fn test_not_divisible_by_97() {
        let iban = "DE89370400440532013001";
        let expected_output = IbanVO {
            iban: String::from(iban),
            is_alphanumeric: true,
            is_valid_country: true,
            is_correct_length: true,
            is_divisible_by_97: false,
        };
        assert_eq!(get_iban_vo_from_str(iban), expected_output);
    }

    #[test]
    fn test_is_alphanumeric() {
        let num1 = "LKSDJFLKJHLKJ39872389476";
        assert_eq!(is_alphanumeric(num1), true);
        let num2 = "ASDÖLKölsdgjlknqåeptoilkbxöcbjewrjiqopwer923874013658971";
        assert_eq!(is_alphanumeric(num2), true);
        let num3 = "ABC_";
        assert_eq!(is_alphanumeric(num3), false);
        let num4 = "ABC;";
        assert_eq!(is_alphanumeric(num4), false);
    }

    #[test]
    fn test_incorrect_length() {
        let iban = "AT4832000000123234245864";
        let expected_output = IbanVO {
            iban: String::from(iban),
            is_alphanumeric: true,
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
            is_alphanumeric: true,
            is_valid_country: false,
            is_correct_length: false,
            is_divisible_by_97: false,
        };
        assert_eq!(get_iban_vo_from_str(iban), expected_output);
    }
}
