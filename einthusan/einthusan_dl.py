#!/usr/bin/env python
# -*- coding: utf-8 -*-

"""
For downloading movies from Einthusan.com / Einthusan.tv website.

Examples:
  einthusan-dl <url>
  einthusan-dl <url1> <url2> --path /tmp/downloads --wget --log /tmp/output.log
"""

import argparse
import base64
import errno
import json
import logging
import os
import time
import re
import requests
from requests.adapters import HTTPAdapter

requests.utils.default_user_agent = lambda: 'Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/56.0.2924.87 Safari/537.36'


def beautiful_soup(page):
    from bs4 import BeautifulSoup as BeautifulSoup_
    return BeautifulSoup_(page, 'html.parser')


def get_page(session, url):
    """
    Download an HTML page using the requests session.
    """

    r = session.get(url)

    try:
        r.raise_for_status()
    except requests.exceptions.HTTPError as e:
        logging.error('Error %s getting page %s', e, url)
        raise

    return r.text


def get_movie_page(session, movie_page_url):
    """
    Get the movie webpage.
    """

    page = get_page(session, movie_page_url)
    logging.info('Downloaded %s (%d bytes)', movie_page_url, len(page))

    return beautiful_soup(page)


def decode(value):
    value_len = len(value)
    encoded_value = value[0:10] + value[value_len - 1] + value[12:value_len - 1]
    decoded_value = base64.b64decode(encoded_value).decode('utf-8')
    return json.loads(decoded_value)


def get_movie_url(session, page, movie_page_url):
    """
    Parses a Einthusan movie page and retrieves the movie url.
    """

    page_id = page.find('html')['data-pageid']                                  
    ejpingables = page.find('section', {'id': 'UIVideoPlayer'})['data-ejpingables']
    line = page.find('section', {'id': 'UIVideoPlayer'})['data-mp4-link']       
    ip_addr_regex = re.compile(r'\b(?:[0-9]{1,3}\.){3}[0-9]{1,3}\b')            
    line = re.sub(ip_addr_regex, 'cdn2.einthusan.io', line)                     
                                                                                
    return line          


def get_movie_name(page):
    summary_section = page.find('section', {'id': 'UIMovieSummary'})
    return summary_section.findAll('a', {'class': 'title'})[0].findAll('h3')[0].get_text()


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

    logging.debug('Start downloading movie [%s] from url %s', movie_name, movie_url)

    slugified_filename = "_" + ''.join([c if c.isalnum() else '_' for c in movie_name])
    dest = os.path.join(path, slugified_filename)

    if not os.path.exists(dest):
        logging.debug('Destination path %s not exists. Let\'s create one.', dest)
        mkdir_p(dest)

    filename = os.path.join(dest, slugified_filename + '.mp4')

    if overwrite or not os.path.exists(filename):
        if os.path.exists(filename):
            logging.info('File already exists and will be overwritten since option "--overwrite" is enabled.')

        if not skip_download:
            logging.info('Downloading: %s', movie_name)
            start = time.time()
            downloader.download(movie_url, filename)
            time_taken = time.time() - start
            logging.info('Successfully downloaded the movie [%s]. Took [%s] seconds!', movie_name, int(time_taken))
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

    parser = argparse.ArgumentParser(description='Download movie from Einthusan.')

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
    session.mount('https://', HTTPAdapter(max_retries=3))

    # get the web page of the movie
    page = get_movie_page(session, movie_page_url)
    # print(page)
    # parse the page and get the url
    movie_url = get_movie_url(session, page, movie_page_url)

    movie_name = get_movie_name(page)

    from .downloaders import get_downloader
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
