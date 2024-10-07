from pathlib import Path

import alea_preprocess

FILE_PATH = Path(__file__)
PROJECT_PATH = FILE_PATH.parent.parent.parent
RESOURCE_PATH = PROJECT_PATH / "resources/"


def test_directories():
    print(RESOURCE_PATH)
    results = alea_preprocess.io.fs.directories.get_directories(str(PROJECT_PATH))
    assert len(results) > 5
    assert str(RESOURCE_PATH.absolute()) in results
    assert str(FILE_PATH.absolute()) not in results


def test_files():
    results = alea_preprocess.io.fs.directories.get_files(str(PROJECT_PATH))
    assert len(results) > 5
    assert str(RESOURCE_PATH.absolute()) not in results
    assert str(FILE_PATH.absolute()) in results


def test_entries():
    results = alea_preprocess.io.fs.directories.get_entries(str(PROJECT_PATH))
    assert len(results) > 5
    assert str(RESOURCE_PATH.absolute()) in results
    assert str(FILE_PATH.absolute()) in results


def test_get_all_file_info():
    test_path = RESOURCE_PATH
    file_info_list = alea_preprocess.io.fs.directories.get_all_file_info(str(test_path))
    assert len(file_info_list) > 5
