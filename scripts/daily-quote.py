#! /usr/bin/env python

from datetime import datetime
from dotenv import find_dotenv, load_dotenv
from ISStreamer.Streamer import Streamer
from sh import echo, groupme, quote_storage
from typing import Tuple

import os

"""This script is written in Python 3.6"""

assert load_dotenv(find_dotenv()), "Could not find .env config"

BUCKET_NAME = os.environ.get("IS_BUCKET_NAME")
BUCKET_KEY = os.environ.get("IS_BUCKET_KEY")
ACCESS_KEY = os.environ.get("IS_ACCESS_KEY")

def get_quote_by_day_of_week(is_thursday: bool) -> str:
    if is_thursday:
        return quote_storage.retrieve("--random-retrieved")

    return quote_storage.retrieve("--first-unretrieved", "--mark-retrieved")

def get_quote_counts() -> Tuple[int, int]:
    unretrieved = int(quote_storage.count("unretrieved-quotes"))
    retrieved = int(quote_storage.count("retrieved-quotes"))

    return retrieved, unretrieved

def get_author_count() -> int:
    return int(quote_storage.count.authors())

if __name__ == "__main__":
    is_thursday = datetime.today().weekday() == 3
    quote = get_quote_by_day_of_week(is_thursday)
    retrieved_count, unretrieved_count = get_quote_counts()
    author_count = get_author_count()

    print(f"Today's quote is:\n{quote}")

    quote = str(quote)

    if is_thursday:
        quote = "Throwback Thursday:\n\n" + quote

    streamer = Streamer(access_key=ACCESS_KEY,
                        bucket_name=BUCKET_NAME,
                        bucket_key=BUCKET_KEY)

    streamer.log("Unretrieved Quotes", unretrieved_count)
    streamer.log("Retrieved Quotes", retrieved_count)
    streamer.log("Total Quotes", unretrieved_count + retrieved_count)
    streamer.log("Total Authors", author_count)

    groupme.bot.send(echo(quote))

    streamer.log("Today's quote sent", True)  # TODO: See if send was actually successful
    streamer.close()
