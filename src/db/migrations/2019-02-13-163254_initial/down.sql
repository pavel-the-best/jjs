-- indicies
DROP INDEX runs_id_unique_index;
-- tables
DROP TABLE invocations;
DROP TABLE runs;
DROP TABLE registrations;
DROP TABLE users;
-- sequences
DROP SEQUENCE user_id_seq;
DROP SEQUENCE run_id_seq;
DROP SEQUENCE inv_id_seq;
DROP SEQUENCE reg_id_seq;
-- types
DROP DOMAIN unsigned_integer;
