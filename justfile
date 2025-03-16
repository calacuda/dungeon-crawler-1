_:
  @just --list

# test-client:
#   cargo run --bin client
#
# test-server:
#   cargo run --bin server

run-game:
  cargo run --bin game

_new-window NAME CMD:
  tmux new-w -t dc-1 -n "{{NAME}}"
  tmux send-keys -t dc-1:"{{NAME}}" "{{CMD}}" ENTER

tmux:
  tmux new -ds dc-1 -n "README"
  tmux send-keys -t dc-1:README 'nv ./README.md "+set wrap"' ENTER
  @just _new-window "Edit" ""
  @just _new-window "Run" ""
  @just _new-window "Git" "git status"
  @just _new-window "Misc" ""
  tmux a -t dc-1
