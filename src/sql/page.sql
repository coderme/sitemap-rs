-- Selects published pages with
-- $1 = maximum rows to fetch
-- $2 = start offset.

SELECT slug, modtime
FROM pages
WHERE status = 'PUBLISHED'
      AND pubtime <= now()
ORDER BY id
LIMIT $1 OFFSET $2;


