# test-ip-validation

This exists simply to help us feel confident that our client-side IP address
and network validation in the web console considers the same inputs valid and
invalid as our server-side Rust code. 100% parity is ideal, though not a hard
requirement: it is probably ok if the two codebases disagree on a few edge-case
inputs that users are very unlikely to actually want. It's better for the
client-side validation to be more permissive than to be too restrictive, i.e.,
better to allow the occasional bad input to hit the server and get rejected
there than to disallow the occasional good input and force the user to use the
CLI.
