from os import listdir
from os.path import isfile, join

PATH_TO_NAME_PROVIDERS = "../teams_maker/name_providers/"


def get_file_names_from_name_providers():

    file_names = [
        f
        for f in listdir(PATH_TO_NAME_PROVIDERS)
        if isfile(join(PATH_TO_NAME_PROVIDERS, f))
    ]
    return file_names
