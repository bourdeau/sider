pub async fn pong() -> String {
    "PONG\n".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_pong() {
        let res = pong().await;
        assert_eq!(res, "PONG\n");
    }
}
