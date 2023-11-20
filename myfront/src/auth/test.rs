#[cfg(test)]
mod test {
    use crate::auth::Jwt;

    const SECRET: &str = "不负信赖";
    const ISS: &str = "圣农集团";
    #[test]
    fn test_gen_token() {
        let jwt = Jwt::new(SECRET.to_string(), ISS.to_string());
        let claims = jwt.new_claims("1".to_string(), "team@axum.rs".to_string(), 30);
        let token = jwt.token(&claims).unwrap();
        println!("{:?}", &token);
    }
    #[test]
    fn test_get_claims() {
        let jwt = Jwt::new(SECRET.to_string(), ISS.to_string());
        let claims = jwt.new_claims("1".to_string(), "team@axum.rs".to_string(), 30);
        let token = jwt.token(&claims).unwrap();
        let claims = jwt.verify_and_get(token.as_str()).unwrap();
        println!("{}", claims);
    }
}
