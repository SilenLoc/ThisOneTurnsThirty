

use anyhow::anyhow;
use rocket::{
    http::Status,
    request::{FromRequest, Outcome},
    Request, State,
};
use serde::{Deserialize};






#[derive(Debug, Deserialize)]
pub struct AdminAccessToken {
    pub code: String,
}


#[async_trait]
impl<'r> FromRequest<'r > for AdminAccessToken {
    type Error = anyhow::Error;
    async fn from_request(request: &'r Request<'_>) -> Outcome<Self, Self::Error> {
        let Some(code) = request
            .headers()
            .get("authorization")
            .next()
            .and_then(|v| v.strip_prefix("Bearer "))
        else {
            return Outcome::Error((
                Status::Unauthorized,
                anyhow!("missing authorization token"),
            ));
        };

        let Outcome::Success(expected) = request.guard::<&State<AdminAccessToken>>().await else {
            panic!("no AdminAccessToken");
        };

      if expected.code == code {
           return  Outcome::Success(AdminAccessToken{code: code.to_string()})
        } else {
           return  Outcome::Error((
                Status::Unauthorized,
                anyhow!("missing authorization token"),
            ));
        }
       
    }
}
