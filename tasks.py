import invoke
from spherical.dev.node import node_install, npm_install
from spherical.dev.tasks import isort  # noqa: F401


@invoke.task(node_install)
def wrangler_install(ctx):
    npm_install(ctx, ['wrangler@2.0.0'])


@invoke.task(wrangler_install)
def dev(ctx):
    ctx.run('wrangler dev --local')


@invoke.task(wrangler_install)
def publish(ctx):
    ctx.run('wrangler publish')
