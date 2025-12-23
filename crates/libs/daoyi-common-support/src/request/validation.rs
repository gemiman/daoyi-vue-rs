use regex::Regex;
use std::borrow::Cow;
use std::cell::LazyCell;
use validator::ValidationError;

const MOBILE_PHONE_REGEX: LazyCell<Regex> = LazyCell::new(|| {
    Regex::new(r"^(?:\+?86)?1[3456789]\d{9}$|^(?:\+?852)?[5-9]\d{3,7}$|^(?:\+?853)?6\d{7}$|^(?:\+?886)?9\d{8}$|^(?:\+?855)?[1-9]\d{7,9}$|^(?:\+?850)?[1-9]\d{7,9}$|^(?:\+?82)?1[0-9]{8,9}$|^(?:\+?81)?[789]0[0-9]{7}$|^(?:\+?65)?[89]\d{7}$|^(?:\+?60)?1[0-9]{8,9}$|^(?:\+?66)?[6-9]\d{7,8}$|^(?:\+?62)?8[1-9]\d{6,9}$|^(?:\+?63)?9[0-9]{9}$|^(?:\+?64)?[2-9]\d{7,9}$|^(?:\+?61)?4[0-9]{8}$|^(?:\+?33)?6[0-9]{8}$|^(?:\+?49)?1[57][0-9]{8}$|^(?:\+?34)?[67]\d{8}$|^(?:\+?39)?3[13457-9]\d{8}$|^(?:\+?44)?7[1-9]\d{8}$|^(?:\+?1)?[2-9][0-9]{2}[2-9][0-9]{2}[0-9]{4}$").expect("Failed to compile mobile phone regex")
});
pub fn is_mobile_phone(value: &str) -> Result<(), ValidationError> {
    if MOBILE_PHONE_REGEX.is_match(value) {
        Ok(())
    } else {
        Err(build_validation_error("手机号码格式不正确"))
    }
}

fn build_validation_error(error: &'static str) -> ValidationError {
    ValidationError {
        code: Cow::from("invalid"),
        message: Some(Cow::from(error)),
        params: Default::default(),
    }
}
