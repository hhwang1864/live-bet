use pgvector::Vector;
use serde::Serialize;
use sqlx::PgPool;

pub const EMBEDDING_DIM: usize = 128;

#[derive(Debug, Serialize, sqlx::FromRow)]
pub struct SearchResult {
    pub id: i32,
    pub name: Option<String>,
    pub image_url: Option<String>,
    pub region: Option<String>,
    pub bedroom_no: Option<i32>,
    pub price: Option<i32>,
    pub distance: f64,
}

/// Deterministic hash function used to map words to vector positions.
pub fn simple_hash(s: &str, seed: u32) -> usize {
    let mut hash: u32 = seed.wrapping_mul(2654435761);
    for byte in s.bytes() {
        hash = hash.wrapping_mul(31).wrapping_add(byte as u32);
    }
    hash as usize
}

/// Convert text into a fixed-dimension embedding vector.
///
/// Uses a deterministic hash-based approach (similar to feature hashing)
/// to map words into positions in a 128-dimensional vector. Each word
/// activates 3 positions with decreasing weights. The result is L2-normalized.
///
/// In production, swap this for neural embeddings (e.g. OpenAI, Cohere).
pub fn text_to_embedding(text: &str) -> Vector {
    let mut vec = vec![0.0f32; EMBEDDING_DIM];
    let text = text.to_lowercase();
    let words: Vec<&str> = text.split_whitespace().filter(|w| w.len() > 1).collect();

    for word in &words {
        let h1 = simple_hash(word, 0) % EMBEDDING_DIM;
        let h2 = simple_hash(word, 1) % EMBEDDING_DIM;
        let h3 = simple_hash(word, 2) % EMBEDDING_DIM;
        vec[h1] += 1.0;
        vec[h2] += 0.5;
        vec[h3] += 0.25;
    }

    // L2 normalize
    let mag: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
    if mag > 0.0 {
        for v in vec.iter_mut() {
            *v /= mag;
        }
    }

    Vector::from(vec)
}

/// Build a rich text description from property fields for embedding.
pub fn property_to_text(name: &str, region: &str, bedroom_no: i32, price: i32) -> String {
    let price_desc = match price {
        0..=300_000 => "affordable budget entry-level",
        300_001..=700_000 => "moderate mid-range",
        700_001..=1_500_000 => "premium upscale",
        _ => "luxury ultra-premium estate mansion",
    };
    let bed_desc = match bedroom_no {
        1 => "one bedroom studio compact single",
        2 => "two bedroom medium couple",
        3 => "three bedroom family spacious",
        _ => "large multi-bedroom estate mansion family",
    };
    format!("{} {} {} {}", name, region, bed_desc, price_desc)
}

/// Search properties by semantic similarity using pgvector cosine distance.
pub async fn search_properties(db: &PgPool, query: &str, limit: i64) -> Vec<SearchResult> {
    let embedding = text_to_embedding(query);
    sqlx::query_as::<_, SearchResult>(
        "SELECT id, name, image_url, region, bedroom_no, price,
                (embedding <=> $1::vector) AS distance
         FROM properties
         WHERE embedding IS NOT NULL
         ORDER BY embedding <=> $1::vector
         LIMIT $2",
    )
    .bind(embedding)
    .bind(limit)
    .fetch_all(db)
    .await
    .unwrap_or_default()
}

/// Store/update the embedding for a property.
pub async fn update_embedding(
    db: &PgPool,
    id: i32,
    name: &str,
    region: &str,
    bedroom_no: i32,
    price: i32,
) {
    let text = property_to_text(name, region, bedroom_no, price);
    let embedding = text_to_embedding(&text);
    sqlx::query("UPDATE properties SET embedding = $2 WHERE id = $1")
        .bind(id)
        .bind(embedding)
        .execute(db)
        .await
        .ok();
}

