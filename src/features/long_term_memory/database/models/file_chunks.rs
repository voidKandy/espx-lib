#[derive(sqlx::FromRow, Clone)]
pub struct FileChunkModelSql {
    pub id: String,
    pub parent_file_id: String,
    pub parent_filepath: String,
    pub idx: i16,
    pub content: String,
    pub content_embedding: super::super::vector_embeddings::EmbeddingVector,
}

#[derive(Clone)]
pub struct CreateFileChunkBody {
    pub parent_file_id: String,
    pub parent_filepath: String,
    pub idx: i16,
    pub content: String,
    pub content_embedding: super::super::vector_embeddings::EmbeddingVector,
}

pub struct GetFileChunkParams {
    pub parent_file_id: String,
}

pub struct DeleteFileChunkParams {
    pub parent_file_id: String,
    pub idx: i16,
}
