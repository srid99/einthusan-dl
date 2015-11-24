#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
For downloading movies from Einthusan.com website.

Examples:
  einthusan-dl <url>
  einthusan-dl <url1> <url2> --path /tmp/downloads --wget --log /tmp/output.log
"""

import argparse
import errno
import logging
import os
from datetime import datetime

import requests
from ago import human

get_movie_api_url = "http://cdn.einthusan.com/geturl/{0}/hd/London%2CToronto%2CDallas%2CWashington%2CSan%2CSydney/"

try:
    from BeautifulSoup import BeautifulSoup
except ImportError:
    from bs4 import BeautifulSoup as BeautifulSoup_
    BeautifulSoup = lambda page: BeautifulSoup_(page, 'html.parser')

from .downloaders import get_downloader


def get_page(session, url):
    """
    Download an HTML page using the requests session.
    """

    r = session.get(url)

    try:
        r.raise_for_status()
    except requests.exceptions.HTTPError as e:
        logging.error("Error %s getting page %s", e, url)
        raise

    return r.text


def get_movie_page(session, movie_page_url):
    """
    Get the movie webpage.
    """

    page = get_page(session, movie_page_url)
    logging.info('Downloaded %s (%d bytes)', movie_page_url, len(page))

    return page


def get_movie_url(session, page):
    """
    Parses a Einthusan movie page and retrieves the movie url.
    """

    soup = BeautifulSoup(page)

    movie_id = soup.find('div', {'id': 'mediaplayer'})['data-movieid']
    movie_url_api = get_movie_api_url.format(movie_id)
    movie_url = get_page(session, movie_url_api)

    logging.info('Movie url: %s', movie_url)

    return movie_url


def get_movie_name(page):
    soup = BeautifulSoup(page)
    name = soup.findAll("a", {"class": 'movie-title'})[0].get_text()
    return name


def start_download_movie(downloader,
                         movie_name,
                         movie_url,
                         overwrite=False,
                         skip_download=False,
                         path=''
                         ):
    """
    Downloads the movie from the given url.
    """

    logging.debug('Start downloading movie %s from url %s', movie_name, movie_url)

    dest = os.path.join(path, movie_name)

    if not os.path.exists(dest):
        logging.debug('Destination path %s not exists. Let\'s create one.', dest)
        mkdir_p(dest)

    filename = os.path.join(dest, movie_name + '.mp4')

    if overwrite or not os.path.exists(filename):
        if os.path.exists(filename):
            logging.info('File already exists and will be overwritten since option "--overwrite" is enabled.')

        if not skip_download:
            logging.info('Downloading: %s', movie_name)
            start = datetime.now()
            downloader.download(movie_url, filename)
            logging.info('Successfully downloaded the movie %s. Took %s', movie_name, human(start, past_tense='{0}'))
        else:
            logging.info('Skipping downloading the movie %s since the option "--skip-download" is enabled.', movie_name)
    else:
        logging.info('%s already downloaded', filename)

    return


def mkdir_p(path, mode=0o777):
    """
    Create subdirectory hierarchy given in the paths argument.
    """

    try:
        os.makedirs(path, mode)
    except OSError as exc:
        if exc.errno == errno.EEXIST and os.path.isdir(path):
            pass
        else:
            raise


def parse_args():
    """
    Parse the arguments/options passed to the program on the command line.
    """

    parser = argparse.ArgumentParser(description='Download movie from Einthusan.com.')

    # positional
    parser.add_argument('url', action='store', nargs='+', help='url(s) of the movie to be downloaded')

    # optional
    parser.add_argument('-o', '--overwrite', dest='overwrite', action='store_true', default=False,
                        help='whether existing files should be overwritten (default: False)')
    parser.add_argument('-l', '--log', dest='logfile', action='store', default=None,
                        help='logs to the specified logfile (default: logs to console)')
    parser.add_argument('--wget', dest='wget', action='store', nargs='?', const='wget', default=None,
                        help='use wget for downloading, optionally specify wget bin')
    parser.add_argument('--curl', dest='curl', action='store', nargs='?', const='curl', default=None,
                        help='use curl for downloading, optionally specify curl bin')
    parser.add_argument('--skip-download', dest='skip_download', action='store_true', default=False,
                        help='for debugging: skip actual downloading of files')
    parser.add_argument('--path', dest='path', action='store', default='', help='path to save the file')
    parser.add_argument('--debug', dest='debug', action='store_true', default=False,
                        help='print lots of debug information')

    args = parser.parse_args()

    # Initialize the logging system first so that other functions
    # can use it right away
    if args.debug:
        logging.basicConfig(filename=args.logfile, level=logging.DEBUG,
                            format='%(asctime)s %(name)s[%(funcName)s] %(message)s')
    else:
        logging.basicConfig(filename=args.logfile, level=logging.INFO, format='%(asctime)s %(message)s')

    return args


def download_movie(args, movie_page_url):
    """
    Download the movie from the given movie_page_url.
    Returns True if the movie appears completed.
    """

    session = requests.Session()

    # get the web page of the movie
    page = get_movie_page(session, movie_page_url)

    # parse the page and get the url
    movie_url = get_movie_url(session, page)

    movie_name = get_movie_name(page)

    downloader = get_downloader(session, args)

    # obtain the resources
    start_download_movie(
        downloader,
        movie_name,
        movie_url,
        args.overwrite,
        args.skip_download,
        args.path)

    return


def main():
    """
    Main entry point for execution as a program (instead of as a module).
    """

    args = parse_args()

    for movie_page_url in args.url:
        try:
            logging.info('Downloading movie from page : %s', movie_page_url)
            download_movie(args, movie_page_url)
        except requests.exceptions.HTTPError as e:
            logging.error('HTTPError %s', e)
        except KeyboardInterrupt:
            logging.error('Downloading interrupted!')
        except BaseException as e:
            logging.error('Unhandled exception: %s', e)


if __name__ == '__main__':
    main()
