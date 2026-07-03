use crate::net::client::Client;

impl Client {
    pub fn crypt(&self, data: &mut [u8]) {
        let Some(sign) = &self.sign_buffer else {
            return;
        };

        let len = sign.len();

        for (i, byte) in data.iter_mut().enumerate() {
            *byte ^= sign[i % len];
        }
    }
}