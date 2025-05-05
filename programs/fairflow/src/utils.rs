pub fn encrypt_decrypt_salary(key: u16, salary: u16) -> u16 {
    salary ^ key
}
