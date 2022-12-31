# Astronomer

Astronomer looks at the stars. At all the stars you’ve got on your public GitHub repositories.

Each star has its own story, so the astronomer weights them by the number of lines written in each programming language in the repository that earned that star.

From the astronomer’s point of view, you surface how much each programming language contributes to your constellation of GitHub stars.

## Boring stuff

Get a [personal access token from GitHub](https://github.com/settings/tokens) with `public_repo` and `read_user` scopes, and save it as en environment variable called `ASTRONOMER_GITHUB_TOKEN`. If you don’t have a `PORT` environment variable, astronomer will set `8000` for you.

From there `cargo run` will get you started. All the fun happens at [`localhost:8000`](http://localhost:8000) unless you use a different port. You are ready to type different GitHub usernames in the URL box and stalk them.
