default:
  @just --list
test:
  test -n "" || true
  @cargo test
# tutorial comment
test_recipe:
  echo 'This is another recipe.'
