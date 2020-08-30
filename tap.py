import sys
import os
from pathlib import Path
import shutil

filetypes = {
    "i": {"name": "__init__.py"},
    "r": {"name": "README.md"},
    "l": {"name": "LICENSE"},
    "t": {"name": "test"}
}

config_path = "/home/jake/.local/share/tap/"


class Setup:
    def __init__(self):
        self.make_dir()

    def make_dir(self):
        if not os.path.isdir(config_path):
            Path(config_path).mkdir(parents=True)
            self.copy_defualts()

    def copy_defualts(self):
        path = os.getcwd()
        full_path = os.path.join(path, "files")
        src_files = os.listdir(full_path)
        for file_name in src_files:
            full_file_name = os.path.join(full_path, file_name)
            if os.path.isfile(full_file_name):
                shutil.copy(full_file_name, config_path)


class Tap:
    def __init__(self):
        self.copy_file()

    def parse_args(self):
        if len(arg := sys.argv) == 2:
            for key, value in filetypes.items():
                if key == arg[1]:
                    return value

    def get_name(self):
        name = self.parse_args()
        if name is None:
            sys.exit()
        filename = name['name']
        return filename

    def copy_file(self):
        dst = os.getcwd()
        name = self.get_name()
        path_from = os.path.join(config_path, name)
        path_to = os.path.join(dst, name)
        if os.path.isfile(path_to):
            replace = input("Replace file {path_to}: [Y/n] ")
            if replace.upper() != "Y":
                exit()
        print(f"Copied {path_from} to {path_to}")
        shutil.copyfile(path_from, path_to)


if __name__ == "__main__":
    setup = Setup()
    tap = Tap()
    tap.get_name()
