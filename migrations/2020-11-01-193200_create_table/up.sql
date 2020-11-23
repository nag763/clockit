-- Your SQL goes here
CREATE TABLE tasks (
	label TEXT UNIQUE NOT NULL PRIMARY KEY,
	time INTEGER NOT NULL,
    	created_on INTEGER NOT NULL,
    	begin_dt INTEGER NOT NULL,
    	end_dt INTEGER NOT NULL,	
	state TEXT CHECK(state IN ('created', 'started', 'paused', 'ended')) NOT NULL
)
