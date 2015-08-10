# Einthusan Downloader

[Einthusan][1] is a premium south asian video publisher providing
movies and music videos on-demand.

This script [einthusan-dl] comes handy if you want to download movies from Einthusan site.


## Instructions

`einthusan-dl` requires Python 2 (2.6 or newer) or Python 3 (3.2 or newer). On Windows
ensure that Python executable location is added to the PATH environment variable.

### Install any missing dependencies.

Make sure you have installed and/or updated all of your dependencies 
according to the `requirements.txt` file.

You can use [`pip`][2] to install the dependencies. Follow the methods described in the link to install pip.

Once you have pip installed you can run the below command to install the dependencies,

```
pip install -r requirements.txt
```

### Running the script

Run the script to download the videos by providing the movie url(s),
as well as any additional parameters:

    General:                        einthusan-dl "<movie_url1>"
    Multiple movies:                einthusan-dl "<movie_url1>" "<movie_url2>" "<movie_url3>"
    Specify external downloader:    einthusan-dl "<movie_url1>" --wget
    Specify download path:          einthusan-dl --path=C:\downloads\movies "<movie_url1>"
    Specify log path:          		einthusan-dl --log=C:\downloads\logs\output.log "<movie_url1>"
    Display help:                   einthusan-dl --help

**Note:** If you have noticed the movie url's are enclosed with double quotes("). We need this to avoid the problems with
uri escaped characters (like &, +, @, $ and so on).

### How to get the movie url ?

On clicking any movie in Einthusan site, now you must be able to copy the url from the address bar.
Make sure the url matches with below format,

http://www.einthusan.com/movies/watch.php?tamilmoviesonline={MOVIE-NAME}&lang=tamil&id={MOVIE-ID}

Note that the above sample url is based on a tamil movie. Similarly for other languages (hindi/malayalam/telugu)
the word 'tamil' should get replaced with their respective languages.

## Disclaimer:

As per the [`Terms of Service`][4] from Einthusan.com,

> "You may not either directly or through the use of any device,
software, internet site, web-based service, or other means copy, download, stream capture, reproduce, duplicate,
archive, distribute, upload, publish, modify, translate, broadcast, perform, display, sell, transmit or retransmit
the Content unless expressly permitted by Einthusan.com in writing."

So be aware of this and use the script on your own risk :)

[1]: https://www.einthusan.com
[2]: https://pip.pypa.io/en/latest/installing.html#install-pip
[3]: http://python-distribute.org/pip_distribute.png
[4]: http://www.einthusan.com/terms/
