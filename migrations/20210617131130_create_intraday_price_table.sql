-- Add migration script here
----------------------------------------
-- Hypertable to store financial intraday data
----------------------------------------
-- Step 1: Define regular table
CREATE TABLE IF NOT EXISTS intraday_prices (

   time TIMESTAMP WITHOUT TIME ZONE NOT NULL,
   ticker text NULL,
   high real NULL,
   low real NULL,
   "open" real NULL,
   "close" real NULL,
   average real NULL,
   volume real NULL,
   notional real NULL,
   number_of_trades int NULL,
   change_over_time real NULL
);

-- Step 2: Turn into hypertable
SELECT create_hypertable('intraday_prices','time');