use base64::{Engine as _, engine::general_purpose};

#[derive(Debug)]
pub struct Authorization {
   pub client_id: String,
   pub client_secret: String,
}

impl Authorization {
    /// Returns a new Authorization instance from a decoded basic authorization header.
    ///
    /// # Arguments
    /// * `auth` - A string slice that holds the basic authorization header.
    ///
    /// # Example
    /// ```
    /// use authorization::Authorization;
    /// mod authorization;
    /// let auth_header = "Basic dGVzdGNsaWVudDp0ZXN0WHd1R1lpS1FUek9vbVJSc3JuR0psVXp2R09WVw==";
    /// let authorization = Authorization::from_basic_header(auth_header);
    /// ```
   pub fn from_basic_header(auth_header: &str) -> Result<Authorization, &'static str> {
      // Step 1: Return an error if the string "auth" does not starts with "Basic ".
      if !auth_header.starts_with("Basic ") {
          return Err("Invalid authorization header. Must start with 'Basic '");
      }

      // Step 2: Base64 decode the part of the string "auth" behind the "Basic "-token.
      let encoded_auth = &auth_header[6..];
      let decoded_bytes = match general_purpose::STANDARD.decode(&encoded_auth) {
          Ok(bytes) => bytes,
          Err(_) => return Err("Cannot decode authorization header as base64"),
      };
      let decoded_string = match String::from_utf8(decoded_bytes) {
          Ok(string) => string,
          Err(_) => return Err("Cannot decode authorization header as utf8"),
      };

      // Step 3: Split the decoded string at the ":"-character.
      //         Split only at the first occurence of ":". The username cannot contain ":", but the passowrd can.
      let parts: Vec<&str> = decoded_string.splitn(2, ':').collect();
      if parts.len() != 2 {
          return Err("Invalid authorization header");
      }
      // Step 4: Create an instance of the Struct "Authorization".
      let authorization = Authorization {
          client_id: parts[0].to_string(),
          client_secret: parts[1].to_string(),
      };
      Ok(authorization)
  }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn success() {
        let client_id = "any_client_id";
        let client_secret = "any-client-secret";
        let encoded_auth = general_purpose::STANDARD.encode(format!("{}:{}", client_id, client_secret));
        let auth_header = format!("Basic {}", encoded_auth);

        let authorization = Authorization::from_basic_header(&auth_header).unwrap();

        assert_eq!(client_id, authorization.client_id);
        assert_eq!(client_secret, authorization.client_secret);
    }
}
