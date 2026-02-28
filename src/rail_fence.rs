// Rail Fence Cipher implementation in Rust
// Zig Zag
use crate::{Cipher, CipherError};

#[derive(Debug)]
pub struct RailFence {
	rails: usize,
}

impl RailFence {
	pub fn new(rails: usize) -> Result<Self, CipherError> {
		if rails < 2 {
			return Err(CipherError::InvalidKey(
				"Rail Fence rails must be >= 2".to_string(),
			));
		}

		Ok(Self { rails })
	}
}

fn build_pattern(text_len: usize, rails: usize) -> Vec<usize> {
	let mut pattern = Vec::with_capacity(text_len);
	let mut rail = 0usize;
	let mut direction_down = true;

	for _ in 0..text_len {
		pattern.push(rail);

		if rail == 0 {
			direction_down = true;
		} else if rail == rails - 1 {
			direction_down = false;
		}

		rail = if direction_down { rail + 1 } else { rail - 1 };
	}

	pattern
}

pub fn encrypt(text: &str, rails: usize) -> String {
	if text.chars().count() <= 1 {
		return text.to_string();
	}

	let mut rows = vec![String::new(); rails];
	let pattern = build_pattern(text.chars().count(), rails);

	for (ch, rail) in text.chars().zip(pattern.iter().copied()) {
		rows[rail].push(ch);
	}

	rows.concat()
}

pub fn decrypt(text: &str, rails: usize) -> String {
	let chars: Vec<char> = text.chars().collect();
	let text_len = chars.len();

	if text_len <= 1 {
		return text.to_string();
	}

	let pattern = build_pattern(text_len, rails);

	let mut rail_counts = vec![0usize; rails];
	for rail in &pattern {
		rail_counts[*rail] += 1;
	}

	let mut rails_chars: Vec<Vec<char>> = Vec::with_capacity(rails);
	let mut cursor = 0usize;
	for count in rail_counts {
		let segment = chars[cursor..cursor + count].to_vec();
		rails_chars.push(segment);
		cursor += count;
	}

	let mut rail_positions = vec![0usize; rails];
	let mut plain = String::with_capacity(text_len);

	for rail in pattern {
		let pos = rail_positions[rail];
		plain.push(rails_chars[rail][pos]);
		rail_positions[rail] += 1;
	}

	plain
}

impl Cipher for RailFence {
	fn encrypt(&self, text: &str) -> Result<String, CipherError> {
		Ok(encrypt(text, self.rails))
	}

	fn decrypt(&self, text: &str) -> Result<String, CipherError> {
		Ok(decrypt(text, self.rails))
	}
}

#[cfg(test)]
mod tests {
	use super::*;

	#[test]
	fn test_rail_fence_encrypt_three_rails() {
		let input = "WEAREDISCOVEREDFLEEATONCE";
		let cipher = RailFence::new(3).unwrap();
		let encrypted = cipher.encrypt(input).unwrap();
		assert_eq!(encrypted, "WECRLTEERDSOEEFEAOCAIVDEN");
	}

	#[test]
	fn test_rail_fence_decrypt_three_rails() {
		let input = "WECRLTEERDSOEEFEAOCAIVDEN";
		let cipher = RailFence::new(3).unwrap();
		let decrypted = cipher.decrypt(input).unwrap();
		assert_eq!(decrypted, "WEAREDISCOVEREDFLEEATONCE");
	}

	#[test]
	fn test_rail_fence_roundtrip_unicode() {
		let input = "Hello 世界 Rust 🦀";
		let cipher = RailFence::new(4).unwrap();
		let encrypted = cipher.encrypt(input).unwrap();
		let decrypted = cipher.decrypt(&encrypted).unwrap();
		assert_eq!(decrypted, input);
	}

	#[test]
	fn test_rail_fence_two_rails() {
		assert_eq!(encrypt("HELLO", 2), "HLOEL");
		assert_eq!(decrypt("HLOEL", 2), "HELLO");
	}

	#[test]
	fn test_rail_fence_invalid_rails() {
		let result = RailFence::new(1);
		assert!(result.is_err());
		assert_eq!(
			result.unwrap_err(),
			CipherError::InvalidKey("Rail Fence rails must be >= 2".to_string())
		);
	}
}
