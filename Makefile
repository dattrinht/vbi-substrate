test_pallet_kitties:
	cargo test -p pallet-kitties

benchmark_pallet_kitties:
	./target/release/node-kitties benchmark \
    --chain dev \
    --execution wasm \
    --wasm-execution compiled \
    --pallet pallet_kitties \
    --extrinsic '*' \
    --steps 20 \
    --repeat 10 \
    --output ./pallets/kitties/src/weights.rs