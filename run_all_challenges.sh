cargo metadata --no-deps --format-version 1 \
  | jq -r '.packages[].targets[] | select(.kind[] | contains("bin")) | .name' \
  | while read bin; do
      echo "Running $bin..."
      cargo run --bin "$bin" || { echo "$name failed"; exit 1; }
  done
