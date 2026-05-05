import logging

import aiohttp
from interactions import Extension, SlashContext, slash_command

logger = logging.getLogger(__name__)


class IPCommands(Extension):
    @slash_command(name="get-ip", description="Check the server's public IP address")
    async def get_ip(self, ctx: SlashContext) -> None:
        logger.info("get-ip invoked by %s (guild=%s)", ctx.author, ctx.guild_id)
        async with (
            aiohttp.ClientSession() as session,
            session.get("https://checkip.amazonaws.com/") as response,
        ):
            ip = (await response.text()).strip()
        logger.info("Resolved public IP: %s", ip)
        await ctx.send(f"Current public IP: `{ip}`")
