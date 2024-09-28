# imports

# packages

# extension module
import alea_preprocess


def test_nfc():
    assert alea_preprocess.algos.unicode.normalizations.nfc_str("cafe\u0301") == "café"


def test_nfd():
    assert alea_preprocess.algos.unicode.normalizations.nfd_str("café") == "cafe\u0301"


def test_nfkc():
    assert alea_preprocess.algos.unicode.normalizations.nfkc_str("cafe\u0301") == "café"


def test_nfkd():
    assert alea_preprocess.algos.unicode.normalizations.nfkd_str("café") == "cafe\u0301"
