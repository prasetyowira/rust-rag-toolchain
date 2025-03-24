

#[cfg(all(test, feature = "pg_vector"))]
mod tests {
    use super::*;
    use crate::common::OpenAIEmbeddingModel::TextEmbeddingAda002;

    #[tokio::test]
    async fn test_throws_correct_errors() {
        let result = PostgresVectorStore::try_new("test", TextEmbeddingAda002)
            .await
            .unwrap_err();
        assert!(matches!(result, PostgresVectorStoreError::EnvVarError(_)));

        std::env::set_var("POSTGRES_USER", "postgres");
        let result = PostgresVectorStore::try_new("test", TextEmbeddingAda002)
            .await
            .unwrap_err();
        assert!(matches!(result, PostgresVectorStoreError::EnvVarError(_)));

        std::env::set_var("POSTGRES_PASSWORD", "postgres");
        let result = PostgresVectorStore::try_new("test", TextEmbeddingAda002)
            .await
            .unwrap_err();
        assert!(matches!(result, PostgresVectorStoreError::EnvVarError(_)));

        std::env::set_var("POSTGRES_HOST", "localhost");
        let result = PostgresVectorStore::try_new("test", TextEmbeddingAda002)
            .await
            .unwrap_err();
        assert!(matches!(result, PostgresVectorStoreError::EnvVarError(_)));

        std::env::set_var("POSTGRES_DATABASE", "postgres");
        let result = PostgresVectorStore::try_new("test", TextEmbeddingAda002)
            .await
            .unwrap_err();
        assert!(matches!(
            result,
            PostgresVectorStoreError::ConnectionError(_)
        ));
    }
}
