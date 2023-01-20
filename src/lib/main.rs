#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]


use iban::*;
// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[tauri::command]
fn validate_iban(iban_input: &str) -> String {
    let account = iban_input.parse::<Iban>()?;
    Result::expect(account.country_code(), "DE");
    assert_eq!(account.country_code(), "DE");
    assert_eq!(account.check_digits(), 44);
    assert_eq!(account.bban(), "500105175407324931");
    assert_eq!(account.electronic_str(), "DE44500105175407324931");
    assert_eq!(account.to_string(), "DE44 5001 0517 5407 3249 31");
    assert_eq!(account.bank_identifier(), Some("50010517"));
    assert_eq!(account.branch_identifier(), None);
    format!("Hello, {}! You've been greeted from Rust!", "valid iban")
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![validate_iban])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
