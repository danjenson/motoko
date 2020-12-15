CREATE OR REPLACE FUNCTION truncate_tables() RETURNS void AS $$
DECLARE
    statements CURSOR FOR
    SELECT table_name
    FROM information_schema.tables
    WHERE table_type = 'BASE TABLE'
    AND table_schema = 'public';
BEGIN
    FOR stmt IN statements LOOP
        EXECUTE 'TRUNCATE TABLE ' || quote_ident(stmt.table_name) || ' CASCADE';
    END LOOP;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION drop_tables() RETURNS void AS $$
DECLARE
    statements CURSOR FOR
    SELECT table_name
    FROM information_schema.tables
    WHERE table_type = 'BASE TABLE'
    AND table_schema = 'public';
BEGIN
    FOR stmt IN statements LOOP
        EXECUTE 'DROP TABLE ' || quote_ident(stmt.table_name) || ' CASCADE';
    END LOOP;
END;
$$ LANGUAGE plpgsql;

CREATE OR REPLACE FUNCTION drop_views() RETURNS void AS $$
DECLARE
    statements CURSOR FOR
    SELECT table_name
    FROM information_schema.tables
    WHERE table_type = 'VIEW'
    AND table_schema = 'public';
BEGIN
    FOR stmt IN statements LOOP
        EXECUTE 'DROP VIEW ' || quote_ident(stmt.table_name) || ' CASCADE';
    END LOOP;
END;
$$ LANGUAGE plpgsql;
