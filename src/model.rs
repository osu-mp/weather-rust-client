//
// Check out `quicktype`.
//   On GitHub: https://github.com/quicktype/quicktype
//   In action: https://app.quicktype.io
//

use serde::{Serialize, Deserialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct Weather {
    main: Main,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Main {
    temp: f64,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct Token {
    pub main: TokenMain,
}

// main structure for token response
#[derive(Debug, Serialize, Deserialize)]
pub struct TokenMain {
    pub token: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct SiteUser {
    pub username: String,
    pub password: String,
}

// main data structure for hello greeting
#[derive(Debug, Serialize, Deserialize)]
pub struct Hello {
    pub main: HelloMain,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct HelloMain {
    pub greeting: String,
}
