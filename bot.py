import logging
import os

import colorlog
from interactions import Client, Intents, listen
from interactions.api.events import Ready

logger = logging.getLogger(__name__)


def _setup_logging() -> None:
    level = logging.getLevelName(os.environ.get("LOG_LEVEL", "INFO").upper())
    handler = colorlog.StreamHandler()
    handler.setFormatter(colorlog.ColoredFormatter(
        fmt="%(asctime)s | %(log_color)s%(levelname)-8s%(reset)s | %(name)s - %(message)s",
        datefmt="%Y-%m-%d %H:%M:%S",
        log_colors={
            "DEBUG":    "cyan",
            "INFO":     "green",
            "WARNING":  "yellow",
            "ERROR":    "red",
            "CRITICAL": "bold_red",
        },
    ))
    logging.basicConfig(level=level, handlers=[handler])
    logging.getLogger("interactions").setLevel(logging.WARNING)


_setup_logging()

bot = Client(token=os.environ["DISCORD_TOKEN"], intents=Intents.DEFAULT)
bot.load_extension("extensions.ip_commands")


@listen(Ready)
async def on_ready(_: Ready) -> None:
    logger.info("Bot online: %s", bot.user.tag)


bot.start()
