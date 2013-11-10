# Einthusan Downloader

[Einthusan][1] is a premium south asian video publisher providing
movies and music videos on-demand.

This script helps us to download movies from Einthusan site.


## Instructions

`einthusan-dl` requires Python 2 (2.6 or newer) or Python 3 (3.2 or newer). On Windows
ensure that Python executable location is added to the PATH environment variable.

### Install any missing dependencies.

Make sure you have installed and/or updated all of your dependencies 
according to the `requirements.txt` file.

You can use [`pip`][2] to install the dependencies. To do so you have to install
[`pip`][2], as in it is the current [preferred method][3].  

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
    Display help:                   einthusan-dl --help

### How to get the movie url ?

On clicking any movie in Einthusan site, now you must be able to copy the url from the address bar.
Make sure the url matches with below format,

http://www.einthusan.com/movies/watch.php?tamilmoviesonline={MOVIE-NAME}&lang=tamil&id={MOVIE-ID}

Note that the above sample url is based on a tamil movie. Similarly for other languages (hindi/malayalam/telugu) the word 'tamil' should get replaced.


[1]: https://www.einthusan.com
[2]: http://www.pip-installer.org/en/latest/
[3]: http://python-distribute.org/pip_distribute.png
