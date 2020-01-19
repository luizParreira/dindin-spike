use crate::models::{AuthenticationPin, User};

pub fn send_email(user: &User, pin: &AuthenticationPin) {
    println!(
        "Send email to: {:?} with pin: {}",
        user.email.as_ref().unwrap(),
        pin.pin
    );
}

pub fn send_text_message(user: &User, pin: &AuthenticationPin) {
    println!(
        "Send text message to: {} with pin: {}",
        user.phone_number.as_ref().unwrap(),
        pin.pin
    );
}
