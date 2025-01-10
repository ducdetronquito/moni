CREATE OR REPLACE FUNCTION bump_updated_column()   
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated = STATEMENT_TIMESTAMP();
    RETURN NEW;
END;
$$ language 'plpgsql';
