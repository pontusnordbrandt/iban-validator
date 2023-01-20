#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]



 fn has_valid_country_code(iban: &str) -> bool  {
    if iban.len() < 2 {
        return false;
    }
    let country_code: &str = &iban[0..2];

    return country_code.eq("DE");
}

fn has_valid_check_digits(iban: &str) -> bool  {
    if iban.len() < 2 {
        return false;
    }
    let country_code: &str = &iban[2..4];

    return country_code.eq("DE");
}

fn has_valid_format(iban: &str) -> bool  {
    if iban.len() < 2 {
        return false;
    }
    let country_code: &str = &iban[2..4];

    return country_code.eq("DE");
}

use serde::ser::{Serialize, Serializer, SerializeStruct};

struct IbanVO {
    iban: String,
    is_valid_country: bool,
}

impl Serialize for IbanVO {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut state = serializer.serialize_struct("IbanVO", 2)?;
        state.serialize_field("iban", &self.iban)?;
        state.serialize_field("isValidCountry", &self.is_valid_country)?;
        state.end()
    }
}

#[tauri::command] 
fn validate_iban(iban: &str) -> IbanVO {
    if !has_valid_country_code(iban) {
        return IbanVO {
            iban: String::from(iban),
            is_valid_country: false,
        };
    }

    if !has_valid_check_digits(iban) {
        return IbanVO {
            iban: String::from(iban),
            is_valid_country: false,
        };
    }

    if !has_valid_format(iban) {
        return IbanVO {
            iban: String::from(iban),
            is_valid_country: false,
        };
    }

    return IbanVO {
        iban: String::from(iban),
        is_valid_country: true,
    };
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
    fn should_return_true() {
        assert_eq!(has_valid_country_code("DE"), true);
    }
}