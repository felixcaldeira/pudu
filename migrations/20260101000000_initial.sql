-- fotos
CREATE TABLE images (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    nanoid CHAR(12) UNIQUE NOT NULL,
    mime_type VARCHAR(100),
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- dateien table
CREATE TABLE files (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    file_name VARCHAR(255) NOT NULL,
    mime_type VARCHAR(16) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP
);

-- Users table (for admin)
CREATE TABLE users (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    image_id INT UNSIGNED,
    username VARCHAR(100) UNIQUE NOT NULL,
    academic_title VARCHAR(100),
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100) NOT NULL,
    flags INT UNSIGNED DEFAULT 0,
    email VARCHAR(255) UNIQUE NOT NULL,
    password_hash VARCHAR(255) NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    FOREIGN KEY (image_id)
        REFERENCES images(id)
        ON DELETE SET NULL
);

-- Articles table
CREATE TABLE articles (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    image_id INT UNSIGNED DEFAULT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    content LONGTEXT NOT NULL,
    published BOOLEAN DEFAULT false,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,
    
    FOREIGN KEY (image_id)
        REFERENCES images(id)
        ON DELETE SET NULL
);

-- fachgruppen
CREATE TABLE module_categories (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    image_id INT UNSIGNED DEFAULT NULL,
    slug VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    published BOOLEAN DEFAULT false,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    FOREIGN KEY (image_id)
        REFERENCES images(id)
        ON DELETE SET NULL
);

-- Unterrichtseinheiten
CREATE TABLE modules (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    category_id INT UNSIGNED DEFAULT NULL,
    image_id INT UNSIGNED DEFAULT NULL, -- cover image
    user_id INT UNSIGNED DEFAULT NULL, -- author
    slug VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    content TEXT NOT NULL, -- in markdown
    grade_flags SMALLINT UNSIGNED NOT NULL DEFAULT 0, -- grade which this module is meant for, bitfield
    published BOOLEAN DEFAULT false,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP,

    FOREIGN KEY (user_id)
        REFERENCES users(id)
        ON DELETE SET NULL,

    FOREIGN KEY (category_id)
        REFERENCES module_categories(id)
        ON DELETE SET NULL,

    FOREIGN KEY (image_id)
        REFERENCES images(id)
        ON DELETE SET NULL
);

-- ablaufplan stunden
CREATE TABLE module_lessons (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    module_id INT UNSIGNED NOT NULL,
    title VARCHAR(255) NOT NULL,
    position INT UNSIGNED NOT NULL, -- e.g., "1", "2", "3" or "13", "14"

    UNIQUE (module_id, position),
    INDEX idx_module_position (module_id, position),

    FOREIGN KEY (module_id) 
        REFERENCES modules(id) 
        ON DELETE CASCADE
);

-- ablaufplan stunden sections
CREATE TABLE module_lesson_sections (
    id INT UNSIGNED PRIMARY KEY AUTO_INCREMENT,
    module_lesson_id INT UNSIGNED NOT NULL,
    title VARCHAR(255) NOT NULL,
    duration INT UNSIGNED DEFAULT NULL,
    content TEXT NOT NULL, -- text content of each individual lesson in markdown
    position INT UNSIGNED NOT NULL, -- e.g., "1", "2", "3" or "13", "14"

    UNIQUE (module_lesson_id, position),
    INDEX idx_lesson_position (module_lesson_id, position),

    FOREIGN KEY (module_lesson_id) 
        REFERENCES module_lessons(id) 
        ON DELETE CASCADE
);

-- lernmaterialien
CREATE TABLE module_materials (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    file_id INT UNSIGNED NOT NULL,
    module_id INT UNSIGNED NOT NULL,
    title VARCHAR(255) NOT NULL,
    material_type VARCHAR(255) NOT NULL,
    position INT UNSIGNED NOT NULL,
    
    UNIQUE (module_id, position),

    FOREIGN KEY (file_id)
        REFERENCES files(id)
        ON DELETE CASCADE,

    FOREIGN KEY (module_id)
        REFERENCES modules(id)
        ON DELETE CASCADE
);

-- newsletters table
CREATE TABLE newsletters (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    image_id VARCHAR(500),
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    content LONGTEXT NOT NULL,
    published BOOLEAN DEFAULT false,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);

-- Workshops table
CREATE TABLE workshops (
    id INT UNSIGNED AUTO_INCREMENT PRIMARY KEY,
    image_id VARCHAR(500),
    slug VARCHAR(255) UNIQUE NOT NULL,
    title VARCHAR(255) NOT NULL,
    description TEXT NOT NULL,
    content LONGTEXT NOT NULL,
    workshop_date DATETIME NOT NULL,
    published BOOLEAN DEFAULT false,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    updated_at DATETIME DEFAULT CURRENT_TIMESTAMP ON UPDATE CURRENT_TIMESTAMP
);