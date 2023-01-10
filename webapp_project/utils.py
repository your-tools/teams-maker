from os import listdir
from os.path import isfile, join


def get_file_name_from_name_providers():
    path_to_name_providers = "../teams_maker/name_providers/"
    file_names = [
        f
        for f in listdir(path_to_name_providers)
        if isfile(join(path_to_name_providers, f))
    ]
    return file_names
