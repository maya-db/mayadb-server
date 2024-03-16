use super::exceptions::DOCUMENT_MAX_SIZE;
use std::mem::size_of_val;
use super::Type;


pub struct Validation;

impl Validation {
    pub fn check_document_size(value: &Type, document_size: &mut usize, document_max_size: usize) -> Result<bool, &'static str> {
        let mut type_size = size_of_val(value);

        if let Type::Str(s) = &value {
            type_size += s.len() * 4;
        }

        *document_size += type_size;

        println!("[document_size {}] [document_max_size {}]", *document_size, document_max_size);

        if *document_size > document_max_size {
            return Err(DOCUMENT_MAX_SIZE);
        }

        Ok(true)
    }
}
