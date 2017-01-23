#! /usr/bin/env python

from datetime import datetime

from sh import echo, groupme, quote_storage

"""This script is written in Python 3.6"""

def get_quote_by_day_of_week(is_thursday: bool) -> str:
    if is_thursday:
        return quote_storage.retrieve("--random-retrieved")

    return quote_storage.retrieve("--first-unretrieved", "--mark-retrieved")

if __name__ == "__main__":
    is_thursday = datetime.today().weekday() == 4
    quote = get_quote_by_day_of_week(is_thursday)

    print(f"Today's quote is:\n{quote}")

    quote = str(quote)

    if is_thursday:
        quote = "Throwback Thursday:\n\n" + quote

    groupme.bot.send(echo(quote))