/// Compute cosine similarity between two vectors (1.0 = identical).
pub fn cosine_similarity(a: &[f32], b: &[f32]) -> f32 {
    a.iter().zip(b.iter()).map(|(x, y)| x * y).sum()
}

// ---------------------------------------------------------------------------
// Unit tests
// ---------------------------------------------------------------------------
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_embedding_dimension() {
        let emb = text_to_embedding("hello world");
        assert_eq!(emb.to_vec().len(), EMBEDDING_DIM);
    }

    #[test]
    fn test_embedding_normalized() {
        let emb = text_to_embedding("luxury house in sydney");
        let vec = emb.to_vec();
        let mag: f32 = vec.iter().map(|x| x * x).sum::<f32>().sqrt();
        assert!(
            (mag - 1.0).abs() < 0.001,
            "expected magnitude ~1.0, got {}",
            mag
        );
    }

    #[test]
    fn test_empty_text_is_zero_vector() {
        let emb = text_to_embedding("");
        assert!(emb.to_vec().iter().all(|&x| x == 0.0));
    }

    #[test]
    fn test_single_char_words_ignored() {
        let emb = text_to_embedding("a b c");
        assert!(
            emb.to_vec().iter().all(|&x| x == 0.0),
            "single-char words should be filtered out"
        );
    }

    #[test]
    fn test_deterministic() {
        let a = text_to_embedding("beach house sydney");
        let b = text_to_embedding("beach house sydney");
        assert_eq!(a.to_vec(), b.to_vec());
    }

    #[test]
    fn test_case_insensitive() {
        let a = text_to_embedding("Sydney House");
        let b = text_to_embedding("sydney house");
        assert_eq!(a.to_vec(), b.to_vec());
    }

    #[test]
    fn test_similar_queries_closer() {
        let house_beach = text_to_embedding("beach house coastal ocean");
        let house_city = text_to_embedding("city apartment urban downtown");
        let beach_query = text_to_embedding("beach house");

        let sim_beach = cosine_similarity(&house_beach.to_vec(), &beach_query.to_vec());
        let sim_city = cosine_similarity(&house_city.to_vec(), &beach_query.to_vec());
        assert!(
            sim_beach > sim_city,
            "beach house ({}) should be more similar than city apartment ({}) to 'beach house'",
            sim_beach,
            sim_city
        );
    }

    #[test]
    fn test_property_to_text_contains_fields() {
        let text = property_to_text("Beach Villa", "Sydney", 3, 1_200_000);
        assert!(text.contains("Beach Villa"));
        assert!(text.contains("Sydney"));
        assert!(text.contains("family"));
        assert!(text.contains("premium"));
    }

    #[test]
    fn test_property_to_text_budget_range() {
        let text = property_to_text("Flat", "Perth", 1, 250_000);
        assert!(text.contains("affordable"));
        assert!(text.contains("studio"));
    }

    #[test]
    fn test_property_to_text_luxury_range() {
        let text = property_to_text("Mansion", "Toorak", 10, 5_000_000);
        assert!(text.contains("luxury"));
        assert!(text.contains("multi-bedroom"));
    }

    #[test]
    fn test_simple_hash_deterministic() {
        assert_eq!(simple_hash("hello", 0), simple_hash("hello", 0));
    }

    #[test]
    fn test_simple_hash_seed_varies() {
        assert_ne!(simple_hash("hello", 0), simple_hash("hello", 1));
    }

    #[test]
    fn test_cosine_similarity_identical() {
        let v = vec![0.5, 0.5, 0.5, 0.5];
        let sim = cosine_similarity(&v, &v);
        assert!((sim - 1.0).abs() < 0.001);
    }

    #[test]
    fn test_cosine_similarity_orthogonal() {
        let a = vec![1.0, 0.0, 0.0];
        let b = vec![0.0, 1.0, 0.0];
        let sim = cosine_similarity(&a, &b);
        assert!((sim - 0.0).abs() < 0.001);
    }
}
