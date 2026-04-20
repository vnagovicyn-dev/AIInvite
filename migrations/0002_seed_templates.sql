INSERT INTO templates (id, code, name, category, preview_url, is_active)
VALUES
    ('11111111-1111-1111-1111-111111111111', 'wedding-classic', 'Classic Wedding', 'wedding', 'https://cdn.example.com/templates/wedding-classic.jpg', true),
    ('11111111-1111-1111-1111-111111111112', 'wedding-modern', 'Modern Wedding', 'wedding', 'https://cdn.example.com/templates/wedding-modern.jpg', true),
    ('11111111-1111-1111-1111-111111111113', 'birthday-confetti', 'Birthday Confetti', 'birthday', 'https://cdn.example.com/templates/birthday-confetti.jpg', true),
    ('11111111-1111-1111-1111-111111111114', 'birthday-neon', 'Birthday Neon', 'birthday', 'https://cdn.example.com/templates/birthday-neon.jpg', false),
    ('11111111-1111-1111-1111-111111111115', 'corporate-clean', 'Corporate Clean', 'corporate', 'https://cdn.example.com/templates/corporate-clean.jpg', true),
    ('11111111-1111-1111-1111-111111111116', 'corporate-bold', 'Corporate Bold', 'corporate', 'https://cdn.example.com/templates/corporate-bold.jpg', true),
    ('11111111-1111-1111-1111-111111111117', 'baby-soft', 'Baby Soft', 'baby', 'https://cdn.example.com/templates/baby-soft.jpg', true),
    ('11111111-1111-1111-1111-111111111118', 'baby-stars', 'Baby Stars', 'baby', 'https://cdn.example.com/templates/baby-stars.jpg', false),
    ('11111111-1111-1111-1111-111111111119', 'anniversary-gold', 'Anniversary Gold', 'anniversary', 'https://cdn.example.com/templates/anniversary-gold.jpg', true),
    ('11111111-1111-1111-1111-111111111120', 'anniversary-minimal', 'Anniversary Minimal', 'anniversary', 'https://cdn.example.com/templates/anniversary-minimal.jpg', true)
ON CONFLICT (code) DO UPDATE
SET
    name = EXCLUDED.name,
    category = EXCLUDED.category,
    preview_url = EXCLUDED.preview_url,
    is_active = EXCLUDED.is_active;
