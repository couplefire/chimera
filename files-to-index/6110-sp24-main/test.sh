#!/usr/bin/env bash

echo "Running cargo tests..."

cargo test

echo "Running integration tests..."

./build.sh

rm -r test_out
mkdir test_out

for file in public-tests/scanner/input/*; do
  echo "Running scanner test for $file"
  filename=$(basename "$file" .dcf)
  ./run.sh -t scan -o test_out/$filename.out $file 2>test_out/$filename.out
  exit_code=$?

  if [[ "$filename" == *"invalid"* ]]; then
    if [ $exit_code -eq 0 ]; then
      echo "Test $filename should have failed, but it passed."
      echo "Expected output:"
      cat public-tests/scanner/output/$filename.out
      echo "Actual output:"
      cat test_out/$filename.out
      exit 1
    fi
  else
    diff test_out/$filename.out public-tests/scanner/output/$filename.out
    if [ $? -ne 0 ]; then
      echo "Test $filename failed."
      exit 1
    fi
  fi
done
