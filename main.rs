use std::io::{self, Write};

fn cipher(text_bytes: &[u8], key: &str) -> Vec<u8> {
    text_bytes.iter()
        .zip(key.bytes().cycle())
        .map(|(b, k)| b ^ k)
        .collect()
}

fn main() {
    println!("=================================");
    println!("🔐 МЕССЕНДЖЕР ZERO: КРИПТО-ЯДРО (ФИКС)");
    println!("=================================");

    let secret_key = "SUPER_SECRET_KEY_2026_ZERO_CORE";

    print!("Введи секретное сообщение: ");
    io::stdout().flush().unwrap();
    
    let mut input_text = String::new();
    io::stdin().read_line(&mut input_text).unwrap();
    let input_text = input_text.trim();

    if input_text.is_empty() { return; }

    // 1. Шифруем оригинальный текст (переводим его в байты)
    let encrypted_bytes = cipher(input_text.as_bytes(), secret_key);
    let hex_encoded: String = encrypted_bytes.iter().map(|b| format!("{:02x}", b)).collect();

    println!("\n[ПЕРЕДАЧА] Зашифрованный пакет: 📦 HASH: {}", hex_encoded);

    // 2. Расшифровываем ЗАШИФРОВАННЫЕ байты обратно
    let decrypted_bytes = cipher(&encrypted_bytes, secret_key);
    let decrypted_text = String::from_utf8(decrypted_bytes).unwrap_or_else(|_| "Ошибка".to_string());

    println!("\n[ПРИЕМНИК] Применение ключа...");
    println!("💬 Текст в чате получателя: «{}»", decrypted_text);
    println!("=================================");
}
