# leptonica-plumbing
Crate to expose a safe version of the leptonica-sys api



cargo test --no-run && valgrind --leak-check=yes --error-exitcode=1 target/debug/deps/leptonica_plumbing-99cde7e43309039b; echo $?