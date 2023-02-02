* Run pallet benchmarks
```
  ./target/production/serv-node benchmark pallet \
    --chain dev \
    --execution=wasm \
    --wasm-execution=compiled \
    --pallet pallet_schema_registry \
    --extrinsic "*" \
    --steps 50 \
    --repeat 20 \
    --output pallets/schema-registry/src/weights.rs
```
