#[cfg(test)]
mod tests {
    use cnctd_go::Go;

    #[tokio::test]
    async fn test_commands() {
        Go::check_for_go().await.unwrap();
    }
}
