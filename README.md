# Astronomer

Astronomer looks at the stars. At all the stars you’ve got on your public GitHub repositories.

Each star has its own story, so the astronomer weights them by the number of lines written in each programming language in the repository that earned that star.

From the astronomer’s point of view, you understand how much each programming language contributes to your constellation of GitHub stars.

## Usage

### Filtering and limiting programming languages

Astronomer accepts URL query parameters:

* `exclude` as a comma-separated list of programming languages to ignore (they should be written as they show up in the chart, and it is case-sensitive)
* `top` to limit the number of programming languages to show.

For example, to get @cuducos's top 5 programming languages, excluding HTML and Mako: [`https://astronomer.onrender.com/cuducos?exclude=HTML, Mako&top=5`](https://astronomer.onrender.com/cuducos?exclude=HTML,%20Mako&top=5).

### API

Wanna use the data without Astronomer's visualization? No problem, just append `.json` to the username in the URL. For example:

* [`https://astronomer.onrender.com/cuducos.json`](https://astronomer.onrender.com/cuducos.json)

* [`https://astronomer.onrender.com/cuducos.json?exclude=HTML, Mako&top=5`](https://astronomer.onrender.com/cuducos.json?exclude=HTML,%20Mako&top=5)


## Boring stuff

Get a [personal access token from GitHub](https://github.com/settings/tokens) with `public_repo` and `read_user` scopes, and save it as an environment variable called `ASTRONOMER_GITHUB_TOKEN`.

If you don’t have a `PORT` environment variable, astronomer will use `8000` for the HTTP server. From there, `cargo run` will get you started. All the fun happens at [`localhost:8000`](http://localhost:8000) (unless you use a different port). You are ready to type different GitHub usernames in the URL address bar and stalk them.
