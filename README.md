# foxford-ical-worker

This is a Rust project designed to work as a Cloudflare Worker and convert a Foxford JSON calendar to the iCal format.

## Usage
### Deployed version

Go to [Foxford family](https://foxford.ru/dashboard/family) click on your child schedule. Copy the page link. It will look like:
```
https://foxford.ru/dashboard/shared_calendar/<child-id>/<token>
```

Create new link to the worker:
```
https://foxford-ical-worker.patrushev.workers.dev/<token>/<child-id>
```

Use this link in your calendar apps to subscribe to external calendar.

### Deploy your own instance

To deploy this project, you will need to have [Rust installed](https://www.rust-lang.org/tools/install) on your computer. Also you need a [Cloudflare account](https://dash.cloudflare.com/login).

You can clone this repository using the following commands:
```
git clone https://github.com/apatrushev/foxford-ical-worker
cd foxford-ical-worker
```

The project uses [spherical-dev](https://pypi.org/project/spherical-dev/) python infrastructure for development tasks. It helps to keep environment isolation from other projects. To build and deploy a project please do the following steps:
```
python3 -m vevn venv
. venv/bin/activate
pip install -r requirements.txt
inv publish
```

This will deploy worker instance to your cloudflare account. It will be available for the requests on address `foxford-ical-worker.<yourname>.workers.dev` and you can proceed with [click on this link](#deployed-version) instruction.

Under the hood `inv publish` command creates isolated node js deployment, installs cloudflare wrangler toolchain and use `wrangler publish` to deploy the project.

You can also use `inv dev` for development purpose to run local instance of worker and play with the code.

## License
This project is licensed under the MIT License. See [LICENSE](LICENSE) for more details.
