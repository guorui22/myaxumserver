#[cfg(test)]
mod test {
    use crate::Jwt;

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
        // let claims = jwt.verify_and_get(token.as_str()).unwrap();
        let claims = jwt.verify_and_get("eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJjb2RlIjoiZ3IwMDEiLCJuYW1lIjoi6YOt552_IiwiaXNzIjoi5Zyj5Yac6ZuG5ZuiIiwiZXhwIjoxNzE1Mzc1MjY5fQ.cZQxDMDnErO0TCKwsJVKk1eDgH04P76kpVQpWqjcWak").unwrap();
        println!("{}", claims);
    }
}
