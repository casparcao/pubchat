/*
  # Initial Schema for Brand Rating System
  
  1. Tables
    - brands
    - product_models 
    - user_ratings
    - raw_data
    - analyzed_data
    - ratings_history
    - rating_weights
*/

-- Brands Table
CREATE TABLE IF NOT EXISTS brands (
    id BIGINT PRIMARY KEY,
    name VARCHAR(255) NOT NULL,
    category VARCHAR(50) NOT NULL,
    rating DECIMAL(3,2) NOT NULL,
    review_count INT NOT NULL,
    platform_scores JSON NOT NULL,
    social_scores JSON NOT NULL,
    is_top_brand BOOLEAN NOT NULL DEFAULT FALSE,
    advantages JSON NOT NULL,
    market_share DECIMAL(5,2),
    price_range JSON,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Product Models Table
CREATE TABLE IF NOT EXISTS product_models (
    id BIGINT PRIMARY KEY,
    brand_id BIGINT NOT NULL,
    name VARCHAR(255) NOT NULL,
    release_date DATE NOT NULL,
    price DECIMAL(10,2) NOT NULL,
    rating DECIMAL(3,2) NOT NULL,
    review_count INT NOT NULL,
    platform_scores JSON NOT NULL,
    social_scores JSON NOT NULL,
    specifications JSON NOT NULL,
    advantages JSON NOT NULL,
    disadvantages JSON NOT NULL,
    is_top_seller BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    FOREIGN KEY (brand_id) REFERENCES brands(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- User Ratings Table
CREATE TABLE IF NOT EXISTS user_ratings (
    id BIGINT PRIMARY KEY,
    user_id BIGINT NOT NULL,
    model_id BIGINT NOT NULL,
    rating TINYINT NOT NULL CHECK (rating >= 1 AND rating <= 5),
    review_text TEXT,
    purchase_verified BOOLEAN NOT NULL DEFAULT FALSE,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY unique_user_model (user_id, model_id),
    FOREIGN KEY (model_id) REFERENCES product_models(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Raw Data Table
CREATE TABLE IF NOT EXISTS raw_data (
    id BIGINT PRIMARY KEY,
    keyword VARCHAR(255) NOT NULL,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    data JSON NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Analyzed Data Table
CREATE TABLE IF NOT EXISTS analyzed_data (
    id BIGINT PRIMARY KEY,
    raw_data_id BIGINT NOT NULL,
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    analysis JSON NOT NULL,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (raw_data_id) REFERENCES raw_data(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Ratings History Table
CREATE TABLE IF NOT EXISTS ratings_history (
    id BIGINT PRIMARY KEY,
    model_id BIGINT NOT NULL,
    platform VARCHAR(50) NOT NULL,
    rating DECIMAL(3,2) NOT NULL CHECK (rating >= 0 AND rating <= 5),
    timestamp DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    created_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP,
    FOREIGN KEY (model_id) REFERENCES product_models(id)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Rating Weights Table
CREATE TABLE IF NOT EXISTS rating_weights (
    id BIGINT PRIMARY KEY,
    platform VARCHAR(50) NOT NULL,
    weight DECIMAL(3,2) NOT NULL CHECK (weight >= 0 AND weight <= 1),
    updated_at DATETIME NOT NULL DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    UNIQUE KEY unique_platform (platform)
) ENGINE=InnoDB DEFAULT CHARSET=utf8mb4 COLLATE=utf8mb4_unicode_ci;

-- Create indexes
CREATE INDEX idx_brands_category ON brands(category);
CREATE INDEX idx_brands_rating ON brands(rating);
CREATE INDEX idx_product_models_brand_id ON product_models(brand_id);
CREATE INDEX idx_raw_data_keyword ON raw_data(keyword);
CREATE INDEX idx_analyzed_data_raw_data_id ON analyzed_data(raw_data_id);
CREATE INDEX idx_ratings_history_model_id ON ratings_history(model_id);
CREATE INDEX idx_ratings_history_platform ON ratings_history(platform);

-- Insert default rating weights
INSERT INTO rating_weights (id, platform, weight) VALUES
    (UUID_SHORT(), 'user_verified', 0.3),
    (UUID_SHORT(), 'user_unverified', 0.1),
    (UUID_SHORT(), 'jd', 0.2),
    (UUID_SHORT(), 'tmall', 0.2),
    (UUID_SHORT(), 'taobao', 0.2)
ON DUPLICATE KEY UPDATE weight = VALUES(weight);