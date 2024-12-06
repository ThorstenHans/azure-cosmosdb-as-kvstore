# Use Azure CosmosDB as Key-Value Store in Spin

## Prerequisites

- Spin 3.0.0
- An Azure CosmosDB Account
  - A collection with partition key set to `/id`

## Update the Runtime Configuration File (`azure.toml`)

Update the `azure.toml` file and provide necessary information (see additional comments in `azure.toml`)

## Building and Running the app locally

```bash
spin up --runtime-config-file ./azure.toml --build
Compiling kv-demo v0.1.0 (/Users/thorsten/dev/demos/kv-demo)
...
 Finished `release` profile [optimized] target(s) in 0.76s
Finished building all Spin components
Logging component stdio to ".spin/logs/"
Using [key_value_store.azure: cosmos] runtime config from "./azure.toml"
Serving http://127.0.0.1:3000
Available Routes:
  kv-demo: http://127.0.0.1:3000 (wildcard)
```

## Using the App

Grab your g'old `curl` and send some requests:

```bash
# Insert a single value at key foo
curl -iX POST localhost:3000/azure/foo -H 'Content-Type: application/json' -d '{"value": "bar"}'

# Get a value using its key (foo here)
curl -iX GET localhost:3000/azure/foo

# Get all keys from your Azure CosmosDB Container
curl -iX GET localhost:3000/azure/keys/all

# Get count of keys in Azure CosmosDB
curl -iX GET localhost:3000/azure/keys/count

# Bulk Upsert (keys `bar` and `hey`)
curl -iX POST localhost:3000/azure/bulk -H 'Content-Type: application/json' -d '{"values": ["bar":"baz", "hey": "ho"]}'
```